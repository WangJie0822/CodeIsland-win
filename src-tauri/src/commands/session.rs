use tauri::State;
use crate::app_state::AppState;
use crate::session::store::SessionSummary;

#[tauri::command]
pub async fn get_sessions(state: State<'_, AppState>) -> Result<Vec<SessionSummary>, String> {
    let store = state.store.lock().await;
    Ok(store.get_summaries())
}

#[tauri::command]
pub async fn get_session_count(state: State<'_, AppState>) -> Result<usize, String> {
    let store = state.store.lock().await;
    Ok(store.active_count())
}

#[tauri::command]
pub async fn send_to_terminal(
    state: State<'_, AppState>,
    session_id: String,
    text: String,
) -> Result<bool, String> {
    let store = state.store.lock().await;
    let pid = store.get_session(&session_id).and_then(|s| s.pid);
    drop(store);

    match pid {
        Some(pid) => Ok(crate::terminal::writer::send_text_to_terminal(&text, pid)),
        None => Err("会话无 PID".into()),
    }
}
