/// Tauri 命令层：LaunchPreset CRUD + 启动。
///
/// **Stage 2 待办**：
/// - 在 `commands/mod.rs` 注册本模块（`pub mod presets;`）
/// - 在 `main.rs` `generate_handler!` 中添加 4 个命令
/// - 添加 `AppEvent::PresetsUpdated`，并在 main.rs 订阅后 emit `codeisland:presets:updated`
use crate::presets::launcher::{launch_preset_with_runner, RealCommandRunner};
use crate::presets::persist::{load_presets, save_presets};
use crate::presets::types::LaunchPreset;

/// 列出所有已保存的启动预设，按 sort_order 升序排序。
#[tauri::command]
pub async fn list_presets() -> Result<Vec<LaunchPreset>, String> {
    let mut presets = load_presets()?;
    presets.sort_by_key(|p| p.sort_order);
    Ok(presets)
}

/// 保存（新增或更新）一个 preset。
///
/// - 若 id 为空，自动生成新 id。
/// - 若已存在相同 id，替换；否则追加。
#[tauri::command]
pub async fn save_preset(preset: LaunchPreset) -> Result<LaunchPreset, String> {
    let mut presets = load_presets()?;

    // 若 id 为空，生成新 id
    let mut preset = preset;
    if preset.id.is_empty() {
        preset.id = LaunchPreset::new_id();
    }

    // 替换已有或追加
    if let Some(pos) = presets.iter().position(|p| p.id == preset.id) {
        presets[pos] = preset.clone();
    } else {
        presets.push(preset.clone());
    }

    save_presets(&presets)?;
    Ok(preset)
}

/// 删除指定 id 的 preset；id 不存在时静默成功。
#[tauri::command]
pub async fn delete_preset(id: String) -> Result<(), String> {
    let mut presets = load_presets()?;
    presets.retain(|p| p.id != id);
    save_presets(&presets)?;
    Ok(())
}

/// 启动指定 id 的 preset 对应的 Claude CLI 会话。
#[tauri::command]
pub async fn launch_preset(id: String) -> Result<(), String> {
    let presets = load_presets()?;
    let preset = presets
        .iter()
        .find(|p| p.id == id)
        .ok_or_else(|| format!("[preset_launch] 未找到 preset id={id}"))?;

    let runner = RealCommandRunner;
    launch_preset_with_runner(preset, &runner)
}
