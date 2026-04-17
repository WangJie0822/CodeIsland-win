use std::sync::Arc;
use tokio::sync::Mutex;
use crate::session::store::SessionStore;
use crate::hook::pipe_server::HookServer;
use crate::sound::manager::SoundManager;

/// 应用内事件总线载荷。
///
/// 所有后端子系统通过 `event_tx.send(AppEvent::...)` 广播状态变化，
/// 由 `main.rs` 的订阅协程转发为规范化的 Tauri emit 事件名。
#[derive(Debug, Clone)]
pub enum AppEvent {
    SessionsUpdated,
}

impl AppEvent {
    /// 将事件映射到规范化的 Tauri emit 名称：`codeisland:<domain>:<action>`。
    pub fn topic(&self) -> &'static str {
        match self {
            AppEvent::SessionsUpdated => "codeisland:sessions:updated",
        }
    }
}

pub struct AppState {
    pub store: Arc<Mutex<SessionStore>>,
    pub hook_server: Arc<HookServer>,
    pub sound: Arc<Mutex<SoundManager>>,
    pub event_tx: tokio::sync::broadcast::Sender<AppEvent>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sessions_updated_topic() {
        assert_eq!(
            AppEvent::SessionsUpdated.topic(),
            "codeisland:sessions:updated"
        );
    }

    #[tokio::test]
    async fn test_broadcast_delivers_event() {
        let (tx, mut rx) = tokio::sync::broadcast::channel::<AppEvent>(4);
        tx.send(AppEvent::SessionsUpdated).unwrap();
        let received = rx.recv().await.unwrap();
        assert_eq!(received.topic(), "codeisland:sessions:updated");
    }
}
