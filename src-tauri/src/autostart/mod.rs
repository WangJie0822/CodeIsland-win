#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{get_autostart, set_autostart};

#[cfg(not(target_os = "windows"))]
pub fn get_autostart() -> Result<bool, String> {
    Ok(false)
}
#[cfg(not(target_os = "windows"))]
pub fn set_autostart(_enabled: bool) -> Result<bool, String> {
    Err("[autostart] 仅 Windows 支持".to_string())
}
