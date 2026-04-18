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
    SettingsChanged,
    BuddyUnlocked,
    UsageUpdated,
    PresetsUpdated,
    NotchPositionChanged,
}

impl AppEvent {
    /// 将事件映射到规范化的 Tauri emit 名称：`codeisland:<domain>:<action>`。
    pub fn topic(&self) -> &'static str {
        match self {
            AppEvent::SessionsUpdated => "codeisland:sessions:updated",
            AppEvent::SettingsChanged => "codeisland:settings:changed",
            AppEvent::BuddyUnlocked => "codeisland:buddy:unlocked",
            AppEvent::UsageUpdated => "codeisland:usage:updated",
            AppEvent::PresetsUpdated => "codeisland:presets:updated",
            AppEvent::NotchPositionChanged => "codeisland:notch:position-changed",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EventBusAction {
    Emit(&'static str),
    Warn(u64),
    Break,
}

/// 把 broadcast recv 的结果分派为具体动作。
///
/// 纯函数方便测试；主循环在 main.rs 中按返回值执行副作用。
pub fn handle_event_bus_recv(
    result: Result<AppEvent, tokio::sync::broadcast::error::RecvError>,
) -> EventBusAction {
    use tokio::sync::broadcast::error::RecvError;
    match result {
        Ok(event) => EventBusAction::Emit(event.topic()),
        Err(RecvError::Lagged(n)) => EventBusAction::Warn(n),
        Err(RecvError::Closed) => EventBusAction::Break,
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

    #[test]
    fn notch_position_changed_topic() {
        assert_eq!(
            AppEvent::NotchPositionChanged.topic(),
            "codeisland:notch:position-changed"
        );
    }

    use tokio::sync::broadcast::error::RecvError;

    #[test]
    fn handle_event_bus_recv_ok_returns_emit() {
        let result = super::handle_event_bus_recv(Ok(AppEvent::SessionsUpdated));
        assert!(matches!(result, super::EventBusAction::Emit("codeisland:sessions:updated")));
    }

    #[test]
    fn handle_event_bus_recv_lagged_returns_warn() {
        let result = super::handle_event_bus_recv(Err(RecvError::Lagged(3)));
        assert!(matches!(result, super::EventBusAction::Warn(3)));
    }

    #[test]
    fn handle_event_bus_recv_closed_returns_break() {
        let result = super::handle_event_bus_recv(Err(RecvError::Closed));
        assert!(matches!(result, super::EventBusAction::Break));
    }
}
