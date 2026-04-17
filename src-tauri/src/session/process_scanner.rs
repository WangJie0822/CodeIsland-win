use log::warn;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};

#[derive(Debug, Clone)]
pub struct ClaudeProcess {
    pub pid: u32,
    pub cwd: Option<String>,
    pub parent_pid: Option<u32>,
    pub terminal_type: Option<String>,
}

/// Stage 0 保留空实现；Stage 1 `buddy` / `usage` 模块如需扫描 claude 进程，
/// 在 Win32 ToolHelp32Snapshot 基础上扩展，详见 spec §10.1。
pub fn scan_claude_processes() -> Vec<ClaudeProcess> {
    Vec::new()
}

pub fn is_process_alive(pid: u32) -> bool {
    unsafe {
        match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) if !handle.is_invalid() => {
                if let Err(e) = CloseHandle(handle) {
                    warn!("[ProcessScanner] CloseHandle 失败: {:?}", e);
                }
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_process_alive() {
        let pid = std::process::id();
        assert!(is_process_alive(pid));
    }

    #[test]
    fn test_nonexistent_process_not_alive() {
        assert!(!is_process_alive(999_999_999));
    }

    #[test]
    fn test_process_scanner_is_windows_only() {
        // 避免 include_str! 自引用——把 needle 拆成两段字面量，编译期合并、源码搜索不命中
        let src = include_str!("process_scanner.rs");
        let forbidden_fn = concat!("scan", "_unix");
        let cross_cfg = concat!("cfg(not(", "target_os");
        let proc_path = concat!("/pr", "oc/");
        assert!(!src.contains(forbidden_fn), "unix scanner 残留");
        assert!(!src.contains(cross_cfg), "跨平台 cfg 残留");
        assert!(!src.contains(proc_path), "/proc 路径残留");
    }
}
