use serde::Serialize;
use chrono::{DateTime, Local};
use crate::session::phase::{SessionPhase, PermissionContext};

#[derive(Debug, Clone, Default, Serialize)]
pub struct ConversationInfo {
    pub summary: Option<String>,
    pub last_message: Option<String>,
    pub last_message_role: Option<String>,
    pub last_tool_name: Option<String>,
    pub latest_user_message: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SessionState {
    pub session_id: String,
    pub cwd: String,
    pub project_name: String,
    pub pid: Option<u32>,
    pub tty: Option<String>,
    pub terminal_app: Option<String>,
    pub phase: SessionPhase,
    pub conversation_info: ConversationInfo,
    pub last_activity: DateTime<Local>,
    pub created_at: DateTime<Local>,
}

impl SessionState {
    pub fn new(session_id: String, cwd: String) -> Self {
        let project_name = cwd
            .rsplit(['/', '\\'])
            .find(|s| !s.is_empty())
            .unwrap_or("unknown")
            .to_string();
        Self {
            session_id, cwd, project_name,
            pid: None, tty: None, terminal_app: None,
            phase: SessionPhase::Idle,
            conversation_info: ConversationInfo::default(),
            last_activity: Local::now(),
            created_at: Local::now(),
        }
    }

    pub fn active_permission(&self) -> Option<&PermissionContext> {
        self.phase.approval_context()
    }

    pub fn pending_tool_name(&self) -> Option<&str> {
        self.active_permission().map(|p| p.tool_name.as_str())
    }

    pub fn touch(&mut self) {
        self.last_activity = Local::now();
    }
}
