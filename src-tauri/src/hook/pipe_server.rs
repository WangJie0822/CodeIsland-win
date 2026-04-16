use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use log::{info, warn, error};
use crate::hook::protocol::{HookEvent, HookResponse};
use crate::session::store::SessionStore;

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
        #[cfg(target_os = "windows")]
        self.start_named_pipe().await;

        #[cfg(not(target_os = "windows"))]
        self.start_unix_socket().await;
    }

    #[cfg(target_os = "windows")]
    async fn start_named_pipe(&self) {
        use tokio::net::windows::named_pipe::ServerOptions;

        let pipe_name = r"\\.\pipe\codeisland";
        info!("[PipeServer] 启动 Named Pipe: {}", pipe_name);

        loop {
            let server = match ServerOptions::new()
                .first_pipe_instance(false)
                .create(pipe_name)
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

    #[cfg(not(target_os = "windows"))]
    async fn start_unix_socket(&self) {
        use tokio::net::UnixListener;

        let socket_path = "/tmp/codeisland.sock";
        let _ = std::fs::remove_file(socket_path);

        let listener = match UnixListener::bind(socket_path) {
            Ok(l) => l,
            Err(e) => {
                error!("[PipeServer] 绑定 Unix socket 失败: {}", e);
                return;
            }
        };

        info!("[PipeServer] 启动 Unix Socket: {}", socket_path);

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let store = self.store.clone();
                    let pending = self.pending.clone();
                    let event_tx = self.event_tx.clone();

                    tokio::spawn(async move {
                        handle_connection(stream, store, pending, event_tx).await;
                    });
                }
                Err(e) => {
                    error!("[PipeServer] accept 失败: {}", e);
                }
            }
        }
    }
}

async fn handle_connection<S>(
    mut stream: S,
    store: Arc<Mutex<SessionStore>>,
    pending: Arc<Mutex<std::collections::HashMap<String, tokio::sync::oneshot::Sender<HookResponse>>>>,
    event_tx: tokio::sync::broadcast::Sender<String>,
) where
    S: AsyncReadExt + AsyncWriteExt + Unpin,
{
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
                info!("[PipeServer] 已回复审批: {} → {}", &session_id[..8.min(session_id.len())], response.decision);
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
