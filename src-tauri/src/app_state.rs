use std::sync::Arc;
use tokio::sync::Mutex;
use crate::session::store::SessionStore;
use crate::hook::pipe_server::HookServer;
use crate::sound::manager::SoundManager;

pub struct AppState {
    pub store: Arc<Mutex<SessionStore>>,
    pub hook_server: Arc<HookServer>,
    pub sound: Arc<Mutex<SoundManager>>,
    pub event_tx: tokio::sync::broadcast::Sender<String>,
}
