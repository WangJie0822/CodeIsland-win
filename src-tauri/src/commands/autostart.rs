use crate::autostart::{get_autostart, set_autostart};

// NOTE (Stage 2): 在 commands/mod.rs 追加 `pub mod autostart;`，
//   并在 main.rs invoke_handler 中注册 get_autostart / set_autostart。

/// 查询当前用户是否已配置登录自启动。
///
/// # Returns
/// `Ok(true)` 若已启用。
///
/// # Errors
/// 格式 `"[registry] <原因>"`
#[tauri::command]
pub async fn cmd_get_autostart() -> Result<bool, String> {
    get_autostart()
}

/// 设置或清除登录自启动。
///
/// # Errors
/// 格式 `"[registry] <原因>"`
#[tauri::command]
pub async fn cmd_set_autostart(enabled: bool) -> Result<bool, String> {
    set_autostart(enabled)
}
