use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, debug};
use crate::app_state::AppEvent;
use crate::session::state::ConversationInfo;
use crate::session::store::SessionStore;

struct ParseState {
    offset: u64,
    last_message: Option<String>,
    last_role: Option<String>,
    last_tool: Option<String>,
    latest_user_message: Option<String>,
}

pub struct JsonlWatcher {
    store: Arc<Mutex<SessionStore>>,
    event_tx: tokio::sync::broadcast::Sender<AppEvent>,
    states: HashMap<String, ParseState>,
    paths: HashMap<String, PathBuf>,
}

impl JsonlWatcher {
    pub fn new(
        store: Arc<Mutex<SessionStore>>,
        event_tx: tokio::sync::broadcast::Sender<AppEvent>,
    ) -> Self {
        Self { store, event_tx, states: HashMap::new(), paths: HashMap::new() }
    }

    pub fn register_session(&mut self, session_id: &str, cwd: &str) {
        if self.paths.contains_key(session_id) { return; }

        let claude_dir = dirs::home_dir()
            .map(|h| h.join(".claude"))
            .unwrap_or_default();

        let encoded = encode_cwd(cwd);
        let project_dir = claude_dir.join("projects").join(&encoded);
        let jsonl_path = project_dir.join(format!("{}.jsonl", session_id));

        if jsonl_path.exists() {
            info!("[JsonlWatcher] 注册: {} → {:?}", &session_id[..8.min(session_id.len())], jsonl_path);
            self.paths.insert(session_id.to_string(), jsonl_path);
            self.states.insert(session_id.to_string(), ParseState {
                offset: 0, last_message: None, last_role: None,
                last_tool: None, latest_user_message: None,
            });
        } else {
            debug!("[JsonlWatcher] JSONL 文件不存在: {:?}", jsonl_path);
        }
    }

    pub async fn parse_incremental(&mut self, session_id: &str) {
        let path = match self.paths.get(session_id) {
            Some(p) => p.clone(),
            None => return,
        };
        let state = match self.states.get_mut(session_id) {
            Some(s) => s,
            None => return,
        };

        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => return,
        };

        let bytes = content.as_bytes();
        if (bytes.len() as u64) <= state.offset { return; }

        let new_content = &content[state.offset as usize..];
        state.offset = bytes.len() as u64;

        for line in new_content.lines() {
            let line = line.trim();
            if line.is_empty() { continue; }
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                parse_jsonl_line(&json, state);
            }
        }

        let info = ConversationInfo {
            summary: None,
            last_message: state.last_message.clone(),
            last_message_role: state.last_role.clone(),
            last_tool_name: state.last_tool.clone(),
            latest_user_message: state.latest_user_message.clone(),
        };

        {
            let mut store = self.store.lock().await;
            store.update_conversation_info(session_id, info);
        }
        let _ = self.event_tx.send(AppEvent::SessionsUpdated);
    }

    pub async fn scan_all(&mut self) {
        let session_ids: Vec<String> = self.paths.keys().cloned().collect();
        for session_id in session_ids {
            self.parse_incremental(&session_id).await;
        }
    }
}

fn parse_jsonl_line(json: &serde_json::Value, state: &mut ParseState) {
    if json.get("type").and_then(|v| v.as_str()) == Some("assistant") {
        if let Some(message) = json.get("message") {
            if let Some(content) = message.get("content") {
                if let Some(text) = extract_text_from_content(content) {
                    state.last_message = Some(truncate(&text, 200));
                    state.last_role = Some("assistant".into());
                }
            }
        }
    }

    if json.get("type").and_then(|v| v.as_str()) == Some("user") {
        if let Some(message) = json.get("message") {
            if let Some(content) = message.get("content") {
                if let Some(text) = extract_text_from_content(content) {
                    state.last_message = Some(truncate(&text, 200));
                    state.last_role = Some("user".into());
                    state.latest_user_message = Some(truncate(&text, 200));
                }
            }
        }
    }

    if json.get("type").and_then(|v| v.as_str()) == Some("tool_result") {
        if let Some(name) = json.get("toolName").and_then(|v| v.as_str()) {
            state.last_tool = Some(name.to_string());
        }
    }
}

fn extract_text_from_content(content: &serde_json::Value) -> Option<String> {
    if let Some(s) = content.as_str() {
        return Some(s.to_string());
    }
    if let Some(blocks) = content.as_array() {
        let texts: Vec<&str> = blocks.iter()
            .filter_map(|b| {
                if b.get("type").and_then(|v| v.as_str()) == Some("text") {
                    b.get("text").and_then(|v| v.as_str())
                } else { None }
            })
            .collect();
        if !texts.is_empty() { return Some(texts.join("\n")); }
    }
    None
}

pub fn encode_cwd(cwd: &str) -> String {
    let mut encoded = String::new();
    for ch in cwd.chars() {
        match ch {
            '/' | '\\' => encoded.push('-'),
            ':' => {}
            _ => encoded.push(ch),
        }
    }
    if !encoded.starts_with('-') { encoded.insert(0, '-'); }
    encoded
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max { s.to_string() } else { format!("{}...", &s[..max]) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_cwd_unix() {
        assert_eq!(encode_cwd("/home/user/project"), "-home-user-project");
        assert_eq!(encode_cwd("/"), "-");
    }

    #[test]
    fn test_encode_cwd_windows() {
        assert_eq!(encode_cwd("C:\\Users\\wj\\project"), "-C-Users-wj-project");
    }

    #[test]
    fn test_parse_assistant_message() {
        let mut state = ParseState {
            offset: 0, last_message: None, last_role: None,
            last_tool: None, latest_user_message: None,
        };
        let json: serde_json::Value = serde_json::json!({
            "type": "assistant",
            "message": { "content": [{"type": "text", "text": "Hello, I can help you."}] }
        });
        parse_jsonl_line(&json, &mut state);
        assert_eq!(state.last_message.as_deref(), Some("Hello, I can help you."));
        assert_eq!(state.last_role.as_deref(), Some("assistant"));
    }

    #[test]
    fn test_parse_user_message() {
        let mut state = ParseState {
            offset: 0, last_message: None, last_role: None,
            last_tool: None, latest_user_message: None,
        };
        let json: serde_json::Value = serde_json::json!({
            "type": "user",
            "message": { "content": "Fix the bug" }
        });
        parse_jsonl_line(&json, &mut state);
        assert_eq!(state.latest_user_message.as_deref(), Some("Fix the bug"));
    }
}
