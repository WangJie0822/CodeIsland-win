use serde::Serialize;
use crate::hook::protocol::HookEvent;

/// 权限审批上下文
#[derive(Debug, Clone, Serialize)]
pub struct PermissionContext {
    pub tool_use_id: String,
    pub tool_name: String,
    pub tool_input: Option<serde_json::Value>,
    pub received_at: chrono::DateTime<chrono::Local>,
}

impl PartialEq for PermissionContext {
    fn eq(&self, other: &Self) -> bool {
        self.tool_use_id == other.tool_use_id && self.tool_name == other.tool_name
    }
}

/// 会话阶段状态机（与 macOS 版 SessionPhase.swift 对齐）
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "context")]
pub enum SessionPhase {
    Idle,
    Processing,
    WaitingForInput,
    WaitingForApproval(PermissionContext),
    Compacting,
    Ended,
}

impl PartialEq for SessionPhase {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Idle, Self::Idle)
                | (Self::Processing, Self::Processing)
                | (Self::WaitingForInput, Self::WaitingForInput)
                | (Self::Compacting, Self::Compacting)
                | (Self::Ended, Self::Ended)
                | (Self::WaitingForApproval(_), Self::WaitingForApproval(_))
        )
    }
}

impl SessionPhase {
    pub fn can_transition(&self, next: &SessionPhase) -> bool {
        use SessionPhase::*;
        match (self, next) {
            (Ended, _) => false,
            (_, Ended) => true,
            (Idle, Processing) => true,
            (Idle, WaitingForApproval(_)) => true,
            (Idle, Compacting) => true,
            (Idle, WaitingForInput) => true,
            (Processing, WaitingForInput) => true,
            (Processing, WaitingForApproval(_)) => true,
            (Processing, Compacting) => true,
            (Processing, Idle) => true,
            (WaitingForInput, Processing) => true,
            (WaitingForInput, Idle) => true,
            (WaitingForInput, Compacting) => true,
            (WaitingForApproval(_), Processing) => true,
            (WaitingForApproval(_), Idle) => true,
            (WaitingForApproval(_), WaitingForInput) => true,
            (WaitingForApproval(_), WaitingForApproval(_)) => true,
            (Compacting, Processing) => true,
            (Compacting, Idle) => true,
            (Compacting, WaitingForInput) => true,
            _ => self == next,
        }
    }

    pub fn transition(&self, next: SessionPhase) -> Option<SessionPhase> {
        if self.can_transition(&next) { Some(next) } else { None }
    }

    pub fn needs_attention(&self) -> bool {
        matches!(self, Self::WaitingForApproval(_) | Self::WaitingForInput)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, Self::Processing | Self::Compacting)
    }

    pub fn approval_context(&self) -> Option<&PermissionContext> {
        if let Self::WaitingForApproval(ctx) = self { Some(ctx) } else { None }
    }

    pub fn from_hook_event(event: &HookEvent) -> Self {
        if event.event == "PreCompact" {
            return Self::Compacting;
        }
        if event.expects_response() {
            if let Some(ref tool) = event.tool {
                return Self::WaitingForApproval(PermissionContext {
                    tool_use_id: event.tool_use_id.clone().unwrap_or_default(),
                    tool_name: tool.clone(),
                    tool_input: event.tool_input.clone(),
                    received_at: chrono::Local::now(),
                });
            }
        }
        if event.event == "Notification" && event.notification_type.as_deref() == Some("idle_prompt") {
            return Self::Idle;
        }
        match event.status.as_str() {
            "waiting_for_input" => Self::WaitingForInput,
            "running_tool" | "processing" | "starting" => Self::Processing,
            "compacting" => Self::Compacting,
            "ended" => Self::Ended,
            _ => Self::Idle,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Idle => "idle",
            Self::Processing => "processing",
            Self::WaitingForInput => "waitingForInput",
            Self::WaitingForApproval(_) => "waitingForApproval",
            Self::Compacting => "compacting",
            Self::Ended => "ended",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ctx(tool: &str) -> PermissionContext {
        PermissionContext {
            tool_use_id: "tu-1".into(),
            tool_name: tool.into(),
            tool_input: None,
            received_at: chrono::Local::now(),
        }
    }

    #[test]
    fn test_ended_is_terminal() {
        let ended = SessionPhase::Ended;
        assert!(!ended.can_transition(&SessionPhase::Idle));
        assert!(!ended.can_transition(&SessionPhase::Processing));
        assert!(!ended.can_transition(&SessionPhase::Ended));
    }

    #[test]
    fn test_any_to_ended() {
        for phase in [
            SessionPhase::Idle,
            SessionPhase::Processing,
            SessionPhase::WaitingForInput,
            SessionPhase::WaitingForApproval(make_ctx("Bash")),
            SessionPhase::Compacting,
        ] {
            assert!(phase.can_transition(&SessionPhase::Ended));
        }
    }

    #[test]
    fn test_approval_to_approval_allowed() {
        let a = SessionPhase::WaitingForApproval(make_ctx("Bash"));
        let b = SessionPhase::WaitingForApproval(make_ctx("Edit"));
        assert!(a.can_transition(&b));
    }

    #[test]
    fn test_idle_to_processing() {
        assert!(SessionPhase::Idle.can_transition(&SessionPhase::Processing));
    }

    #[test]
    fn test_waiting_for_input_cannot_go_to_approval_directly() {
        let wfi = SessionPhase::WaitingForInput;
        let wfa = SessionPhase::WaitingForApproval(make_ctx("Bash"));
        assert!(!wfi.can_transition(&wfa));
    }

    #[test]
    fn test_from_hook_event_processing() {
        let event = HookEvent {
            session_id: "s1".into(), cwd: "/tmp".into(),
            event: "UserPromptSubmit".into(), status: "processing".into(),
            pid: None, tty: None, tool: None, tool_input: None,
            tool_use_id: None, notification_type: None, message: None,
        };
        assert_eq!(SessionPhase::from_hook_event(&event).name(), "processing");
    }

    #[test]
    fn test_from_hook_event_permission_request() {
        let event = HookEvent {
            session_id: "s1".into(), cwd: "/tmp".into(),
            event: "PermissionRequest".into(), status: "waiting_for_approval".into(),
            pid: None, tty: None,
            tool: Some("Bash".into()),
            tool_input: Some(serde_json::json!({"command": "ls"})),
            tool_use_id: Some("tu-1".into()),
            notification_type: None, message: None,
        };
        let phase = SessionPhase::from_hook_event(&event);
        assert_eq!(phase.name(), "waitingForApproval");
        assert_eq!(phase.approval_context().unwrap().tool_name, "Bash");
    }
}
