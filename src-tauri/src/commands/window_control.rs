use tauri::{AppHandle, LogicalPosition, LogicalSize, Manager};

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
}
