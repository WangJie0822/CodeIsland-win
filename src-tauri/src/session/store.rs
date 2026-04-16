use std::collections::HashMap;
use log::{info, warn};
use crate::hook::protocol::HookEvent;
use crate::session::phase::SessionPhase;
use crate::session::state::{SessionState, ConversationInfo};

pub struct SessionStore {
    sessions: HashMap<String, SessionState>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SessionSummary {
    pub session_id: String,
    pub project_name: String,
    pub cwd: String,
    pub phase: String,
    pub needs_attention: bool,
    pub last_message: Option<String>,
    pub tool_name: Option<String>,
    pub tool_input: Option<serde_json::Value>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self { sessions: HashMap::new() }
    }

    pub fn process_hook_event(&mut self, event: &HookEvent) -> Option<String> {
        let session_id = &event.session_id;

        if !self.sessions.contains_key(session_id) {
            let session = SessionState::new(session_id.clone(), event.cwd.clone());
            self.sessions.insert(session_id.clone(), session);
            info!("[Store] 新建会话: {}", &session_id[..8.min(session_id.len())]);
        }

        let session = self.sessions.get_mut(session_id).unwrap();

        if let Some(pid) = event.pid {
            session.pid = Some(pid);
        }
        if event.tty.is_some() {
            session.tty = event.tty.clone();
        }
        if !event.cwd.is_empty() && session.cwd.is_empty() {
            session.cwd = event.cwd.clone();
            session.project_name = event.cwd
                .rsplit(['/', '\\'])
                .find(|s| !s.is_empty())
                .unwrap_or("unknown")
                .to_string();
        }

        let target_phase = SessionPhase::from_hook_event(event);

        if session.phase.can_transition(&target_phase) {
            info!(
                "[Store] 会话 {} 状态转换: {} → {}",
                &session_id[..8.min(session_id.len())],
                session.phase.name(),
                target_phase.name()
            );
            session.phase = target_phase;
        } else {
            warn!(
                "[Store] 会话 {} 无效转换: {} → {}, 忽略",
                &session_id[..8.min(session_id.len())],
                session.phase.name(),
                target_phase.name()
            );
        }

        session.touch();
        Some(session_id.clone())
    }

    pub fn process_approval(&mut self, session_id: &str) -> bool {
        if let Some(session) = self.sessions.get_mut(session_id) {
            if session.phase.can_transition(&SessionPhase::Processing) {
                session.phase = SessionPhase::Processing;
                session.touch();
                return true;
            }
        }
        false
    }

    pub fn process_denial(&mut self, session_id: &str) -> bool {
        if let Some(session) = self.sessions.get_mut(session_id) {
            if session.phase.can_transition(&SessionPhase::Idle) {
                session.phase = SessionPhase::Idle;
                session.touch();
                return true;
            }
        }
        false
    }

