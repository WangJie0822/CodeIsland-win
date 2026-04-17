use tauri::State;
use serde_json::Value;
use crate::app_state::AppState;
use crate::settings::{Settings, load_settings, save_settings};

// ─── 既有音效命令 ─────────────────────────────────────────────────────────────

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

// ─── 新增设置命令 ─────────────────────────────────────────────────────────────
// NOTE (Stage 2): 在 commands/mod.rs 追加 `pub mod settings;` 已有，
//   但需要在 mod.rs 的 pub use 或 main.rs invoke_handler 中注册以下命令：
//   get_settings / update_settings

/// 读取当前设置（从磁盘加载，若不存在返回默认值）。
///
/// # Errors
/// 不会返回错误；保持 `Result<Settings, String>` 签名以便前端统一处理。
#[tauri::command]
pub async fn get_settings() -> Result<Settings, String> {
    Ok(load_settings())
}

/// 以 JSON Patch 的方式更新设置，将变更字段合并到当前设置并持久化。
///
/// `patch` 只需包含要修改的字段，其余字段保持不变。
///
/// # Errors
/// 格式 `"[settings] <原因>"`
#[tauri::command]
pub async fn update_settings(patch: Value) -> Result<Settings, String> {
    let mut current = load_settings();

    // 将当前设置序列化为 Value，合并 patch，再反序列化
    let mut current_val = serde_json::to_value(&current)
        .map_err(|e| format!("[settings] 序列化当前设置失败: {}", e))?;

    if let (Value::Object(base), Value::Object(delta)) = (&mut current_val, patch) {
        for (k, v) in delta {
            base.insert(k, v);
        }
    }

    current = serde_json::from_value(current_val)
        .map_err(|e| format!("[settings] 合并 patch 后反序列化失败: {}", e))?;

    save_settings(&current)?;
    Ok(current)
}
