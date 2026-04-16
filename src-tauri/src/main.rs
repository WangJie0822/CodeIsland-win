#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_state;
mod commands;
mod hook;
mod session;
mod terminal;
mod sound;

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{Emitter, Manager};

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            let (event_tx, _) = tokio::sync::broadcast::channel::<String>(256);

            let store = Arc::new(Mutex::new(session::store::SessionStore::new()));
            let hook_server = Arc::new(hook::pipe_server::HookServer::new(
                store.clone(),
                event_tx.clone(),
            ));
            let sound = Arc::new(Mutex::new(sound::manager::SoundManager::new()));

            let state = app_state::AppState {
                store: store.clone(),
                hook_server: hook_server.clone(),
                sound: sound.clone(),
                event_tx: event_tx.clone(),
            };

            app.manage(state);

            // 安装 hook 脚本
            let script_bytes = include_bytes!("../resources/codeisland-state.py");
            hook::installer::install_if_needed(script_bytes);

            // 启动 Hook IPC 服务
            let server = hook_server.clone();
            tauri::async_runtime::spawn(async move {
                server.start().await;
            });

            // 启动 JSONL 监听定时扫描
            let store_for_jsonl = store.clone();
            let event_tx_for_jsonl = event_tx.clone();
            tauri::async_runtime::spawn(async move {
                let mut watcher = session::jsonl_watcher::JsonlWatcher::new(
                    store_for_jsonl,
                    event_tx_for_jsonl,
                );
                loop {
                    watcher.scan_all().await;
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            });

            // 启动进程存活检查
            let store_for_proc = store.clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    let mut store = store_for_proc.lock().await;
                    store.remove_ended_sessions();
                }
            });

            // 监听 broadcast 事件，转发到前端
            let app_handle = app.handle().clone();
            let mut event_rx = event_tx.subscribe();
            tauri::async_runtime::spawn(async move {
                while let Ok(_session_id) = event_rx.recv().await {
                    let _ = app_handle.emit("sessions-updated", ());
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::session::get_sessions,
            commands::session::get_session_count,
            commands::session::send_to_terminal,
            commands::approval::approve_permission,
            commands::approval::deny_permission,
            commands::settings::get_sound_enabled,
            commands::settings::set_sound_enabled,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Code Island 失败");
}
