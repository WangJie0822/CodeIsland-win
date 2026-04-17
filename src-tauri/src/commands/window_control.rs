use tauri::{AppHandle, LogicalPosition, LogicalSize, Manager, Runtime};

#[tauri::command]
#[specta::specta]
pub async fn set_ignore_cursor_events<R: Runtime>(
    app: AppHandle<R>,
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
pub async fn set_window_size<R: Runtime>(
    app: AppHandle<R>,
    window_label: String,
    width: f64,
    height: f64,
) -> Result<(), String> {
    if !(width.is_finite() && height.is_finite()) || width <= 0.0 || height <= 0.0 {
        return Err(format!("尺寸非法: {}x{}", width, height));
    }
    let window = app
        .get_webview_window(&window_label)
        .ok_or_else(|| format!("窗口不存在: {}", window_label))?;
    window
        .set_size(LogicalSize::new(width, height))
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn set_window_position<R: Runtime>(
    app: AppHandle<R>,
    window_label: String,
    x: f64,
    y: f64,
) -> Result<(), String> {
    if !(x.is_finite() && y.is_finite()) {
        return Err(format!("位置非法: ({}, {})", x, y));
    }
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

    #[tokio::test]
    async fn test_set_window_size_rejects_zero() {
        let app = tauri::test::mock_app();
        let err = set_window_size(app.handle().clone(), "island".into(), 0.0, 100.0)
            .await
            .unwrap_err();
        assert!(err.contains("尺寸非法"));
    }

    #[tokio::test]
    async fn test_set_window_size_rejects_nan() {
        let app = tauri::test::mock_app();
        let err = set_window_size(
            app.handle().clone(),
            "island".into(),
            f64::NAN,
            100.0,
        )
        .await
        .unwrap_err();
        assert!(err.contains("尺寸非法"));
    }

    #[tokio::test]
    async fn test_set_window_position_rejects_nan() {
        let app = tauri::test::mock_app();
        let err = set_window_position(
            app.handle().clone(),
            "island".into(),
            f64::NAN,
            0.0,
        )
        .await
        .unwrap_err();
        assert!(err.contains("位置非法"));
    }

    #[tokio::test]
    async fn test_commands_report_missing_window() {
        let app = tauri::test::mock_app();
        let err =
            set_ignore_cursor_events(app.handle().clone(), "nonexistent".into(), true)
                .await
                .unwrap_err();
        assert!(err.contains("窗口不存在"));
    }
}
