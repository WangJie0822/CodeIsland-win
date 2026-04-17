use tauri::State;
use crate::app_state::AppState;

#[tauri::command]
#[specta::specta]
pub async fn get_sound_enabled(_state: State<'_, AppState>) -> Result<bool, String> {
    Ok(true)
}

#[tauri::command]
#[specta::specta]
pub async fn set_sound_enabled(
    state: State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    let mut sound = state.sound.lock().await;
    sound.set_enabled(enabled);
    Ok(())
}