    pub fn update_conversation_info(&mut self, session_id: &str, info: ConversationInfo) {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.conversation_info = info;
            session.touch();
        }
    }

    pub fn remove_ended_sessions(&mut self) {
        self.sessions.retain(|_, s| s.phase != SessionPhase::Ended);
    }

    pub fn get_summaries(&self) -> Vec<SessionSummary> {
        let mut summaries: Vec<_> = self.sessions.values()
            .filter(|s| s.phase != SessionPhase::Ended)
            .map(|s| {
                let (tool_name, tool_input) = s.active_permission()
                    .map(|p| (Some(p.tool_name.clone()), p.tool_input.clone()))
                    .unwrap_or((None, None));
                SessionSummary {
                    session_id: s.session_id.clone(),
                    project_name: s.project_name.clone(),
                    cwd: s.cwd.clone(),
                    phase: s.phase.name().to_string(),
                    needs_attention: s.phase.needs_attention(),
                    last_message: s.conversation_info.last_message.clone(),
                    tool_name,
                    tool_input,
                }
            })
            .collect();
        summaries.sort_by(|a, b| b.needs_attention.cmp(&a.needs_attention));
        summaries
    }

    pub fn get_session(&self, session_id: &str) -> Option<&SessionState> {
        self.sessions.get(session_id)
    }

    pub fn active_count(&self) -> usize {
        self.sessions.values().filter(|s| s.phase != SessionPhase::Ended).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_event(session_id: &str, event: &str, status: &str) -> HookEvent {
        HookEvent {
            session_id: session_id.into(), cwd: "/test/project".into(),
            event: event.into(), status: status.into(),
            pid: Some(1234), tty: None, tool: None, tool_input: None,
            tool_use_id: None, notification_type: None, message: None,
        }
    }

    fn make_permission_event(session_id: &str, tool: &str) -> HookEvent {
        HookEvent {
            session_id: session_id.into(), cwd: "/test/project".into(),
            event: "PermissionRequest".into(), status: "waiting_for_approval".into(),
            pid: Some(1234), tty: None,
            tool: Some(tool.into()),
            tool_input: Some(serde_json::json!({"command": "test"})),
            tool_use_id: Some("tu-1".into()),
            notification_type: None, message: None,
        }
    }

    #[test]
    fn test_new_session_created_on_first_event() {
        let mut store = SessionStore::new();
        store.process_hook_event(&make_event("s1", "UserPromptSubmit", "processing"));
        assert_eq!(store.active_count(), 1);
        let session = store.get_session("s1").unwrap();
        assert_eq!(session.phase.name(), "processing");
        assert_eq!(session.project_name, "project");
    }

    #[test]
    fn test_processing_to_waiting_for_approval() {
        let mut store = SessionStore::new();
        store.process_hook_event(&make_event("s1", "UserPromptSubmit", "processing"));
        store.process_hook_event(&make_permission_event("s1", "Bash"));
        let session = store.get_session("s1").unwrap();
        assert_eq!(session.phase.name(), "waitingForApproval");
        assert_eq!(session.pending_tool_name(), Some("Bash"));
    }

    #[test]
    fn test_approve_permission() {
        let mut store = SessionStore::new();
        store.process_hook_event(&make_permission_event("s1", "Bash"));
        assert!(store.process_approval("s1"));
        assert_eq!(store.get_session("s1").unwrap().phase.name(), "processing");
    }

    #[test]
    fn test_deny_permission() {
        let mut store = SessionStore::new();
        store.process_hook_event(&make_permission_event("s1", "Bash"));
        assert!(store.process_denial("s1"));
        assert_eq!(store.get_session("s1").unwrap().phase.name(), "idle");
    }

    #[test]
    fn test_session_ended_removed() {
        let mut store = SessionStore::new();
        store.process_hook_event(&make_event("s1", "SessionStart", "waiting_for_input"));
        store.process_hook_event(&make_event("s1", "SessionEnd", "ended"));
        assert_eq!(store.active_count(), 0);
        store.remove_ended_sessions();
        assert!(store.get_session("s1").is_none());
    }

    #[test]
    fn test_summaries_attention_first() {
        let mut store = SessionStore::new();
        store.process_hook_event(&make_event("s1", "UserPromptSubmit", "processing"));
        store.process_hook_event(&make_permission_event("s2", "Edit"));
        let summaries = store.get_summaries();
        assert_eq!(summaries.len(), 2);
        assert!(summaries[0].needs_attention);
        assert!(!summaries[1].needs_attention);
    }

    #[test]
    fn test_multiple_sessions() {
        let mut store = SessionStore::new();
        store.process_hook_event(&make_event("s1", "SessionStart", "waiting_for_input"));
        store.process_hook_event(&make_event("s2", "UserPromptSubmit", "processing"));
        store.process_hook_event(&make_event("s3", "SessionStart", "waiting_for_input"));
        assert_eq!(store.active_count(), 3);
    }
}
