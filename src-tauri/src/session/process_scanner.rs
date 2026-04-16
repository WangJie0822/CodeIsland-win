use log::{debug, warn};

#[derive(Debug, Clone)]
pub struct ClaudeProcess {
    pub pid: u32,
    pub cwd: Option<String>,
    pub parent_pid: Option<u32>,
    pub terminal_type: Option<String>,
}

pub fn scan_claude_processes() -> Vec<ClaudeProcess> {
    #[cfg(target_os = "windows")]
    return scan_windows();
    #[cfg(not(target_os = "windows"))]
    return scan_unix();
}

pub fn is_process_alive(pid: u32) -> bool {
    #[cfg(target_os = "windows")]
    {
        // Win32 OpenProcess check
        false // stub on macOS build
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::path::Path::new(&format!("/proc/{}", pid)).exists()
            || std::process::Command::new("kill")
                .args(["-0", &pid.to_string()])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
    }
}

#[cfg(target_os = "windows")]
fn scan_windows() -> Vec<ClaudeProcess> {
    // Win32 CreateToolhelp32Snapshot implementation
    // Only compiles on Windows
    Vec::new()
}

#[cfg(not(target_os = "windows"))]
fn scan_unix() -> Vec<ClaudeProcess> {
    let output = std::process::Command::new("ps")
        .args(["-Ax", "-o", "pid,command"])
        .output()
        .ok();

    let mut results = Vec::new();
    if let Some(output) = output {
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            if line.contains("claude") && !line.contains("grep") {
                let parts: Vec<&str> = line.trim().splitn(2, ' ').collect();
                if let Some(pid_str) = parts.first() {
                    if let Ok(pid) = pid_str.trim().parse::<u32>() {
                        results.push(ClaudeProcess {
                            pid,
                            cwd: None,
                            parent_pid: None,
                            terminal_type: None,
                        });
                    }
                }
            }
        }
    }
    results
}
