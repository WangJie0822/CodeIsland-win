use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::windows::named_pipe::{NamedPipeServer, ServerOptions};
use log::{info, warn, error};
use crate::hook::protocol::{HookEvent, HookResponse};
use crate::session::store::SessionStore;

pub const PIPE_NAME: &str = r"\\.\pipe\codeisland";

pub struct HookServer {
    store: Arc<Mutex<SessionStore>>,
    pending: Arc<Mutex<std::collections::HashMap<String, tokio::sync::oneshot::Sender<HookResponse>>>>,
    event_tx: tokio::sync::broadcast::Sender<String>,
}

impl HookServer {
    pub fn new(
        store: Arc<Mutex<SessionStore>>,
        event_tx: tokio::sync::broadcast::Sender<String>,
    ) -> Self {
        Self {
            store,
            pending: Arc::new(Mutex::new(std::collections::HashMap::new())),
            event_tx,
        }
    }

    pub async fn respond(&self, session_id: &str, response: HookResponse) -> bool {
        let mut pending = self.pending.lock().await;
        if let Some(tx) = pending.remove(session_id) {
            let _ = tx.send(response);
            true
        } else {
            warn!("[PipeServer] 无待审批连接: {}", session_id);
            false
        }
    }

    pub async fn start(&self) {
        info!("[PipeServer] 启动 Named Pipe: {}", PIPE_NAME);

        loop {
            let server = match ServerOptions::new()
                .first_pipe_instance(false)
                .create(PIPE_NAME)
            {
                Ok(s) => s,
                Err(e) => {
                    error!("[PipeServer] 创建 pipe 失败: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    continue;
                }
            };

            if let Err(e) = server.connect().await {
                error!("[PipeServer] 等待连接失败: {}", e);
                continue;
            }

            let store = self.store.clone();
            let pending = self.pending.clone();
            let event_tx = self.event_tx.clone();

            tokio::spawn(async move {
                handle_connection(server, store, pending, event_tx).await;
            });
        }
    }
}

async fn handle_connection(
    mut stream: NamedPipeServer,
    store: Arc<Mutex<SessionStore>>,
    pending: Arc<Mutex<std::collections::HashMap<String, tokio::sync::oneshot::Sender<HookResponse>>>>,
    event_tx: tokio::sync::broadcast::Sender<String>,
) {
    let mut buf = vec![0u8; 65536];
    let n = match stream.read(&mut buf).await {
        Ok(0) => return,
        Ok(n) => n,
        Err(e) => {
            warn!("[PipeServer] 读取失败: {}", e);
            return;
        }
    };

    let event: HookEvent = match serde_json::from_slice(&buf[..n]) {
        Ok(e) => e,
        Err(e) => {
            warn!("[PipeServer] JSON 解析失败: {}", e);
            return;
        }
    };

    let session_id = event.session_id.clone();
    let expects_response = event.expects_response();

    {
        let mut store = store.lock().await;
        store.process_hook_event(&event);
    }

    let _ = event_tx.send(session_id.clone());

    if expects_response {
        let (tx, rx) = tokio::sync::oneshot::channel::<HookResponse>();
        {
            let mut p = pending.lock().await;
            p.insert(session_id.clone(), tx);
        }

        info!("[PipeServer] 等待用户审批: {}", &session_id[..8.min(session_id.len())]);

        match tokio::time::timeout(std::time::Duration::from_secs(300), rx).await {
            Ok(Ok(response)) => {
                let json = serde_json::to_vec(&response).unwrap_or_default();
                let _ = stream.write_all(&json).await;
                info!(
                    "[PipeServer] 已回复审批: {} → {}",
                    &session_id[..8.min(session_id.len())],
                    response.decision
                );
            }
            _ => {
                warn!("[PipeServer] 审批超时: {}", &session_id[..8.min(session_id.len())]);
                let mut p = pending.lock().await;
                p.remove(&session_id);
            }
        }
    } else {
        let _ = stream.write_all(b"ok\n").await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_name_constant() {
        assert_eq!(PIPE_NAME, r"\\.\pipe\codeisland");
    }

    #[test]
    fn test_pipe_server_is_windows_only() {
        // 避免 include_str! 自引用——把 needle 拆成两段字面量，编译期合并、源码搜索不命中
        let src = include_str!("pipe_server.rs");
        let unix_listener = concat!("Unix", "Listener");
        let cross_cfg = concat!("cfg(not(", "target_os");
        let unix_sock = concat!("/tmp/cod", "eisland.sock");
        assert!(!src.contains(unix_listener), "Unix 分支残留");
        assert!(!src.contains(cross_cfg), "跨平台 cfg 残留");
        assert!(!src.contains(unix_sock), "Unix socket 路径残留");
    }
}
