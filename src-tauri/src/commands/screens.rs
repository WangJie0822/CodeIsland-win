use tauri::AppHandle;
use crate::screens::ScreenInfo;

// NOTE (Stage 2): 在 commands/mod.rs 追加 `pub mod screens;`，
//   并在 main.rs invoke_handler 中注册 get_screens。

/// 枚举所有可用屏幕，返回列表。
///
/// # Errors
/// 格式 `"[screens] <原因>"`
#[tauri::command]
pub async fn get_screens(app: AppHandle) -> Result<Vec<ScreenInfo>, String> {
    crate::screens::get_screens(&app)
}
