/// Windows 登录自启动管理。
///
/// # TODO (Stage 2)
///
/// 当前为占位实现，仅返回 `Ok(())` / `Ok(false)`。
///
/// 真实实现需要在 `Cargo.toml` 中启用以下任一依赖之后替换：
///   - `winreg = "0.52"` crate（推荐，API 简洁）
///   - 或为 `windows` crate 启用 `Win32_System_Registry` feature
///
/// 真实实现应读写注册表键：
///   `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`
///   值名：`CodeIsland`，值：可执行文件路径（含参数）

/// 注册表键名。
const REG_VALUE_NAME: &str = "CodeIsland";

/// 查询当前用户是否已配置登录自启动。
///
/// # Returns
/// `Ok(true)` 若已启用，`Ok(false)` 若未启用。
///
/// # Errors
/// 格式 `"[registry] <原因>"`。
pub fn get_autostart() -> Result<bool, String> {
    // TODO (Stage 2): 替换为真实 winreg 读取
    // 示例（winreg crate）：
    // ```
    // use winreg::enums::*;
    // use winreg::RegKey;
    // let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    // let run = hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
    //     .map_err(|e| format!("[registry] 打开注册表键失败: {}", e))?;
    // let val: String = run.get_value(REG_VALUE_NAME)
    //     .unwrap_or_default();
    // Ok(!val.is_empty())
    // ```
    log::debug!(
        "[autostart] get_autostart 占位实现（REG_VALUE_NAME={}），返回 false",
        REG_VALUE_NAME
    );
    Ok(false)
}

/// 设置或清除登录自启动注册表项。
///
/// - `enabled = true`：写入当前可执行文件路径到注册表
/// - `enabled = false`：删除注册表值
///
/// # Returns
/// `Ok(enabled)`，反映操作后的状态。
///
/// # Errors
/// 格式 `"[registry] <原因>"`。
pub fn set_autostart(enabled: bool) -> Result<bool, String> {
    // TODO (Stage 2): 替换为真实 winreg 写入/删除
    // 示例（winreg crate）：
    // ```
    // use winreg::enums::*;
    // use winreg::RegKey;
    // let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    // let (run, _) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
    //     .map_err(|e| format!("[registry] 创建注册表键失败: {}", e))?;
    // if enabled {
    //     let exe = std::env::current_exe()
    //         .map_err(|e| format!("[registry] 获取可执行路径失败: {}", e))?;
    //     run.set_value(REG_VALUE_NAME, &exe.to_string_lossy().as_ref())
    //         .map_err(|e| format!("[registry] 写入注册表失败: {}", e))?;
    // } else {
    //     run.delete_value(REG_VALUE_NAME)
    //         .map_err(|e| format!("[registry] 删除注册表值失败: {}", e))?;
    // }
    // Ok(enabled)
    // ```
    log::debug!(
        "[autostart] set_autostart({}) 占位实现，实际未写入注册表",
        enabled
    );
    Ok(enabled)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 占位测试：验证函数签名和错误格式正确（不依赖真实注册表）。
    /// Stage 2 替换真实实现后，需补充集成测试。
    #[test]
    fn get_autostart_placeholder_returns_false() {
        let result = get_autostart();
        // 占位实现应成功返回 false
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn set_autostart_placeholder_echoes_input() {
        assert_eq!(set_autostart(true), Ok(true));
        assert_eq!(set_autostart(false), Ok(false));
    }
}
