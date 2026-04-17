use crate::buddy::types::BuddyState;

/// 获取当前 Buddy 状态（注册表 + 解锁状态 + 当前选中）
///
/// **Stage 2 待办**：
/// - 在 `commands/mod.rs` 中添加 `pub mod buddy;`
/// - 在 `lib.rs` / `main.rs` 的 `invoke_handler` 中注册 `buddy::get_buddy` / `buddy::switch_buddy`
/// - 在 `app_state.rs` 添加 `AppEvent::BuddyUnlocked(Buddy)` variant
/// - 在 `main.rs` 订阅协程中 emit `codeisland:buddy:unlocked`
#[tauri::command]
pub async fn get_buddy() -> Result<BuddyState, String> {
    crate::buddy::ensure_first_unlock();
    Ok(crate::buddy::build_state())
}

/// 切换当前选中的 Buddy（id 必须已解锁）
#[tauri::command]
pub async fn switch_buddy(id: String) -> Result<BuddyState, String> {
    crate::buddy::switch_current(&id)
        .map_err(|e| format!("[buddy] {}", e))
}
