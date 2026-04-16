use serde::{Deserialize, Serialize};

/// Python hook 脚本发来的事件（与 macOS 版 HookEvent 字段对齐）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookEvent {
    pub session_id: String,
    #[serde(default)]
    pub cwd: String,
    #[serde(default)]
    pub event: String,
    #[serde(default)]
    pub status: String,
    pub pid: Option<u32>,
    pub tty: Option<String>,
    pub tool: Option<String>,
    pub tool_input: Option<serde_json::Value>,
    pub tool_use_id: Option<String>,
    pub notification_type: Option<String>,
    pub message: Option<String>,
}

impl HookEvent {
    pub fn expects_response(&self) -> bool {
        self.event == "PermissionRequest" && self.status == "waiting_for_approval"
    }

    pub fn is_tool_event(&self) -> bool {
        matches!(self.event.as_str(), "PreToolUse" | "PostToolUse" | "PermissionRequest")
    }

    pub fn should_sync_file(&self) -> bool {
        matches!(self.event.as_str(), "UserPromptSubmit" | "PreToolUse" | "PostToolUse" | "Stop")
    }
}

/// 回写给 Python hook 脚本的审批决策
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookResponse {
    pub decision: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl HookResponse {
    pub fn allow() -> Self {
        Self { decision: "allow".into(), reason: None }
    }

    pub fn deny(reason: impl Into<String>) -> Self {
        Self { decision: "deny".into(), reason: Some(reason.into()) }
    }
}

/// Named Pipe 上保持的待审批连接
#[derive(Debug)]
pub struct PendingPermission {
    pub session_id: String,
    pub tool_use_id: String,
    pub tool_name: String,
    pub event: HookEvent,
    pub received_at: chrono::DateTime<chrono::Local>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_processing_event() {
        let json = r#"{
            "session_id": "abc-123",
            "cwd": "/home/user/project",
            "event": "UserPromptSubmit",
            "status": "processing",
            "pid": 12345,
            "tty": "/dev/pts/0"
        }"#;
        let event: HookEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.session_id, "abc-123");
        assert_eq!(event.event, "UserPromptSubmit");
        assert_eq!(event.status, "processing");
        assert_eq!(event.pid, Some(12345));
        assert!(!event.expects_response());
        assert!(event.should_sync_file());
    }

    #[test]
    fn test_deserialize_permission_request() {
        let json = r#"{
            "session_id": "abc-123",
            "cwd": "/home/user/project",
            "event": "PermissionRequest",
            "status": "waiting_for_approval",
            "pid": 12345,
            "tool": "Bash",
            "tool_input": {"command": "rm -rf /tmp/test"}
        }"#;
        let event: HookEvent = serde_json::from_str(json).unwrap();
        assert!(event.expects_response());
        assert!(event.is_tool_event());
        assert_eq!(event.tool, Some("Bash".into()));
    }

    #[test]
    fn test_deserialize_minimal_event() {
        let json = r#"{"session_id": "x"}"#;
        let event: HookEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.session_id, "x");
        assert_eq!(event.cwd, "");
        assert_eq!(event.event, "");
        assert!(!event.expects_response());
    }

    #[test]
    fn test_serialize_hook_response() {
        let resp = HookResponse::allow();
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"decision\":\"allow\""));
        assert!(!json.contains("reason"));

        let resp = HookResponse::deny("用户拒绝");
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"deny\""));
        assert!(json.contains("用户拒绝"));
    }
}
