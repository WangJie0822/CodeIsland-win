use log::info;

pub fn send_text_to_terminal(text: &str, pid: u32) -> bool {
    #[cfg(target_os = "windows")]
    {
        // Win32 clipboard + SendInput - only compiles on Windows
        false
    }
    #[cfg(not(target_os = "windows"))]
    {
        info!("[TerminalWriter] macOS stub: 模拟发送 '{}' 到 PID {}", text, pid);
        false
    }
}
