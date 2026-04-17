use serde::{Deserialize, Serialize};
use tauri::AppHandle;

/// 单块屏幕信息。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ScreenInfo {
    /// 屏幕唯一标识（从 0 开始的枚举 ID）。
    pub id: i32,
    /// 屏幕名称（来自 Tauri MonitorHandle，可能为空字符串）。
    pub name: String,
    /// 物理像素宽度。
    pub width: u32,
    /// 物理像素高度。
    pub height: u32,
    /// 是否为主显示器。
    pub is_primary: bool,
}

/// 通过 Tauri `app.available_monitors()` 枚举所有屏幕。
///
/// # Errors
/// 格式 `"[screens] <原因>"`。
pub fn get_screens(app: &AppHandle) -> Result<Vec<ScreenInfo>, String> {
    let monitors = app
        .available_monitors()
        .map_err(|e| format!("[screens] 枚举显示器失败: {}", e))?;

    // 获取主显示器用于比较
    let primary = app.primary_monitor().ok().flatten();
    let primary_pos = primary.as_ref().map(|m| m.position());

    let screens: Vec<ScreenInfo> = monitors
        .into_iter()
        .enumerate()
        .map(|(idx, m)| {
            let size = m.size();
            let is_primary = primary_pos
                .map(|pp| pp == m.position())
                .unwrap_or(idx == 0);
            ScreenInfo {
                id: idx as i32,
                name: m.name().cloned().unwrap_or_default(),
                width: size.width,
                height: size.height,
                is_primary,
            }
        })
        .collect();

    Ok(screens)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ScreenInfo 序列化/反序列化 roundtrip。
    #[test]
    fn screen_info_serde_roundtrip() {
        let info = ScreenInfo {
            id: 0,
            name: "DISPLAY1".into(),
            width: 1920,
            height: 1080,
            is_primary: true,
        };

        let json = serde_json::to_string(&info).unwrap();
        let loaded: ScreenInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded, info);
    }

    #[test]
    fn screen_info_fields_accessible() {
        let info = ScreenInfo {
            id: 1,
            name: "Secondary".into(),
            width: 2560,
            height: 1440,
            is_primary: false,
        };
        assert_eq!(info.id, 1);
        assert!(!info.is_primary);
        assert_eq!(info.width, 2560);
    }
}
