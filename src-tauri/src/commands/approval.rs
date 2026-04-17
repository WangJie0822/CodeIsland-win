use tauri::{State, Emitter, AppHandle};
use crate::app_state::AppState;
use crate::hook::protocol::HookResponse;

#[tauri::command]
#[specta::specta]
pub async fn approve_permission(
    state: State<'_, AppState>,
    app: AppHandle,
    session_id: String,
) -> Result<bool, String> {
    state.hook_server.respond(&session_id, HookResponse::allow()).await;

    let mut store = state.store.lock().await;
    let result = store.process_approval(&session_id);
    drop(store);

    let _ = app.emit("sessions-updated", ());
    Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn deny_permission(
    state: State<'_, AppState>,
    app: AppHandle,
    session_id: String,
    reason: Option<String>,
) -> Result<bool, String> {
    let reason_text = reason.unwrap_or_else(|| "Denied by user via CodeIsland".into());
    state.hook_server.respond(&session_id, HookResponse::deny(&reason_text)).await;

    let mut store = state.store.lock().await;
    let result = store.process_denial(&session_id);
    drop(store);

    let _ = app.emit("sessions-updated", ());
    Ok(result)
}
