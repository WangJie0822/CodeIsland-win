use tauri::{AppHandle, LogicalPosition, LogicalSize, Manager};

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
}

pub const KNOWN_WINDOW_LABELS: &[&str] = &[
    "island",
    "settings",
    "buddy",
    "usage",
    "presets",
    "notch-live-edit",
];

pub fn validate_window_label(label: &str) -> Result<(), String> {
    if KNOWN_WINDOW_LABELS.contains(&label) {
        Ok(())
    } else {
        Err(format!("[window] 未知窗口 label: {}", label))
    }
}

/// 计算 island 窗口顶部居中坐标（逻辑像素）。
///
/// `screen_w` / `screen_h` 是物理像素宽高；`scale` 是 monitor.scale_factor；
/// `win_w` / `win_h` 是逻辑尺寸。返回 (x, y) 逻辑坐标，y=0 固定顶部。
pub fn center_island_window(
    screen_w: f64,
    screen_h: f64,
    scale: f64,
    win_w: f64,
    win_h: f64,
) -> (f64, f64) {
    let _ = screen_h;
    let _ = win_h;
    let logical_w = screen_w / scale;
    let x = (logical_w - win_w) / 2.0;
    (x, 0.0)
}

fn validate_size(width: f64, height: f64) -> Result<(), String> {
    if !(width.is_finite() && height.is_finite()) || width <= 0.0 || height <= 0.0 {
        return Err(format!("尺寸非法: {}x{}", width, height));
    }
    Ok(())
}

fn validate_position(x: f64, y: f64) -> Result<(), String> {
    if !(x.is_finite() && y.is_finite()) {
        return Err(format!("位置非法: ({}, {})", x, y));
    }
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn set_ignore_cursor_events(
    app: AppHandle,
    window_label: String,
    enabled: bool,
) -> Result<(), String> {
    let window = app
        .get_webview_window(&window_label)
        .ok_or_else(|| format!("窗口不存在: {}", window_label))?;
    window
        .set_ignore_cursor_events(enabled)
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn set_window_size(
    app: AppHandle,
    window_label: String,
    width: f64,
    height: f64,
) -> Result<(), String> {
    validate_size(width, height)?;
    let window = app
        .get_webview_window(&window_label)
        .ok_or_else(|| format!("窗口不存在: {}", window_label))?;
    window
        .set_size(LogicalSize::new(width, height))
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn set_window_position(
    app: AppHandle,
    window_label: String,
    x: f64,
    y: f64,
) -> Result<(), String> {
    validate_position(x, y)?;
    let window = app
        .get_webview_window(&window_label)
        .ok_or_else(|| format!("窗口不存在: {}", window_label))?;
    window
        .set_position(LogicalPosition::new(x, y))
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn open_view_window(app: tauri::AppHandle, label: String) -> Result<(), String> {
    validate_window_label(&label)?;
    let window = app
        .get_webview_window(&label)
        .ok_or_else(|| format!("[window] 窗口未创建: {}", label))?;
    window
        .show()
        .map_err(|e| format!("[window] show 失败: {}", e))?;
    window
        .set_focus()
        .map_err(|e| format!("[window] focus 失败: {}", e))?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_window_position(
    app: tauri::AppHandle,
    label: String,
) -> Result<WindowPosition, String> {
    validate_window_label(&label)?;
    let window = app
        .get_webview_window(&label)
        .ok_or_else(|| format!("[window] 窗口未创建: {}", label))?;
    let physical = window
        .outer_position()
        .map_err(|e| format!("[window] 读取位置失败: {}", e))?;
    let scale = window
        .scale_factor()
        .map_err(|e| format!("[window] 读取 DPI scale 失败: {}", e))?;
    let logical = physical.to_logical::<i32>(scale);
    Ok(WindowPosition { x: logical.x, y: logical.y })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_size_rejects_zero() {
        assert!(validate_size(0.0, 100.0).unwrap_err().contains("尺寸非法"));
        assert!(validate_size(100.0, 0.0).unwrap_err().contains("尺寸非法"));
    }

    #[test]
    fn validate_size_rejects_negative() {
        assert!(validate_size(-1.0, 100.0).unwrap_err().contains("尺寸非法"));
    }

    #[test]
    fn validate_size_rejects_nan_inf() {
        assert!(validate_size(f64::NAN, 100.0).unwrap_err().contains("尺寸非法"));
        assert!(validate_size(100.0, f64::INFINITY).unwrap_err().contains("尺寸非法"));
    }

    #[test]
    fn validate_size_accepts_positive() {
        assert!(validate_size(220.0, 32.0).is_ok());
    }

    #[test]
    fn validate_position_rejects_nan() {
        assert!(validate_position(f64::NAN, 0.0).unwrap_err().contains("位置非法"));
        assert!(validate_position(0.0, f64::NAN).unwrap_err().contains("位置非法"));
    }

    #[test]
    fn validate_position_accepts_finite() {
        assert!(validate_position(-100.0, 0.0).is_ok());
        assert!(validate_position(1920.0, 1080.0).is_ok());
    }

    #[test]
    fn validate_window_label_accepts_known() {
        assert!(super::validate_window_label("island").is_ok());
        assert!(super::validate_window_label("settings").is_ok());
        assert!(super::validate_window_label("buddy").is_ok());
        assert!(super::validate_window_label("usage").is_ok());
        assert!(super::validate_window_label("presets").is_ok());
        assert!(super::validate_window_label("notch-live-edit").is_ok());
    }

    #[test]
    fn validate_window_label_rejects_unknown_with_prefix() {
        let err = super::validate_window_label("foobar").unwrap_err();
        assert!(
            err.starts_with("[window]"),
            "error should start with [window] prefix, got: {}",
            err
        );
        assert!(err.contains("foobar"), "error should contain label, got: {}", err);
    }

    #[test]
    fn validate_window_label_rejects_empty() {
        assert!(super::validate_window_label("").is_err());
    }

    #[test]
    fn window_position_fields_are_i32() {
        let pos = super::WindowPosition { x: 100i32, y: 200i32 };
        assert_eq!(pos.x, 100);
        assert_eq!(pos.y, 200);
    }

    #[test]
    fn center_island_window_1920x1080_scale_1() {
        let (x, y) = super::center_island_window(1920.0, 1080.0, 1.0, 220.0, 32.0);
        assert_eq!(x, 850.0);
        assert_eq!(y, 0.0);
    }

    #[test]
    fn center_island_window_2560x1440_scale_1_25() {
        let (x, y) = super::center_island_window(2560.0, 1440.0, 1.25, 220.0, 32.0);
        assert_eq!(x, 914.0);
        assert_eq!(y, 0.0);
    }

    #[test]
    fn center_island_window_uses_physical_width_divided_by_scale() {
        let (x, _) = super::center_island_window(3840.0, 2160.0, 2.0, 220.0, 32.0);
        assert_eq!(x, 850.0);
    }
}
