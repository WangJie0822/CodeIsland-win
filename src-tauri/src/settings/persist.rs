use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 刘海位置（屏幕坐标 + 可选屏幕 ID）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct NotchPosition {
    pub x: i32,
    pub y: i32,
    pub screen_id: Option<i32>,
}

/// 应用级设置，序列化到 `%APPDATA%\codeisland\settings.json`。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Settings {
    /// 指定在哪块屏幕显示刘海；None 表示自动选择主屏幕。
    pub target_screen: Option<i32>,
    /// 通知音效名称（"default" / "chime" / "soft" / "none"）。
    pub notification_sound: String,
    /// 界面语言（"en" / "zh"）。
    pub language: String,
    /// 是否启用像素猫装饰。
    pub pixel_cat_enabled: bool,
    /// 是否按项目分组会话。
    pub group_by_project: bool,
    /// 智能抑制：全屏应用时自动隐藏刘海。
    pub smart_suppression: bool,
    /// 鼠标离开后自动收起刘海。
    pub auto_collapse_on_leave: bool,
    /// 自动收起延迟（毫秒），范围 500–30000。
    pub auto_collapse_ms: u32,
    /// 登录时自动启动。
    pub launch_at_login: bool,
    /// 是否启用 Claude Hooks 接收。
    pub hooks_enabled: bool,
    /// 刘海自定义位置；None 表示顶部居中。
    pub notch_position: Option<NotchPosition>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            target_screen: None,
            notification_sound: "default".into(),
            language: "en".into(),
            pixel_cat_enabled: false,
            group_by_project: false,
            smart_suppression: false,
            auto_collapse_on_leave: true,
            auto_collapse_ms: 3000,
            launch_at_login: false,
            hooks_enabled: true,
            notch_position: None,
        }
    }
}

/// 返回 settings.json 的路径：`%APPDATA%\codeisland\settings.json`。
fn settings_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("codeisland").join("settings.json"))
}

/// 从磁盘加载设置。若文件不存在或解析失败，返回默认值。
pub fn load_settings() -> Settings {
    let path = match settings_path() {
        Some(p) => p,
        None => return Settings::default(),
    };

    if !path.exists() {
        return Settings::default();
    }

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            log::warn!("[settings] 读取文件失败 {:?}: {}", path, e);
            return Settings::default();
        }
    };

    match serde_json::from_str::<Settings>(&content) {
        Ok(s) => s,
        Err(e) => {
            log::warn!("[settings] JSON 解析失败: {}，使用默认值", e);
            Settings::default()
        }
    }
}

/// 将设置保存到磁盘。
pub fn save_settings(settings: &Settings) -> Result<(), String> {
    let path = settings_path()
        .ok_or_else(|| "[settings] 无法确定配置目录".to_string())?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("[settings] 创建目录失败: {}", e))?;
    }

    let content = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("[settings] 序列化失败: {}", e))?;

    std::fs::write(&path, content)
        .map_err(|e| format!("[settings] 写入文件失败: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    /// 在临时目录写入/读取，验证 roundtrip。
    #[test]
    fn settings_roundtrip() {
        let dir = tempdir().expect("创建临时目录失败");
        let path = dir.path().join("settings.json");

        let original = Settings {
            target_screen: Some(1),
            notification_sound: "chime".into(),
            language: "zh".into(),
            pixel_cat_enabled: true,
            group_by_project: true,
            smart_suppression: false,
            auto_collapse_on_leave: true,
            auto_collapse_ms: 5000,
            launch_at_login: true,
            hooks_enabled: false,
            notch_position: Some(NotchPosition { x: 100, y: 0, screen_id: Some(1) }),
        };

        // 序列化写入
        let json = serde_json::to_string_pretty(&original).unwrap();
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(json.as_bytes()).unwrap();

        // 反序列化读取
        let content = std::fs::read_to_string(&path).unwrap();
        let loaded: Settings = serde_json::from_str(&content).unwrap();

        assert_eq!(loaded, original);
    }

    #[test]
    fn default_settings_are_sensible() {
        let s = Settings::default();
        assert_eq!(s.notification_sound, "default");
        assert_eq!(s.language, "en");
        assert!(s.hooks_enabled);
        assert!(s.auto_collapse_on_leave);
        assert_eq!(s.auto_collapse_ms, 3000);
        assert!(!s.launch_at_login);
        assert!(s.notch_position.is_none());
    }

    #[test]
    fn load_settings_returns_default_if_no_file() {
        // 在无法预期 %APPDATA% 存在性的测试环境下，
        // 若文件不存在应静默返回默认值（不 panic）。
        let s = load_settings();
        // 只检查未崩溃，且 hooks_enabled 默认 true
        assert!(s.hooks_enabled);
    }
}
