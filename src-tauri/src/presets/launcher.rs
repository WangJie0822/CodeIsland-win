use crate::presets::types::LaunchPreset;

/// 终端启动器抽象 trait，便于单测时 mock，不实际执行进程。
pub trait CommandRunner: Send + Sync {
    /// 执行命令，返回是否成功启动（注意：spawn 成功不代表进程存活）。
    fn run(&self, program: &str, args: &[&str]) -> Result<(), String>;
}

/// 真实的进程启动器，通过 `std::process::Command` 执行。
pub struct RealCommandRunner;

impl CommandRunner for RealCommandRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<(), String> {
        std::process::Command::new(program)
            .args(args)
            .spawn()
            .map(|_| ())
            .map_err(|e| format!("[preset_launch] 启动进程 '{program}' 失败: {e}"))
    }
}

/// 探测可执行文件是否在 PATH 中（使用 `where`，Windows 专用）。
fn which_windows(exe: &str) -> bool {
    std::process::Command::new("where")
        .arg(exe)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// 探测 Claude CLI 的路径。
/// 优先 `where claude`，fallback `%APPDATA%\npm\claude.cmd`。
fn find_claude_cli() -> Option<String> {
    if which_windows("claude") {
        return Some("claude".to_string());
    }
    // fallback：npm 全局安装路径
    if let Some(appdata) = std::env::var_os("APPDATA") {
        let fallback = std::path::Path::new(&appdata)
            .join("npm")
            .join("claude.cmd");
        if fallback.exists() {
            return Some(fallback.to_string_lossy().to_string());
        }
    }
    None
}

/// 组装启动命令参数，返回 `(program, args)` 元组。
///
/// 终端优先级：`wt.exe` → `pwsh.exe` → `cmd.exe`
pub fn build_launch_command(
    preset: &LaunchPreset,
    claude_cli: &str,
) -> (String, Vec<String>) {
    // 组装 Claude CLI 调用参数
    let mut claude_args: Vec<String> = vec![];
    if let Some(model) = &preset.model {
        if model != "inherit" {
            claude_args.push("--model".to_string());
            claude_args.push(model.clone());
        }
    }
    if let Some(agent) = &preset.agent {
        if !agent.is_empty() {
            claude_args.push("--agent".to_string());
            claude_args.push(agent.clone());
        }
    }
    for server in &preset.mcp_servers {
        if !server.is_empty() {
            claude_args.push("--mcp-server".to_string());
            claude_args.push(server.clone());
        }
    }
    if let Some(prompt) = &preset.initial_prompt {
        if !prompt.is_empty() {
            // 初始提示词作为最后的位置参数传递
            claude_args.push(prompt.clone());
        }
    }

    let claude_cmd = if claude_args.is_empty() {
        claude_cli.to_string()
    } else {
        format!("{} {}", claude_cli, claude_args.join(" "))
    };

    let ps_command = format!("Set-Location '{}'; {}", preset.cwd, claude_cmd);

    if which_windows("wt") {
        // Windows Terminal：在指定目录中打开 PowerShell
        let args = vec![
            "-d".to_string(),
            preset.cwd.clone(),
            "pwsh.exe".to_string(),
            "-NoExit".to_string(),
            "-Command".to_string(),
            claude_cmd.clone(),
        ];
        ("wt.exe".to_string(), args)
    } else if which_windows("pwsh") {
        let args = vec![
            "-NoExit".to_string(),
            "-Command".to_string(),
            ps_command,
        ];
        ("pwsh.exe".to_string(), args)
    } else {
        // fallback: cmd.exe（不支持 PowerShell 参数，仅尝试直接 start）
        let args = vec![
            "/c".to_string(),
            "start".to_string(),
            "cmd.exe".to_string(),
            "/k".to_string(),
            format!("cd /d \"{}\" && {}", preset.cwd, claude_cmd),
        ];
        ("cmd.exe".to_string(), args)
    }
}

/// 启动一个 preset 所描述的 Claude CLI 会话。
pub fn launch_preset_with_runner(
    preset: &LaunchPreset,
    runner: &dyn CommandRunner,
) -> Result<(), String> {
    let claude_cli = find_claude_cli()
        .ok_or_else(|| "[preset_launch] 未找到 Claude CLI（where claude 失败且 fallback 路径不存在）".to_string())?;

    let (program, args) = build_launch_command(preset, &claude_cli);
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    runner.run(&program, &args_ref)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    /// Mock runner：记录被调用的命令，不实际执行。
    struct MockRunner {
        calls: Arc<Mutex<Vec<(String, Vec<String>)>>>,
        should_fail: bool,
    }

    impl MockRunner {
        fn new() -> Self {
            MockRunner {
                calls: Arc::new(Mutex::new(vec![])),
                should_fail: false,
            }
        }

        fn calls(&self) -> Vec<(String, Vec<String>)> {
            self.calls.lock().unwrap().clone()
        }
    }

    impl CommandRunner for MockRunner {
        fn run(&self, program: &str, args: &[&str]) -> Result<(), String> {
            if self.should_fail {
                return Err("mock failure".to_string());
            }
            self.calls.lock().unwrap().push((
                program.to_string(),
                args.iter().map(|s| s.to_string()).collect(),
            ));
            Ok(())
        }
    }

    fn make_preset() -> LaunchPreset {
        LaunchPreset {
            id: "p1".to_string(),
            name: "Test Preset".to_string(),
            icon: "🚀".to_string(),
            cwd: "C:\\projects\\test".to_string(),
            initial_prompt: None,
            model: Some("claude-sonnet-4-5".to_string()),
            agent: None,
            mcp_servers: vec![],
            sort_order: 0,
        }
    }

    #[test]
    fn build_command_includes_model() {
        let preset = make_preset();
        let (program, args) = build_launch_command(&preset, "claude");
        // program 是终端（wt/pwsh/cmd），不能断言具体值（取决于测试机环境）
        // 断言 args 中包含 model 参数
        let args_str = args.join(" ");
        assert!(
            args_str.contains("claude-sonnet-4-5"),
            "args 应包含 model 名称，实际 args = {args_str:?}"
        );
        // program 必须是支持的终端之一
        assert!(
            program == "wt.exe" || program == "pwsh.exe" || program == "cmd.exe",
            "program = {program}"
        );
    }

    #[test]
    fn build_command_inherit_model_omits_model_flag() {
        let mut preset = make_preset();
        preset.model = Some("inherit".to_string());
        let (_, args) = build_launch_command(&preset, "claude");
        let args_str = args.join(" ");
        assert!(
            !args_str.contains("--model"),
            "inherit 时不应包含 --model，实际 args = {args_str:?}"
        );
    }

    #[test]
    fn build_command_with_initial_prompt() {
        let mut preset = make_preset();
        preset.initial_prompt = Some("请帮我写单测".to_string());
        let (_, args) = build_launch_command(&preset, "claude");
        let args_str = args.join(" ");
        assert!(
            args_str.contains("请帮我写单测"),
            "args 应包含初始提示词，实际 args = {args_str:?}"
        );
    }

    #[test]
    fn build_command_with_mcp_servers() {
        let mut preset = make_preset();
        preset.mcp_servers = vec!["server-a".to_string(), "server-b".to_string()];
        let (_, args) = build_launch_command(&preset, "claude");
        let args_str = args.join(" ");
        assert!(
            args_str.contains("server-a") && args_str.contains("server-b"),
            "args 应包含 mcp-server 参数，实际 args = {args_str:?}"
        );
    }

    #[test]
    fn build_command_none_model_no_model_flag() {
        let mut preset = make_preset();
        preset.model = None;
        let (_, args) = build_launch_command(&preset, "claude");
        let args_str = args.join(" ");
        assert!(
            !args_str.contains("--model"),
            "model=None 时不应有 --model，实际 args = {args_str:?}"
        );
    }
}
