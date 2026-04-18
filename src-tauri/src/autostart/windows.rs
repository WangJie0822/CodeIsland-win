//! autostart — 通过 HKCU\Software\Microsoft\Windows\CurrentVersion\Run 实现开机启动。
//!
//! 使用 `windows` crate 的 `Win32_System_Registry` feature（无需新增 crate）。
//!
//! 公开 API 硬编码 VALUE_NAME = "CodeIsland"；内部 inner 版本接受 value_name 参数，
//! 便于单测使用临时键名做 roundtrip。

use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Foundation::ERROR_FILE_NOT_FOUND;
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW,
    RegSetValueExW, HKEY, HKEY_CURRENT_USER, KEY_READ, KEY_WRITE, REG_OPTION_NON_VOLATILE,
    REG_SZ,
};

const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const VALUE_NAME: &str = "CodeIsland";

pub fn get_autostart() -> Result<bool, String> {
    get_autostart_inner(VALUE_NAME)
}

pub fn set_autostart(enabled: bool) -> Result<bool, String> {
    set_autostart_inner(VALUE_NAME, enabled)
}

/// 读取 HKCU Run 子键中某 value_name 是否存在。不存在时返回 Ok(false)。
pub(crate) fn get_autostart_inner(value_name: &str) -> Result<bool, String> {
    unsafe {
        let subkey = HSTRING::from(RUN_KEY);
        let mut hkey = HKEY::default();
        let open_status = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR::from_raw(subkey.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        if open_status.is_err() {
            if open_status.0 as u32 == ERROR_FILE_NOT_FOUND.0 {
                return Ok(false);
            }
            return Err(format!("[autostart] RegOpenKeyExW: {:?}", open_status));
        }

        let value_hstr = HSTRING::from(value_name);
        let query_status = RegQueryValueExW(
            hkey,
            PCWSTR::from_raw(value_hstr.as_ptr()),
            None,
            None,
            None,
            None,
        );
        let _ = RegCloseKey(hkey);
        Ok(query_status.is_ok())
    }
}

/// 写入或删除 HKCU Run 子键的某 value_name。enabled=true 写 exe 绝对路径；false 删除。
pub(crate) fn set_autostart_inner(value_name: &str, enabled: bool) -> Result<bool, String> {
    let exe = std::env::current_exe()
        .map_err(|e| format!("[autostart] current_exe: {}", e))?;
    let exe_str = exe
        .to_str()
        .ok_or_else(|| "[autostart] exe 路径非 UTF-8".to_string())?;

    unsafe {
        let subkey = HSTRING::from(RUN_KEY);
        let mut hkey = HKEY::default();
        let create_status = RegCreateKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR::from_raw(subkey.as_ptr()),
            0,
            PCWSTR::null(),
            REG_OPTION_NON_VOLATILE,
            KEY_READ | KEY_WRITE,
            None,
            &mut hkey,
            None,
        );
        if create_status.is_err() {
            return Err(format!("[autostart] RegCreateKeyExW: {:?}", create_status));
        }

        let value_hstr = HSTRING::from(value_name);
        let result = if enabled {
            // REG_SZ 写 UTF-16 字符串 + null terminator
            let data_hstr = HSTRING::from(exe_str);
            let data_u16 = data_hstr.as_wide();
            // UTF-16 字节数 = (len + 1) * 2（含 null terminator）
            let byte_count = (data_u16.len() + 1) * 2;
            let byte_ptr = data_u16.as_ptr() as *const u8;
            // SAFETY: HSTRING::as_wide 返回的切片生命周期由 data_hstr 持有，此 unsafe 块内成立
            let data_bytes: &[u8] = std::slice::from_raw_parts(byte_ptr, byte_count);

            let set_status = RegSetValueExW(
                hkey,
                PCWSTR::from_raw(value_hstr.as_ptr()),
                0,
                REG_SZ,
                Some(data_bytes),
            );
            if set_status.is_ok() {
                Ok(true)
            } else {
                Err(format!("[autostart] RegSetValueExW: {:?}", set_status))
            }
        } else {
            let status = RegDeleteValueW(hkey, PCWSTR::from_raw(value_hstr.as_ptr()));
            if status.is_ok() || status.0 as u32 == ERROR_FILE_NOT_FOUND.0 {
                Ok(false)
            } else {
                Err(format!("[autostart] RegDeleteValueW: {:?}", status))
            }
        };

        let _ = RegCloseKey(hkey);
        result
    }
}

#[cfg(all(test, target_os = "windows"))]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_set_get_delete() {
        let pid = std::process::id();
        let test_value_name = format!("CodeIsland-test-{}", pid);

        // 初始：不存在
        let initial = get_autostart_inner(&test_value_name).expect("get before set");
        assert!(!initial, "test value should not exist before set");

        // 写入
        set_autostart_inner(&test_value_name, true).expect("set true");
        let after_set = get_autostart_inner(&test_value_name).expect("get after set");
        assert!(after_set, "test value should exist after set true");

        // 删除
        set_autostart_inner(&test_value_name, false).expect("set false");
        let after_del = get_autostart_inner(&test_value_name).expect("get after del");
        assert!(!after_del, "test value should not exist after set false");

        // 幂等删除
        set_autostart_inner(&test_value_name, false).expect("set false again (idempotent)");
    }
}
