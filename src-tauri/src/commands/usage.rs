use chrono::Utc;
use crate::usage::scanner::scan_usage;
use crate::usage::types::UsageReport;

/// 获取当前 token 用量报告（读取上次扫描缓存；若无缓存则立即扫描）。
///
/// # Stage 2 待办
/// - 在 `AppState` 中缓存 `Arc<Mutex<Option<UsageReport>>>`，
///   此命令直接返回缓存而不重新扫描文件系统。
#[tauri::command]
#[specta::specta]
pub async fn get_usage_report() -> Result<UsageReport, String> {
    let now = Utc::now();
    Ok(scan_usage(now))
}

/// 强制重新扫描 JSONL 并返回最新报告。
#[tauri::command]
#[specta::specta]
pub async fn refresh_usage_report() -> Result<UsageReport, String> {
    let now = Utc::now();
    Ok(scan_usage(now))
}
