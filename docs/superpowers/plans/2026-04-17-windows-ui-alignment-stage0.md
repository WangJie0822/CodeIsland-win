# Code Island Windows UI 对齐 · Stage 0 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Stage 0 完成 Windows 专属基建——清理非 Windows IPC/进程/脚本分支，引入 Vue 3 + Pinia + Vite 前端框架，打通 `specta` + `tauri-specta` 前后端类型链，实现虚拟刘海 SVG 外形、窗口尺寸/位置/鼠标穿透命令、AppEvent 事件总线，冻结 Stage 1 并行契约。

**Architecture:** 后端继续使用 Tauri 2 + tokio + Named Pipe，但彻底移除 Unix 分支；前端从裸 DOM 升级为 Vue 3 `<script setup>` + Pinia store + Vue Router；TypeScript 类型由 Rust struct 经 `specta` 自动导出到 `src/types/generated.ts`，禁止手写 domain 类型；虚拟刘海外形用 SVG path + 参数插值动画实现，配合 Tauri `setIgnoreCursorEvents` 根据点是否落在 path 内动态切换鼠标穿透。

**Tech Stack:** Rust 1.80+ / Tauri 2 / tokio / Windows Named Pipe / Vue 3.5 / Pinia 2.2 / Vue Router 4 / Vite 6 / Vitest 2 / @vue/test-utils 2 / jsdom 26 / `specta` 2 / `specta-typescript` 0.0.9 / `tauri-specta` 2 / `thiserror` 1

---

## 前置说明（读 Task 1 之前必读）

**执行环境：**
- Stage 0 由主会话串行完成，不需要 worktree 拆分；所有修改在当前仓库根 `/Users/wj/Work/OpenSource/CodeIsland-win/` 内进行。
- Rust 变更（Task 1 / 2 / 5 / 8 / 9）执行 `cargo test` / `cargo build` 需在 **Windows 10+ 本机**或 GitHub Actions `build-windows.yml` runner 上验证。macOS 开发者在本地可用 `cargo check --target x86_64-pc-windows-msvc`（先 `rustup target add x86_64-pc-windows-msvc`）做 syntax-only 校验；实际测试以 Windows 为准。
- 前端变更（Task 4 / 5 / 6 / 7）使用 `npm run test`（Vitest）与 `npm run build`（tsc + vite build），任意平台均可。
- Python 变更（Task 3）使用 `python -m pytest`，需本机安装 Python 3.11+ 与 `pytest`。
- 文档变更（Task 10 / 11）纯 Markdown 编辑。

**提交规范（每个 Task 末尾 Step）：**
- 格式：`[类型|模块|功能][公共]中文说明`
- 严禁 `--amend` / `--no-verify` / Co-Authored-By 行。
- 每个 Task 一笔 commit，原子化；不夹带无关改动。

**文件所有权：**
- Stage 0 串行，主会话独占全部文件；Stage 1 才进入多 agent 并行，由 Task 10 的 contracts 文档冻结所有权边界。

**File Structure 总览（Stage 0 涉及的全部文件）：**

修改：
- `CLAUDE.md` — Task 11
- `index.html` — Task 6
- `package.json` — Task 4 / 6
- `tsconfig.json` — Task 4
- `src-tauri/Cargo.toml` — Task 1 / 5 / 9
- `src-tauri/src/main.rs` — Task 1 / 5 / 8 / 9
- `src-tauri/src/app_state.rs` — Task 9
- `src-tauri/src/commands/mod.rs` — Task 5 / 8
- `src-tauri/src/commands/approval.rs` — Task 9
- `src-tauri/src/commands/session.rs` — Task 5
- `src-tauri/src/commands/settings.rs` — Task 5
- `src-tauri/src/hook/pipe_server.rs` — Task 1 / 9
- `src-tauri/src/hook/installer.rs` — Task 2
- `src-tauri/src/hook/protocol.rs` — Task 5
- `src-tauri/src/session/process_scanner.rs` — Task 1
- `src-tauri/src/session/store.rs` — Task 5
- `src-tauri/src/session/phase.rs` — Task 5
- `src-tauri/resources/codeisland-state.py` — Task 3
- `src/styles/theme.css` — Task 6 / 7
- `src/styles/island.css` — Task 6 / 7

新建：
- `vite.config.ts` — Task 4
- `vitest.config.ts` — Task 4
- `src/App.vue` — Task 4
- `src/vitest-setup.ts` — Task 4
- `src/App.spec.ts` — Task 4
- `src/stores/sessions.ts` — Task 6
- `src/stores/notch.ts` — Task 7
- `src/views/NotchView.vue` — Task 6
- `src/components/notch/NotchShape.vue` — Task 7
- `src/components/session-card/SessionCard.vue` — Task 6
- `src/components/session-card/ApprovalButtons.vue` — Task 6
- `src/components/session-card/AskUserOptions.vue` — Task 6
- `src/components/common/StatusDot.vue` — Task 6
- `src/components/session-card/SessionCard.spec.ts` — Task 6
- `src/stores/sessions.spec.ts` — Task 6
- `src/utils/notch-shape.ts` — Task 7
- `src/utils/notch-shape.spec.ts` — Task 7
- `src/lib/tauri.ts` — Task 6
- `src/types/generated.ts` — Task 5（自动生成）
- `src-tauri/src/commands/window_control.rs` — Task 8
- `src-tauri/resources/test_codeisland_state.py` — Task 3
- `docs/superpowers/specs/2026-04-17-windows-ui-alignment-contracts.md` — Task 10

删除：
- `scripts/dev.sh` — Task 11
- `src/store.ts` — Task 6
- `src/lib/events.ts` — Task 6
- `src/components/Island.ts` — Task 6
- `src/components/SessionCard.ts` — Task 6
- `src/components/ApprovalButtons.ts` — Task 6
- `src/components/AskUserOptions.ts` — Task 6
- `src/components/StatusIndicator.ts` — Task 6

---

## Task 1: 清理非 Windows IPC 分支

**Files:**
- Modify: `src-tauri/src/hook/pipe_server.rs`
- Modify: `src-tauri/src/session/process_scanner.rs`
- Modify: `src-tauri/Cargo.toml`

- [ ] **Step 1: 写失败测试**

在 `src-tauri/src/hook/pipe_server.rs` 文件末尾追加 `tests` 模块（保留文件其余内容暂不动）：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_name_constant() {
        assert_eq!(PIPE_NAME, r"\\.\pipe\codeisland");
    }

    #[test]
    fn test_pipe_server_is_windows_only() {
        let src = include_str!("pipe_server.rs");
        assert!(!src.contains("UnixListener"), "Unix 分支残留 UnixListener");
        assert!(!src.contains("cfg(not(target_os"), "跨平台 cfg 残留");
        assert!(!src.contains("/tmp/codeisland.sock"), "Unix socket 路径残留");
    }
}
```

在 `src-tauri/src/session/process_scanner.rs` 文件末尾追加：

```rust
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
        let src = include_str!("process_scanner.rs");
        assert!(!src.contains("scan_unix"), "scan_unix 残留");
        assert!(!src.contains("cfg(not(target_os"), "跨平台 cfg 残留");
        assert!(!src.contains("/proc/"), "/proc 路径残留");
    }
}
```

- [ ] **Step 2: 运行测试验证失败**

Run: `cargo test --manifest-path src-tauri/Cargo.toml hook::pipe_server::tests process_scanner::tests`

预期：编译失败，报 `cannot find value PIPE_NAME`；或若常量已加但源文件仍含 Unix 分支，则 `test_pipe_server_is_windows_only` / `test_process_scanner_is_windows_only` FAIL，`test_current_process_alive` FAIL（当前 `is_process_alive` 在 Windows 分支返回硬编码 false）。

- [ ] **Step 3: 实现 — 覆盖 `pipe_server.rs`**

将 `src-tauri/src/hook/pipe_server.rs` 的完整内容替换为：

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::windows::named_pipe::{NamedPipeServer, ServerOptions};
use log::{info, warn, error};
use crate::hook::protocol::{HookEvent, HookResponse};
use crate::session::store::SessionStore;

pub const PIPE_NAME: &str = r"\\.\pipe\codeisland";

pub struct HookServer {
    store: Arc<Mutex<SessionStore>>,
    pending: Arc<Mutex<std::collections::HashMap<String, tokio::sync::oneshot::Sender<HookResponse>>>>,
    event_tx: tokio::sync::broadcast::Sender<String>,
}

impl HookServer {
    pub fn new(
        store: Arc<Mutex<SessionStore>>,
        event_tx: tokio::sync::broadcast::Sender<String>,
    ) -> Self {
        Self {
            store,
            pending: Arc::new(Mutex::new(std::collections::HashMap::new())),
            event_tx,
        }
    }

    pub async fn respond(&self, session_id: &str, response: HookResponse) -> bool {
        let mut pending = self.pending.lock().await;
        if let Some(tx) = pending.remove(session_id) {
            let _ = tx.send(response);
            true
        } else {
            warn!("[PipeServer] 无待审批连接: {}", session_id);
            false
        }
    }

    pub async fn start(&self) {
        info!("[PipeServer] 启动 Named Pipe: {}", PIPE_NAME);

        loop {
            let server = match ServerOptions::new()
                .first_pipe_instance(false)
                .create(PIPE_NAME)
            {
                Ok(s) => s,
                Err(e) => {
                    error!("[PipeServer] 创建 pipe 失败: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    continue;
                }
            };

            if let Err(e) = server.connect().await {
                error!("[PipeServer] 等待连接失败: {}", e);
                continue;
            }

            let store = self.store.clone();
            let pending = self.pending.clone();
            let event_tx = self.event_tx.clone();

            tokio::spawn(async move {
                handle_connection(server, store, pending, event_tx).await;
            });
        }
    }
}

async fn handle_connection(
    mut stream: NamedPipeServer,
    store: Arc<Mutex<SessionStore>>,
    pending: Arc<Mutex<std::collections::HashMap<String, tokio::sync::oneshot::Sender<HookResponse>>>>,
    event_tx: tokio::sync::broadcast::Sender<String>,
) {
    let mut buf = vec![0u8; 65536];
    let n = match stream.read(&mut buf).await {
        Ok(0) => return,
        Ok(n) => n,
        Err(e) => {
            warn!("[PipeServer] 读取失败: {}", e);
            return;
        }
    };

    let event: HookEvent = match serde_json::from_slice(&buf[..n]) {
        Ok(e) => e,
        Err(e) => {
            warn!("[PipeServer] JSON 解析失败: {}", e);
            return;
        }
    };

    let session_id = event.session_id.clone();
    let expects_response = event.expects_response();

    {
        let mut store = store.lock().await;
        store.process_hook_event(&event);
    }

    let _ = event_tx.send(session_id.clone());

    if expects_response {
        let (tx, rx) = tokio::sync::oneshot::channel::<HookResponse>();
        {
            let mut p = pending.lock().await;
            p.insert(session_id.clone(), tx);
        }

        info!("[PipeServer] 等待用户审批: {}", &session_id[..8.min(session_id.len())]);

        match tokio::time::timeout(std::time::Duration::from_secs(300), rx).await {
            Ok(Ok(response)) => {
                let json = serde_json::to_vec(&response).unwrap_or_default();
                let _ = stream.write_all(&json).await;
                info!(
                    "[PipeServer] 已回复审批: {} → {}",
                    &session_id[..8.min(session_id.len())],
                    response.decision
                );
            }
            _ => {
                warn!("[PipeServer] 审批超时: {}", &session_id[..8.min(session_id.len())]);
                let mut p = pending.lock().await;
                p.remove(&session_id);
            }
        }
    } else {
        let _ = stream.write_all(b"ok\n").await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_name_constant() {
        assert_eq!(PIPE_NAME, r"\\.\pipe\codeisland");
    }

    #[test]
    fn test_pipe_server_is_windows_only() {
        let src = include_str!("pipe_server.rs");
        assert!(!src.contains("UnixListener"), "Unix 分支残留 UnixListener");
        assert!(!src.contains("cfg(not(target_os"), "跨平台 cfg 残留");
        assert!(!src.contains("/tmp/codeisland.sock"), "Unix socket 路径残留");
    }
}
```

将 `src-tauri/src/session/process_scanner.rs` 的完整内容替换为：

```rust
use log::warn;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
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
        let src = include_str!("process_scanner.rs");
        assert!(!src.contains("scan_unix"), "scan_unix 残留");
        assert!(!src.contains("cfg(not(target_os"), "跨平台 cfg 残留");
        assert!(!src.contains("/proc/"), "/proc 路径残留");
    }
}
```

将 `src-tauri/Cargo.toml` 整文件替换为（把 `[target.'cfg(windows)']` 合入 `[dependencies]`，保留 `windows` crate 为普通依赖；同时提前加入 `thiserror`，Task 9 会用）：

```toml
[package]
name = "codeisland-win"
version = "0.1.0"
edition = "2021"

[dependencies]
tauri = { version = "2", features = ["tray-icon", "image-png"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
notify = "7"
rodio = "0.19"
log = "0.4"
env_logger = "0.11"
dirs = "6"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1"
windows = { version = "0.58", features = [
    "Win32_Foundation",
    "Win32_System_Threading",
    "Win32_System_Diagnostics_ToolHelp",
    "Win32_UI_Input_KeyboardAndMouse",
    "Win32_UI_WindowsAndMessaging",
    "Win32_System_DataExchange",
    "Win32_System_Console",
] }

[build-dependencies]
tauri-build = { version = "2", features = [] }
```

- [ ] **Step 4: 运行测试验证通过**

Run（Windows 环境）: `cargo test --manifest-path src-tauri/Cargo.toml`

预期：所有既有测试（`session::phase::tests::*`、`hook::protocol::tests::*`）保持 PASS；新增测试通过：
- `hook::pipe_server::tests::test_pipe_name_constant` PASS
- `hook::pipe_server::tests::test_pipe_server_is_windows_only` PASS
- `session::process_scanner::tests::test_current_process_alive` PASS
- `session::process_scanner::tests::test_nonexistent_process_not_alive` PASS
- `session::process_scanner::tests::test_process_scanner_is_windows_only` PASS

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/hook/pipe_server.rs src-tauri/src/session/process_scanner.rs src-tauri/Cargo.toml src-tauri/Cargo.lock
git commit -m "[refactor|backend|pipe][公共]清理非 Windows IPC 分支"
```

---

## Task 2: hook 安装器路径简化为 Windows 专属

**Files:**
- Modify: `src-tauri/src/hook/installer.rs`

- [ ] **Step 1: 写失败测试**

在 `src-tauri/src/hook/installer.rs` 文件末尾追加：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_dir_on_windows() {
        let dir = claude_dir();
        assert!(dir.ends_with(".claude"), "期待 .claude 结尾: {:?}", dir);
        let s = dir.to_string_lossy();
        assert!(
            s.contains('\\') || s.contains(':'),
            "期待 Windows 路径分隔符: {}",
            s
        );
    }

    #[test]
    fn test_python_candidates_are_windows_only() {
        let candidates = python_candidates();
        assert_eq!(candidates, &["py", "python", "python3"]);
    }

    #[test]
    fn test_installer_is_windows_only() {
        let src = include_str!("installer.rs");
        assert!(!src.contains("cfg(unix)"), "cfg(unix) 残留");
        assert!(!src.contains("PermissionsExt"), "Unix 权限代码残留");
        assert!(!src.contains("cfg(not(target_os"), "跨平台 cfg 残留");
    }
}
```

- [ ] **Step 2: 运行测试验证失败**

Run: `cargo test --manifest-path src-tauri/Cargo.toml hook::installer::tests`

预期：编译失败 `cannot find function python_candidates`；或 `test_installer_is_windows_only` FAIL（文件仍有 `#[cfg(unix)]` 块与 `#[cfg(not(target_os = "windows"))]` 分支）。

- [ ] **Step 3: 实现 — 覆盖 `installer.rs`**

将 `src-tauri/src/hook/installer.rs` 的完整内容替换为：

```rust
use std::fs;
use std::path::PathBuf;
use log::{info, warn};

pub fn claude_dir() -> PathBuf {
    dirs::home_dir()
        .expect("无法获取 %USERPROFILE%")
        .join(".claude")
}

pub fn python_candidates() -> &'static [&'static str] {
    &["py", "python", "python3"]
}

pub fn install_if_needed(script_content: &[u8]) {
    let claude = claude_dir();
    let hooks_dir = claude.join("hooks");
    let script_path = hooks_dir.join("codeisland-state.py");
    let settings_path = claude.join("settings.json");

    if let Err(e) = fs::create_dir_all(&hooks_dir) {
        warn!("[Installer] 创建 hooks 目录失败: {}", e);
        return;
    }

    if let Err(e) = fs::write(&script_path, script_content) {
        warn!("[Installer] 写入 hook 脚本失败: {}", e);
        return;
    }

    info!("[Installer] Hook 脚本已安装: {:?}", script_path);
    update_settings(&settings_path);
}

fn update_settings(settings_path: &PathBuf) {
    let mut json: serde_json::Map<String, serde_json::Value> = if settings_path.exists() {
        let content = fs::read_to_string(settings_path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        serde_json::Map::new()
    };

    let python = detect_python();
    let home = dirs::home_dir().expect("无法获取 %USERPROFILE%");
    let script_path = home.join(".claude").join("hooks").join("codeisland-state.py");
    let command = format!("{} \"{}\"", python, script_path.display());

    let hook_entry = serde_json::json!([{"type": "command", "command": command}]);
    let hook_entry_timeout = serde_json::json!([{"type": "command", "command": command, "timeout": 86400}]);

    let with_matcher = |hooks: &serde_json::Value| serde_json::json!([{"matcher": "*", "hooks": hooks}]);
    let without_matcher = |hooks: &serde_json::Value| serde_json::json!([{"hooks": hooks}]);
    let precompact_config = serde_json::json!([
        {"matcher": "auto", "hooks": hook_entry},
        {"matcher": "manual", "hooks": hook_entry}
    ]);

    let hooks = json
        .entry("hooks")
        .or_insert_with(|| serde_json::json!({}))
        .as_object_mut()
        .expect("hooks 应是 object");

    let events: Vec<(&str, serde_json::Value)> = vec![
        ("UserPromptSubmit", without_matcher(&hook_entry)),
        ("PreToolUse", with_matcher(&hook_entry)),
        ("PostToolUse", with_matcher(&hook_entry)),
        ("PermissionRequest", with_matcher(&hook_entry_timeout)),
        ("Notification", with_matcher(&hook_entry)),
        ("Stop", without_matcher(&hook_entry)),
        ("SubagentStop", without_matcher(&hook_entry)),
        ("SessionStart", without_matcher(&hook_entry)),
        ("SessionEnd", without_matcher(&hook_entry)),
        ("PreCompact", precompact_config),
    ];

    for (event_name, config) in events {
        let has_our_hook = hooks
            .get(event_name)
            .and_then(|v| v.as_array())
            .map(|entries| {
                entries.iter().any(|entry| {
                    entry
                        .get("hooks")
                        .and_then(|h| h.as_array())
                        .map(|hooks| {
                            hooks.iter().any(|h| {
                                h.get("command")
                                    .and_then(|c| c.as_str())
                                    .map(|c| c.contains("codeisland-state.py"))
                                    .unwrap_or(false)
                            })
                        })
                        .unwrap_or(false)
                })
            })
            .unwrap_or(false);

        if !has_our_hook {
            if let Some(existing) = hooks.get_mut(event_name).and_then(|v| v.as_array_mut()) {
                if let Some(new_entries) = config.as_array() {
                    existing.extend(new_entries.clone());
                }
            } else {
                hooks.insert(event_name.to_string(), config);
            }
        }
    }

    let json_str = serde_json::to_string_pretty(&json).unwrap_or_default();
    if let Err(e) = fs::write(settings_path, json_str) {
        warn!("[Installer] 写入 settings.json 失败: {}", e);
    } else {
        info!("[Installer] settings.json 已更新");
    }
}

fn detect_python() -> String {
    for candidate in python_candidates() {
        if std::process::Command::new(candidate)
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            return (*candidate).to_string();
        }
    }
    "py".to_string()
}

pub fn uninstall() {
    let claude = claude_dir();
    let script = claude.join("hooks").join("codeisland-state.py");
    let settings_path = claude.join("settings.json");

    let _ = fs::remove_file(&script);

    if let Ok(content) = fs::read_to_string(&settings_path) {
        if let Ok(mut json) = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&content) {
            if let Some(hooks) = json.get_mut("hooks").and_then(|v| v.as_object_mut()) {
                for (_, value) in hooks.iter_mut() {
                    if let Some(entries) = value.as_array_mut() {
                        entries.retain(|entry| {
                            !entry
                                .get("hooks")
                                .and_then(|h| h.as_array())
                                .map(|hooks| {
                                    hooks.iter().any(|h| {
                                        h.get("command")
                                            .and_then(|c| c.as_str())
                                            .map(|c| c.contains("codeisland-state.py"))
                                            .unwrap_or(false)
                                    })
                                })
                                .unwrap_or(false)
                        });
                    }
                }
                hooks.retain(|_, v| v.as_array().map(|a| !a.is_empty()).unwrap_or(true));
            }
            if let Ok(json_str) = serde_json::to_string_pretty(&json) {
                let _ = fs::write(&settings_path, json_str);
            }
        }
    }
    info!("[Installer] Hook 已卸载");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_dir_on_windows() {
        let dir = claude_dir();
        assert!(dir.ends_with(".claude"), "期待 .claude 结尾: {:?}", dir);
        let s = dir.to_string_lossy();
        assert!(
            s.contains('\\') || s.contains(':'),
            "期待 Windows 路径分隔符: {}",
            s
        );
    }

    #[test]
    fn test_python_candidates_are_windows_only() {
        let candidates = python_candidates();
        assert_eq!(candidates, &["py", "python", "python3"]);
    }

    #[test]
    fn test_installer_is_windows_only() {
        let src = include_str!("installer.rs");
        assert!(!src.contains("cfg(unix)"), "cfg(unix) 残留");
        assert!(!src.contains("PermissionsExt"), "Unix 权限代码残留");
        assert!(!src.contains("cfg(not(target_os"), "跨平台 cfg 残留");
    }
}
```

- [ ] **Step 4: 运行测试验证通过**

Run: `cargo test --manifest-path src-tauri/Cargo.toml hook::installer::tests`

预期：3 个测试 PASS：`test_claude_dir_on_windows`、`test_python_candidates_are_windows_only`、`test_installer_is_windows_only`。

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/hook/installer.rs
git commit -m "[refactor|hook|installer][公共]路径简化为 Windows 专属"
```

---

## Task 3: hook 脚本 Windows 化

**Files:**
- Modify: `src-tauri/resources/codeisland-state.py`
- Create: `src-tauri/resources/test_codeisland_state.py`

- [ ] **Step 1: 写失败测试**

创建 `src-tauri/resources/test_codeisland_state.py`：

```python
"""codeisland-state.py 单元测试 — Windows 专属分支校验。"""
import io
import json
import sys
import types
from pathlib import Path
from unittest import mock

import pytest


SCRIPT = Path(__file__).with_name("codeisland-state.py")


def load_module():
    """以模块方式载入 hook 脚本（脚本文件名含连字符，需手工加载）。"""
    import importlib.util
    spec = importlib.util.spec_from_file_location("codeisland_state", SCRIPT)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def test_pipe_path_constant():
    module = load_module()
    assert module.PIPE_PATH == r"\\.\pipe\codeisland"


def test_no_unix_socket_references():
    source = SCRIPT.read_text(encoding="utf-8")
    assert "AF_UNIX" not in source, "Unix Socket 残留"
    assert "/tmp/codeisland.sock" not in source, "Unix socket 路径残留"
    assert "SOCKET_PATH" not in source, "Unix socket 常量残留"
    assert "_send_via_socket" not in source, "Unix socket 发送函数残留"
    assert "import socket" not in source, "socket 模块 import 残留"


def test_is_server_running_checks_pipe(monkeypatch):
    module = load_module()
    opens: list[tuple[str, str]] = []

    class FakePipe:
        def close(self):
            pass

    def fake_open(path, mode, buffering=0):
        opens.append((path, mode))
        return FakePipe()

    monkeypatch.setattr("builtins.open", fake_open)
    assert module.is_server_running() is True
    assert opens == [(r"\\.\pipe\codeisland", "r+b")]


def test_is_server_running_handles_oserror(monkeypatch):
    module = load_module()

    def fake_open(*args, **kwargs):
        raise OSError("pipe not available")

    monkeypatch.setattr("builtins.open", fake_open)
    assert module.is_server_running() is False


def test_main_exits_when_no_server(monkeypatch):
    module = load_module()
    monkeypatch.setattr(module, "is_server_running", lambda: False)
    monkeypatch.setattr(sys, "stdin", io.StringIO("{}"))
    with pytest.raises(SystemExit) as exc:
        module.main()
    assert exc.value.code == 0


def test_permission_request_allow_path(monkeypatch, capsys):
    module = load_module()
    monkeypatch.setattr(module, "is_server_running", lambda: True)

    sample = {
        "session_id": "sid-1",
        "cwd": "C:\\\\Users\\\\u\\\\proj",
        "hook_event_name": "PermissionRequest",
        "tool_name": "Bash",
        "tool_input": {"command": "dir"},
    }
    monkeypatch.setattr(sys, "stdin", io.StringIO(json.dumps(sample)))

    captured_state: dict = {}

    def fake_send(state):
        captured_state.update(state)
        return {"decision": "allow"}

    monkeypatch.setattr(module, "send_event", fake_send)

    with pytest.raises(SystemExit) as exc:
        module.main()
    assert exc.value.code == 0

    assert captured_state["status"] == "waiting_for_approval"
    assert captured_state["tool"] == "Bash"

    out = capsys.readouterr().out
    payload = json.loads(out)
    assert payload["hookSpecificOutput"]["decision"]["behavior"] == "allow"
```

- [ ] **Step 2: 运行测试验证失败**

Run: `python -m pytest src-tauri/resources/test_codeisland_state.py -v`

预期：
- `test_no_unix_socket_references` FAIL（当前脚本含 `AF_UNIX` / `SOCKET_PATH`）
- `test_is_server_running_checks_pipe` FAIL（当前函数在非 Windows 走 `os.path.exists(SOCKET_PATH)` 路径，执行 `open()` 前会先检查 `IS_WINDOWS`）

若 `pytest` 未安装：`pip install pytest` 后重试。

- [ ] **Step 3: 实现 — 覆盖 `codeisland-state.py`**

将 `src-tauri/resources/codeisland-state.py` 完整替换为：

```python
#!/usr/bin/env python3
"""Code Island Hook (Windows 专属)。

通过 Named Pipe `\\\\.\\pipe\\codeisland` 与 Tauri 后端通信；PermissionRequest 时
阻塞等待后端决策，其他事件只上报。"""
import json
import os
import sys

TIMEOUT_SECONDS = 300
PIPE_PATH = r"\\.\pipe\codeisland"


def is_server_running() -> bool:
    """探测 Code Island Named Pipe 是否可连接。"""
    try:
        handle = open(PIPE_PATH, "r+b", buffering=0)
        handle.close()
        return True
    except OSError:
        return False


def send_event(state):
    """向 Named Pipe 写入事件，PermissionRequest 场景同步等待后端回复。"""
    try:
        pipe = open(PIPE_PATH, "r+b", buffering=0)
        pipe.write(json.dumps(state).encode("utf-8"))
        pipe.flush()

        if state.get("status") == "waiting_for_approval":
            import time
            start = time.time()
            data = b""
            while time.time() - start < TIMEOUT_SECONDS:
                try:
                    chunk = pipe.read(4096)
                    if chunk:
                        data += chunk
                        break
                except Exception:
                    time.sleep(0.1)
                    continue
            pipe.close()
            if data:
                return json.loads(data.decode("utf-8"))
        else:
            try:
                pipe.read(64)
            except Exception:
                pass
            pipe.close()
        return None
    except (OSError, json.JSONDecodeError):
        return None


def main():
    if not is_server_running():
        sys.exit(0)

    try:
        data = json.load(sys.stdin)
    except json.JSONDecodeError:
        sys.exit(1)

    session_id = data.get("session_id", "unknown")
    event = data.get("hook_event_name", "")
    cwd = data.get("cwd", "")
    tool_input = data.get("tool_input", {})
    claude_pid = os.getppid()

    state = {
        "session_id": session_id,
        "cwd": cwd,
        "event": event,
        "pid": claude_pid,
        "tty": None,
    }

    if event == "UserPromptSubmit":
        state["status"] = "processing"

    elif event == "PreToolUse":
        state["status"] = "running_tool"
        state["tool"] = data.get("tool_name")
        state["tool_input"] = tool_input
        tool_use_id = data.get("tool_use_id")
        if tool_use_id:
            state["tool_use_id"] = tool_use_id

    elif event == "PostToolUse":
        state["status"] = "processing"
        state["tool"] = data.get("tool_name")
        state["tool_input"] = tool_input
        tool_use_id = data.get("tool_use_id")
        if tool_use_id:
            state["tool_use_id"] = tool_use_id

    elif event == "PermissionRequest":
        state["status"] = "waiting_for_approval"
        state["tool"] = data.get("tool_name")
        state["tool_input"] = tool_input

        response = send_event(state)

        if response:
            decision = response.get("decision", "ask")
            reason = response.get("reason", "")

            if decision == "allow":
                output = {
                    "hookSpecificOutput": {
                        "hookEventName": "PermissionRequest",
                        "decision": {"behavior": "allow"},
                    }
                }
                print(json.dumps(output))
                sys.exit(0)
            elif decision == "deny":
                output = {
                    "hookSpecificOutput": {
                        "hookEventName": "PermissionRequest",
                        "decision": {
                            "behavior": "deny",
                            "message": reason or "Denied by user via CodeIsland",
                        },
                    }
                }
                print(json.dumps(output))
                sys.exit(0)
        sys.exit(0)

    elif event == "Notification":
        notification_type = data.get("notification_type")
        if notification_type == "permission_prompt":
            sys.exit(0)
        elif notification_type == "idle_prompt":
            state["status"] = "waiting_for_input"
        else:
            state["status"] = "notification"
        state["notification_type"] = notification_type
        state["message"] = data.get("message")

    elif event == "Stop":
        state["status"] = "waiting_for_input"
    elif event == "SubagentStop":
        state["status"] = "waiting_for_input"
    elif event == "SessionStart":
        state["status"] = "waiting_for_input"
    elif event == "SessionEnd":
        state["status"] = "ended"
    elif event == "PreCompact":
        state["status"] = "compacting"
    else:
        state["status"] = "unknown"

    send_event(state)


if __name__ == "__main__":
    main()
```

- [ ] **Step 4: 运行测试验证通过**

Run: `python -m pytest src-tauri/resources/test_codeisland_state.py -v`

预期：6 个测试全部 PASS。

同时删除编译缓存（pyc 不在 git 版本控制里，但保险起见执行一次）：

```bash
rm -rf src-tauri/resources/__pycache__
```

- [ ] **Step 5: Commit**

```bash
git add src-tauri/resources/codeisland-state.py src-tauri/resources/test_codeisland_state.py
git commit -m "[refactor|script|python][公共]hook 脚本 Windows 化"
```

---

## Task 4: 前端引入 Vue 3 + Pinia 基建（不迁移现有组件）

**Files:**
- Modify: `package.json`
- Modify: `tsconfig.json`
- Create: `vite.config.ts`
- Create: `vitest.config.ts`
- Create: `src/App.vue`
- Create: `src/vitest-setup.ts`
- Create: `src/App.spec.ts`

> Task 4 只是 **基础设施准备**，不替换 `src/main.ts` 现有挂载入口，不改 `index.html`，保证 `npm run build` 与 dev 运行的 UI 行为不变。Task 6 才真正切换入口。

- [ ] **Step 1: 新增前端依赖**

将 `package.json` 完整替换为：

```json
{
  "name": "codeisland-win",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "vite",
    "build": "vue-tsc --noEmit && vite build",
    "tauri": "tauri",
    "test": "vitest run",
    "test:watch": "vitest"
  },
  "dependencies": {
    "@tauri-apps/api": "^2",
    "pinia": "^2.2.0",
    "vue": "^3.5.0",
    "vue-router": "^4.4.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2",
    "@vitejs/plugin-vue": "^5.1.0",
    "@vue/test-utils": "^2.4.6",
    "esbuild": "^0.28.0",
    "jsdom": "^26.0.0",
    "typescript": "^5.6",
    "vite": "^6",
    "vitest": "^2.1.0",
    "vue-tsc": "^2.1.0"
  }
}
```

执行安装：

```bash
npm install
```

- [ ] **Step 2: 创建 Vite / Vitest / TS 配置**

创建 `vite.config.ts`：

```ts
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from "node:path";

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "src"),
    },
  },
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
  },
});
```

创建 `vitest.config.ts`：

```ts
import { defineConfig, mergeConfig } from "vitest/config";
import viteConfig from "./vite.config";

export default mergeConfig(
  viteConfig,
  defineConfig({
    test: {
      environment: "jsdom",
      globals: true,
      setupFiles: ["src/vitest-setup.ts"],
      include: ["src/**/*.spec.ts"],
    },
  }),
);
```

替换 `tsconfig.json` 完整内容为：

```json
{
  "compilerOptions": {
    "target": "ES2021",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "strict": true,
    "noEmit": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "jsx": "preserve",
    "isolatedModules": true,
    "useDefineForClassFields": true,
    "outDir": "dist",
    "types": ["vitest/globals", "vite/client"],
    "paths": {
      "@/*": ["./src/*"]
    },
    "baseUrl": "."
  },
  "include": ["src/**/*.ts", "src/**/*.vue", "src/**/*.d.ts"]
}
```

- [ ] **Step 3: 创建 Vue 骨架文件与 smoke test**

创建 `src/App.vue`：

```vue
<script setup lang="ts">
// Stage 0 占位根组件；Task 6 会挂载 NotchView、Task 7 引入 SVG 刘海。
</script>

<template>
  <div class="app-root" data-testid="app-root">
    <slot />
  </div>
</template>

<style scoped>
.app-root {
  width: 100%;
  height: 100%;
}
</style>
```

创建 `src/vitest-setup.ts`：

```ts
// 组件测试全局 setup。
// Tauri 的 @tauri-apps/api 在 jsdom 下没有 __TAURI_INTERNALS__，
// 测试中需要 invoke 的地方由各 spec 自行 mock。
```

创建 `src/App.spec.ts`：

```ts
import { describe, expect, it } from "vitest";
import { mount } from "@vue/test-utils";
import App from "./App.vue";

describe("App.vue", () => {
  it("渲染根容器", () => {
    const wrapper = mount(App);
    expect(wrapper.get("[data-testid='app-root']").exists()).toBe(true);
  });
});
```

- [ ] **Step 4: 运行测试验证通过**

Run:

```bash
npm run test
```

预期：`App.vue > 渲染根容器` PASS，总计 1 passed。

Run:

```bash
npm run build
```

预期：vue-tsc 与 vite build 均 PASS（现有 `src/main.ts` / `src/store.ts` / `src/lib/events.ts` / `src/components/*.ts` 未变，行为不变）。

- [ ] **Step 5: Commit**

```bash
git add package.json package-lock.json tsconfig.json vite.config.ts vitest.config.ts src/App.vue src/vitest-setup.ts src/App.spec.ts
git commit -m "[build|frontend|vite][公共]前端引入 Vue 3 + Pinia"
```

---

## Task 5: `specta` 类型导出到前端

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `src-tauri/src/hook/protocol.rs`
- Modify: `src-tauri/src/session/phase.rs`
- Modify: `src-tauri/src/session/store.rs`
- Modify: `src-tauri/src/commands/session.rs`
- Modify: `src-tauri/src/commands/approval.rs`
- Modify: `src-tauri/src/commands/settings.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/main.rs`
- Create: `src-tauri/src/specta_export.rs`
- Create: `src/types/generated.ts`（自动产生）

- [ ] **Step 1: 加入 specta 依赖 + 写失败测试**

在 `src-tauri/Cargo.toml` 的 `[dependencies]` 追加（保留 Task 1 的全部其它依赖）：

```toml
specta = "2.0.0-rc.22"
specta-typescript = "0.0.9"
tauri-specta = { version = "2.0.0-rc.21", features = ["derive", "typescript"] }
```

创建 `src-tauri/src/specta_export.rs`：

```rust
//! 使用 tauri-specta 导出前端类型的测试入口。
//! 将 TypeScript 类型写入 ../src/types/generated.ts。

#[cfg(test)]
mod tests {
    use crate::commands;
    use specta_typescript::Typescript;
    use std::path::PathBuf;
    use tauri_specta::{collect_commands, Builder};

    #[test]
    fn generate_typescript_bindings() {
        let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
            commands::session::get_sessions,
            commands::session::get_session_count,
            commands::session::send_to_terminal,
            commands::approval::approve_permission,
            commands::approval::deny_permission,
            commands::settings::get_sound_enabled,
            commands::settings::set_sound_enabled,
        ]);

        let out = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("src")
            .join("types")
            .join("generated.ts");

        std::fs::create_dir_all(out.parent().unwrap()).unwrap();

        builder
            .export(Typescript::default(), &out)
            .expect("导出 TypeScript 类型失败");

        let content = std::fs::read_to_string(&out).unwrap();
        assert!(
            content.contains("export type SessionSummary"),
            "generated.ts 未包含 SessionSummary: {}",
            content
        );
        assert!(
            content.contains("approve_permission"),
            "generated.ts 未包含命令: {}",
            content
        );
    }
}
```

在 `src-tauri/src/main.rs` 的 `mod` 声明块中加入（Task 1 末状态上继续追加）：

```rust
mod specta_export;
```

位置：现有的

```rust
mod app_state;
mod commands;
mod hook;
mod session;
mod terminal;
mod sound;
```

修改为：

```rust
mod app_state;
mod commands;
mod hook;
mod session;
mod terminal;
mod sound;
mod specta_export;
```

- [ ] **Step 2: 运行测试验证失败**

Run: `cargo test --manifest-path src-tauri/Cargo.toml specta_export::tests::generate_typescript_bindings`

预期：编译失败——原因是 `SessionSummary`、`get_sessions` 等未 derive/annotate `specta`。错误示例：`the trait bound 'SessionSummary: specta::Type' is not satisfied`。

- [ ] **Step 3: 给 struct 加 `#[derive(specta::Type)]`，给 command 加 `#[specta::specta]`**

修改 `src-tauri/src/hook/protocol.rs`：在 `HookEvent` / `HookResponse` / `PendingPermission` 的 derive 列表加 `specta::Type`。具体替换：

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookEvent {
```
改为
```rust
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct HookEvent {
```

同理 `HookResponse`：

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookResponse {
```
改为
```rust
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct HookResponse {
```

`PendingPermission` 仅内部使用，不导出到前端，保持不变。

修改 `src-tauri/src/session/phase.rs`：

```rust
#[derive(Debug, Clone, Serialize)]
pub struct PermissionContext {
```
改为
```rust
#[derive(Debug, Clone, Serialize, specta::Type)]
pub struct PermissionContext {
```

```rust
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "context")]
pub enum SessionPhase {
```
改为
```rust
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(tag = "type", content = "context")]
pub enum SessionPhase {
```

修改 `src-tauri/src/session/store.rs` 中 `SessionSummary` 的 derive：

```rust
#[derive(Debug, Clone, serde::Serialize)]
pub struct SessionSummary {
```
改为
```rust
#[derive(Debug, Clone, serde::Serialize, specta::Type)]
pub struct SessionSummary {
```

修改 `src-tauri/src/commands/session.rs` 完整内容为：

```rust
use tauri::State;
use crate::app_state::AppState;
use crate::session::store::SessionSummary;

#[tauri::command]
#[specta::specta]
pub async fn get_sessions(state: State<'_, AppState>) -> Result<Vec<SessionSummary>, String> {
    let store = state.store.lock().await;
    Ok(store.get_summaries())
}

#[tauri::command]
#[specta::specta]
pub async fn get_session_count(state: State<'_, AppState>) -> Result<usize, String> {
    let store = state.store.lock().await;
    Ok(store.active_count())
}

#[tauri::command]
#[specta::specta]
pub async fn send_to_terminal(
    state: State<'_, AppState>,
    session_id: String,
    text: String,
) -> Result<bool, String> {
    let store = state.store.lock().await;
    let pid = store.get_session(&session_id).and_then(|s| s.pid);
    drop(store);

    match pid {
        Some(pid) => Ok(crate::terminal::writer::send_text_to_terminal(&text, pid)),
        None => Err("会话无 PID".into()),
    }
}
```

修改 `src-tauri/src/commands/approval.rs` 完整内容为：

```rust
use tauri::{State, Emitter, AppHandle};
use crate::app_state::AppState;
use crate::hook::protocol::HookResponse;

#[tauri::command]
#[specta::specta]
pub async fn approve_permission(
    state: State<'_, AppState>,
    app: AppHandle,
    session_id: String,
) -> Result<bool, String> {
    state.hook_server.respond(&session_id, HookResponse::allow()).await;

    let mut store = state.store.lock().await;
    let result = store.process_approval(&session_id);
    drop(store);

    let _ = app.emit("sessions-updated", ());
    Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn deny_permission(
    state: State<'_, AppState>,
    app: AppHandle,
    session_id: String,
    reason: Option<String>,
) -> Result<bool, String> {
    let reason_text = reason.unwrap_or_else(|| "Denied by user via CodeIsland".into());
    state.hook_server.respond(&session_id, HookResponse::deny(&reason_text)).await;

    let mut store = state.store.lock().await;
    let result = store.process_denial(&session_id);
    drop(store);

    let _ = app.emit("sessions-updated", ());
    Ok(result)
}
```

修改 `src-tauri/src/commands/settings.rs` 完整内容为：

```rust
use tauri::State;
use crate::app_state::AppState;

#[tauri::command]
#[specta::specta]
pub async fn get_sound_enabled(_state: State<'_, AppState>) -> Result<bool, String> {
    Ok(true)
}

#[tauri::command]
#[specta::specta]
pub async fn set_sound_enabled(
    state: State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    let mut sound = state.sound.lock().await;
    sound.set_enabled(enabled);
    Ok(())
}
```

- [ ] **Step 4: 运行测试验证通过**

Run: `cargo test --manifest-path src-tauri/Cargo.toml specta_export::tests::generate_typescript_bindings`

预期：PASS，并在 `src/types/generated.ts` 产生 ≥1KB 的 TypeScript 类型声明，包含 `SessionSummary`、`HookResponse`、命令 `approve_permission` / `deny_permission` / `get_sessions` 等。

复核：

```bash
cat src/types/generated.ts | head -20
```

应看到 `// This file has been generated by Specta. DO NOT EDIT.` 开头，及 `export type SessionSummary = ...`。

再跑全量测试确保 session / approval / settings 命令签名兼容：

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

预期：所有测试 PASS，`specta_export::tests::generate_typescript_bindings` 亦 PASS。

- [ ] **Step 5: Commit**

```bash
git add src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/src/main.rs src-tauri/src/specta_export.rs src-tauri/src/hook/protocol.rs src-tauri/src/session/phase.rs src-tauri/src/session/store.rs src-tauri/src/commands/session.rs src-tauri/src/commands/approval.rs src-tauri/src/commands/settings.rs src/types/generated.ts
git commit -m "[build|rust|specta][公共]类型导出 specta"
```

---

## Task 6: 现有组件迁移到 Vue 3

**Files:**
- Modify: `index.html`
- Modify: `src/main.ts`
- Modify: `src/styles/theme.css`
- Modify: `src/styles/island.css`
- Create: `src/stores/sessions.ts`
- Create: `src/stores/sessions.spec.ts`
- Create: `src/lib/tauri.ts`
- Create: `src/views/NotchView.vue`
- Create: `src/components/common/StatusDot.vue`
- Create: `src/components/session-card/SessionCard.vue`
- Create: `src/components/session-card/SessionCard.spec.ts`
- Create: `src/components/session-card/ApprovalButtons.vue`
- Create: `src/components/session-card/AskUserOptions.vue`
- Delete: `src/store.ts`
- Delete: `src/lib/events.ts`
- Delete: `src/components/Island.ts`
- Delete: `src/components/SessionCard.ts`
- Delete: `src/components/ApprovalButtons.ts`
- Delete: `src/components/AskUserOptions.ts`
- Delete: `src/components/StatusIndicator.ts`

- [ ] **Step 1: 写失败组件测试 + store 测试**

创建 `src/stores/sessions.spec.ts`：

```ts
import { describe, expect, it, vi, beforeEach } from "vitest";
import { createPinia, setActivePinia } from "pinia";

vi.mock("@/lib/tauri", () => ({
  invokeGetSessions: vi.fn(async () => [
    {
      session_id: "s-1",
      project_name: "demo",
      cwd: "C:\\\\tmp\\\\demo",
      phase: "waitingForApproval",
      needs_attention: true,
      last_message: null,
      tool_name: "Bash",
      tool_input: { command: "dir" },
    },
  ]),
  invokeApprovePermission: vi.fn(async () => true),
  invokeDenyPermission: vi.fn(async () => true),
  invokeSendToTerminal: vi.fn(async () => true),
  onSessionsUpdated: vi.fn(async () => () => {}),
}));

import { useSessionsStore } from "./sessions";

describe("useSessionsStore", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("refresh() 从 invoke 加载会话", async () => {
    const store = useSessionsStore();
    expect(store.list).toEqual([]);
    await store.refresh();
    expect(store.list.length).toBe(1);
    expect(store.list[0].session_id).toBe("s-1");
  });

  it("hasAttention 反映 needs_attention", async () => {
    const store = useSessionsStore();
    await store.refresh();
    expect(store.hasAttention).toBe(true);
  });

  it("overallStatus 为 attention 当存在 waitingForApproval", async () => {
    const store = useSessionsStore();
    await store.refresh();
    expect(store.overallStatus).toBe("attention");
  });
});
```

创建 `src/components/session-card/SessionCard.spec.ts`：

```ts
import { describe, expect, it, vi } from "vitest";
import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";

vi.mock("@/lib/tauri", () => ({
  invokeGetSessions: vi.fn(),
  invokeApprovePermission: vi.fn(async () => true),
  invokeDenyPermission: vi.fn(async () => true),
  invokeSendToTerminal: vi.fn(async () => true),
  onSessionsUpdated: vi.fn(async () => () => {}),
}));

import SessionCard from "./SessionCard.vue";
import type { SessionSummary } from "@/types/generated";

function makeSession(partial: Partial<SessionSummary> = {}): SessionSummary {
  return {
    session_id: "s-1",
    project_name: "demo",
    cwd: "C:/tmp/demo",
    phase: "idle",
    needs_attention: false,
    last_message: null,
    tool_name: null,
    tool_input: null,
    ...partial,
  } as SessionSummary;
}

describe("SessionCard", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("渲染项目名与 cwd", () => {
    const wrapper = mount(SessionCard, { props: { session: makeSession() } });
    expect(wrapper.text()).toContain("demo");
    expect(wrapper.text()).toContain("C:/tmp/demo");
  });

  it("phase 为 waitingForApproval 时显示 Allow / Deny", () => {
    const wrapper = mount(SessionCard, {
      props: {
        session: makeSession({
          phase: "waitingForApproval",
          needs_attention: true,
          tool_name: "Bash",
          tool_input: { command: "dir" },
        }),
      },
    });
    expect(wrapper.text()).toContain("Allow");
    expect(wrapper.text()).toContain("Deny");
  });

  it("phase 为 processing 时不显示审批按钮", () => {
    const wrapper = mount(SessionCard, {
      props: { session: makeSession({ phase: "processing", tool_name: "Read" }) },
    });
    expect(wrapper.text()).not.toContain("Allow");
  });
});
```

- [ ] **Step 2: 运行测试验证失败**

Run:

```bash
npm run test
```

预期：`sessions.spec.ts` 与 `SessionCard.spec.ts` 均 FAIL——模块 `@/stores/sessions` / `@/components/session-card/SessionCard.vue` / `@/lib/tauri` 不存在。

- [ ] **Step 3: 实现 — 创建 Vue 组件与 Pinia store**

创建 `src/lib/tauri.ts`：

```ts
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { SessionSummary } from "@/types/generated";

export async function invokeGetSessions(): Promise<SessionSummary[]> {
  return invoke("get_sessions");
}

export async function invokeGetSessionCount(): Promise<number> {
  return invoke("get_session_count");
}

export async function invokeApprovePermission(sessionId: string): Promise<boolean> {
  return invoke("approve_permission", { sessionId });
}

export async function invokeDenyPermission(sessionId: string, reason?: string): Promise<boolean> {
  return invoke("deny_permission", { sessionId, reason });
}

export async function invokeSendToTerminal(sessionId: string, text: string): Promise<boolean> {
  return invoke("send_to_terminal", { sessionId, text });
}

export async function onSessionsUpdated(callback: () => void): Promise<UnlistenFn> {
  return listen("sessions-updated", callback);
}
```

创建 `src/stores/sessions.ts`：

```ts
import { defineStore } from "pinia";
import { computed, ref } from "vue";
import type { SessionSummary } from "@/types/generated";
import {
  invokeApprovePermission,
  invokeDenyPermission,
  invokeGetSessions,
  invokeSendToTerminal,
} from "@/lib/tauri";

export const useSessionsStore = defineStore("sessions", () => {
  const list = ref<SessionSummary[]>([]);

  const byId = computed(
    () => new Map(list.value.map((s) => [s.session_id, s] as const)),
  );

  const hasAttention = computed(() => list.value.some((s) => s.needs_attention));

  const overallStatus = computed<"idle" | "processing" | "attention">(() => {
    if (list.value.some((s) => s.phase === "waitingForApproval")) return "attention";
    if (list.value.some((s) => s.phase === "processing" || s.phase === "compacting")) {
      return "processing";
    }
    return "idle";
  });

  async function refresh(): Promise<void> {
    try {
      list.value = await invokeGetSessions();
    } catch (e) {
      console.error("[sessions] refresh 失败:", e);
    }
  }

  async function approve(sessionId: string): Promise<void> {
    await invokeApprovePermission(sessionId);
  }

  async function deny(sessionId: string, reason?: string): Promise<void> {
    await invokeDenyPermission(sessionId, reason);
  }

  async function sendToTerminal(sessionId: string, text: string): Promise<void> {
    await invokeSendToTerminal(sessionId, text);
  }

  return {
    list,
    byId,
    hasAttention,
    overallStatus,
    refresh,
    approve,
    deny,
    sendToTerminal,
  };
});
```

创建 `src/components/common/StatusDot.vue`：

```vue
<script setup lang="ts">
defineProps<{ status: "idle" | "processing" | "attention" | "success" | "error" }>();
</script>

<template>
  <span class="status-dot" :class="status" />
</template>
```

创建 `src/components/session-card/ApprovalButtons.vue`：

```vue
<script setup lang="ts">
import { useSessionsStore } from "@/stores/sessions";

const props = defineProps<{ sessionId: string }>();
const sessions = useSessionsStore();

async function onAllow(e: Event) {
  e.stopPropagation();
  await sessions.approve(props.sessionId);
}

async function onDeny(e: Event) {
  e.stopPropagation();
  await sessions.deny(props.sessionId);
}
</script>

<template>
  <div class="approval-row">
    <button class="btn btn-allow" @click="onAllow">Allow</button>
    <button class="btn btn-deny" @click="onDeny">Deny</button>
  </div>
</template>
```

创建 `src/components/session-card/AskUserOptions.vue`：

```vue
<script setup lang="ts">
import { computed } from "vue";
import { useSessionsStore } from "@/stores/sessions";

const props = defineProps<{
  sessionId: string;
  toolInput: Record<string, unknown> | null;
}>();

const sessions = useSessionsStore();

const options = computed<string[]>(() => {
  if (!props.toolInput) return [];
  const raw = (props.toolInput as { options?: unknown }).options;
  return Array.isArray(raw) ? (raw as string[]) : [];
});

async function onChoose(option: string, e: Event) {
  e.stopPropagation();
  await sessions.sendToTerminal(props.sessionId, option);
}
</script>

<template>
  <div v-if="options.length" class="askuser-options">
    <button
      v-for="option in options"
      :key="option"
      class="btn-option"
      @click="onChoose(option, $event)"
    >
      {{ option }}
    </button>
  </div>
</template>
```

创建 `src/components/session-card/SessionCard.vue`：

```vue
<script setup lang="ts">
import { computed } from "vue";
import type { SessionSummary } from "@/types/generated";
import { useSessionsStore } from "@/stores/sessions";
import StatusDot from "@/components/common/StatusDot.vue";
import ApprovalButtons from "./ApprovalButtons.vue";
import AskUserOptions from "./AskUserOptions.vue";

const props = defineProps<{ session: SessionSummary }>();
const sessions = useSessionsStore();

const dotStatus = computed<"idle" | "processing" | "attention">(() => {
  if (props.session.needs_attention) return "attention";
  if (props.session.phase === "processing") return "processing";
  return "idle";
});

const statusText = computed(() => {
  const s = props.session;
  switch (s.phase) {
    case "processing":
      return `Processing${s.tool_name ? ` (${s.tool_name})` : ""}`;
    case "waitingForApproval":
      return `Awaiting approval: ${s.tool_name ?? "tool"}`;
    case "waitingForInput":
      return s.last_message ?? "Waiting for input";
    case "compacting":
      return "Compacting context...";
    case "idle":
      return s.last_message ?? "Idle";
    default:
      return s.phase;
  }
});

const toolInfoText = computed(() => {
  const s = props.session;
  if (!s.tool_name || !s.tool_input || s.phase !== "waitingForApproval") return null;
  const parts = [`Tool: ${s.tool_name}`];
  for (const [key, value] of Object.entries(s.tool_input)) {
    const str = typeof value === "string" ? value : JSON.stringify(value);
    parts.push(`${key}: ${str.length > 100 ? str.slice(0, 100) + "..." : str}`);
  }
  return parts.join("\n");
});

const isAskUser = computed(
  () =>
    props.session.tool_name === "AskUserQuestion" ||
    props.session.tool_name === "AskUser",
);

async function onCardClick() {
  await sessions.sendToTerminal(props.session.session_id, "");
}
</script>

<template>
  <div class="session-card" @click="onCardClick">
    <div class="session-header">
      <StatusDot :status="dotStatus" />
      <span class="session-name">{{ session.project_name }}</span>
      <span class="session-cwd">{{ session.cwd }}</span>
    </div>
    <div :class="['session-status', { attention: session.needs_attention }]">
      {{ statusText }}
    </div>
    <div v-if="toolInfoText" class="tool-info">{{ toolInfoText }}</div>
    <ApprovalButtons
      v-if="session.phase === 'waitingForApproval'"
      :session-id="session.session_id"
    />
    <AskUserOptions
      v-if="isAskUser"
      :session-id="session.session_id"
      :tool-input="session.tool_input"
    />
  </div>
</template>
```

创建 `src/views/NotchView.vue`（Stage 0 保持圆角矩形，Task 7 再替换为 SVG 刘海）：

```vue
<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useSessionsStore } from "@/stores/sessions";
import { onSessionsUpdated } from "@/lib/tauri";
import StatusDot from "@/components/common/StatusDot.vue";
import SessionCard from "@/components/session-card/SessionCard.vue";

const sessions = useSessionsStore();
const expanded = ref(false);
let collapseTimer: number | null = null;
let unlisten: (() => void) | null = null;

const sessionCount = computed(() => sessions.list.length);
const countLabel = computed(
  () => `${sessionCount.value} session${sessionCount.value !== 1 ? "s" : ""}`,
);

function onEnter() {
  if (collapseTimer !== null) {
    window.clearTimeout(collapseTimer);
    collapseTimer = null;
  }
  expanded.value = true;
}

function onLeave() {
  collapseTimer = window.setTimeout(() => {
    expanded.value = false;
  }, 300);
}

async function onCloseClick(e: Event) {
  e.stopPropagation();
  await getCurrentWindow().hide();
}

onMounted(async () => {
  await sessions.refresh();
  unlisten = await onSessionsUpdated(() => {
    void sessions.refresh();
  });
});

onUnmounted(() => {
  if (unlisten) unlisten();
  if (collapseTimer !== null) window.clearTimeout(collapseTimer);
});
</script>

<template>
  <div
    class="island"
    :class="{ collapsed: !expanded, expanded, alert: sessions.hasAttention }"
    @mouseenter="onEnter"
    @mouseleave="onLeave"
  >
    <div v-if="!expanded" class="island-collapsed" data-tauri-drag-region>
      <StatusDot :status="sessions.overallStatus" />
      <span class="session-count">{{ countLabel }}</span>
      <span class="status-label">{{ sessions.overallStatus }}</span>
    </div>
    <div v-else class="island-expanded">
      <div class="expanded-header" data-tauri-drag-region>
        <span class="header-title">Code Island</span>
        <button class="btn-close" title="隐藏窗口" @click="onCloseClick">×</button>
      </div>
      <div class="session-list">
        <div v-if="sessions.list.length === 0" class="empty-state">
          No active sessions
        </div>
        <SessionCard
          v-for="session in sessions.list"
          :key="session.session_id"
          :session="session"
        />
      </div>
    </div>
  </div>
</template>
```

替换 `index.html` 完整内容为：

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Code Island</title>
  <link rel="stylesheet" href="/src/styles/theme.css" />
  <link rel="stylesheet" href="/src/styles/island.css" />
</head>
<body>
  <div id="app"></div>
  <script type="module" src="/src/main.ts"></script>
</body>
</html>
```

替换 `src/main.ts` 完整内容为：

```ts
import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import NotchView from "./views/NotchView.vue";

const pinia = createPinia();

const app = createApp({
  components: { App, NotchView },
  template: "<App><NotchView /></App>",
});

app.use(pinia);
app.mount("#app");
```

替换 `src/styles/island.css` 末尾追加 `session-list` 相关样式（保留原有 `.island` / `.status-dot` / `.btn-*` 等）：

```css
.session-list { padding: 0 8px 8px; max-height: 70vh; overflow-y: auto; }
```

整文件替换 `src/styles/island.css` 为：

```css
.island {
  position: fixed;
  top: 8px;
  left: 50%;
  transform: translateX(-50%);
  background: var(--bg-primary);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-radius: var(--radius);
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.3);
  transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1),
              height 0.3s cubic-bezier(0.4, 0, 0.2, 1),
              border-radius 0.3s ease;
  overflow: hidden;
  cursor: default;
}

.island.collapsed { width: 220px; height: 36px; }
.island-collapsed {
  display: flex; align-items: center; gap: 8px;
  padding: 0 16px; height: 36px; white-space: nowrap;
}
.island.expanded { width: 380px; height: auto; max-height: 80vh; border-radius: var(--radius); }
.island-expanded { padding: 8px; }
.expanded-header {
  display: flex; align-items: center; justify-content: space-between;
  padding: 8px 12px 4px; cursor: grab;
}
.expanded-header:active { cursor: grabbing; }
.header-title { font-size: 12px; font-weight: 600; color: var(--text-secondary); }
.island.alert { animation: pulse-glow 1.5s ease-in-out infinite; }

@keyframes pulse-glow {
  0%, 100% { box-shadow: 0 4px 24px rgba(0, 0, 0, 0.3); }
  50% { box-shadow: 0 4px 32px rgba(255, 152, 0, 0.4), 0 0 16px rgba(255, 152, 0, 0.2); }
}

.status-dot {
  width: 8px; height: 8px; border-radius: 50%;
  background: var(--accent-green); flex-shrink: 0;
}
.status-dot.processing { background: var(--accent-blue); animation: dot-pulse 1.2s ease-in-out infinite; }
.status-dot.attention { background: var(--accent-yellow); animation: dot-pulse 0.8s ease-in-out infinite; }
.status-dot.idle { background: var(--text-secondary); }

@keyframes dot-pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.4; } }

.session-count { font-size: 12px; font-weight: 500; color: var(--text-primary); }
.status-label { font-size: 11px; color: var(--text-secondary); margin-left: auto; }
.session-list { padding: 0 0 8px; max-height: 70vh; overflow-y: auto; }

.session-card {
  padding: 10px 12px; border-radius: var(--radius-sm);
  background: var(--bg-secondary); margin-bottom: 6px;
  cursor: pointer; transition: background 0.15s ease;
}
.session-card:hover { background: rgba(255, 255, 255, 0.08); }
.session-card:last-child { margin-bottom: 0; }
.session-header { display: flex; align-items: center; gap: 6px; margin-bottom: 4px; }
.session-name { font-size: 12px; font-weight: 600; color: var(--text-primary); }
.session-cwd {
  font-size: 10px; color: var(--text-secondary);
  overflow: hidden; text-overflow: ellipsis; white-space: nowrap; max-width: 180px;
}
.session-status { font-size: 11px; color: var(--text-secondary); margin-top: 2px; }
.session-status.attention { color: var(--accent-yellow); font-weight: 500; }

.approval-row { display: flex; gap: 8px; margin-top: 8px; }
.btn {
  flex: 1; padding: 6px 12px; border: none; border-radius: 8px;
  font-size: 12px; font-weight: 600; cursor: pointer;
  transition: transform 0.1s ease, opacity 0.1s ease;
}
.btn:active { transform: scale(0.96); }
.btn-allow { background: var(--accent-green); color: white; }
.btn-deny { background: var(--accent-red); color: white; }

.askuser-options { display: flex; flex-wrap: wrap; gap: 6px; margin-top: 6px; }
.btn-option {
  padding: 4px 10px; border: 1px solid rgba(255, 255, 255, 0.15);
  border-radius: 6px; background: transparent; color: var(--text-primary);
  font-size: 11px; cursor: pointer; transition: background 0.15s ease;
}
.btn-option:hover { background: rgba(255, 255, 255, 0.1); }

.tool-info {
  font-size: 11px; color: var(--text-secondary); margin-top: 4px;
  padding: 4px 8px; background: rgba(0, 0, 0, 0.2); border-radius: 6px;
  font-family: "Cascadia Code", "Consolas", monospace;
  white-space: pre-wrap; word-break: break-all; max-height: 80px; overflow-y: auto;
}

.empty-state { text-align: center; padding: 20px; color: var(--text-secondary); font-size: 12px; }

.btn-close {
  width: 20px; height: 20px; border: none; border-radius: 50%;
  background: rgba(255, 255, 255, 0.1); color: var(--text-secondary);
  font-size: 14px; line-height: 20px; text-align: center;
  cursor: pointer; transition: background 0.15s ease, color 0.15s ease;
  padding: 0; flex-shrink: 0;
}
.btn-close:hover { background: var(--accent-red); color: white; }
```

删除旧 TS 组件：

```bash
rm src/store.ts src/lib/events.ts src/components/Island.ts src/components/SessionCard.ts src/components/ApprovalButtons.ts src/components/AskUserOptions.ts src/components/StatusIndicator.ts
```

- [ ] **Step 4: 运行测试验证通过**

Run:

```bash
npm run test
```

预期：5 个测试全部 PASS：
- `App.vue > 渲染根容器`
- `useSessionsStore > refresh() 从 invoke 加载会话`
- `useSessionsStore > hasAttention 反映 needs_attention`
- `useSessionsStore > overallStatus 为 attention 当存在 waitingForApproval`
- `SessionCard > 渲染项目名与 cwd` / `phase 为 waitingForApproval 时显示 Allow / Deny` / `phase 为 processing 时不显示审批按钮`

Run:

```bash
npm run build
```

预期：`vue-tsc --noEmit` 无错误，`vite build` 生成 `dist/`；无 `src/store.ts` 等旧文件的 dangling 引用。

手工验证（可选，仅 Windows 环境）：`npm run tauri dev`，观察刘海 UI 仍显示、hover 展开、审批按钮可点击。

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "[refactor|UI|migrate][公共]现有组件迁移 Vue"
```

---

## Task 7: 虚拟刘海 SVG 形状 + 展开动画

**Files:**
- Create: `src/utils/notch-shape.ts`
- Create: `src/utils/notch-shape.spec.ts`
- Create: `src/stores/notch.ts`
- Create: `src/components/notch/NotchShape.vue`
- Modify: `src/views/NotchView.vue`
- Modify: `src/styles/island.css`

- [ ] **Step 1: 写失败单元测试 — `notch-shape.ts` 几何函数**

创建 `src/utils/notch-shape.spec.ts`：

```ts
import { describe, expect, it } from "vitest";
import {
  COLLAPSED,
  EXPANDED,
  computeNotchPath,
  interpolateGeometry,
  type NotchGeometry,
} from "./notch-shape";

describe("interpolateGeometry", () => {
  it("t=0 返回 start", () => {
    expect(interpolateGeometry(COLLAPSED, EXPANDED, 0)).toEqual(COLLAPSED);
  });

  it("t=1 返回 end", () => {
    expect(interpolateGeometry(COLLAPSED, EXPANDED, 1)).toEqual(EXPANDED);
  });

  it("t=0.5 线性插值", () => {
    const mid = interpolateGeometry(COLLAPSED, EXPANDED, 0.5);
    expect(mid.width).toBe((COLLAPSED.width + EXPANDED.width) / 2);
    expect(mid.height).toBe((COLLAPSED.height + EXPANDED.height) / 2);
    expect(mid.topCornerRadius).toBeCloseTo(
      (COLLAPSED.topCornerRadius + EXPANDED.topCornerRadius) / 2,
    );
    expect(mid.bottomCornerRadius).toBeCloseTo(
      (COLLAPSED.bottomCornerRadius + EXPANDED.bottomCornerRadius) / 2,
    );
  });

  it("t 被 clamp 在 [0,1]", () => {
    expect(interpolateGeometry(COLLAPSED, EXPANDED, -0.5)).toEqual(COLLAPSED);
    expect(interpolateGeometry(COLLAPSED, EXPANDED, 1.5)).toEqual(EXPANDED);
  });
});

describe("computeNotchPath", () => {
  it("生成闭合 SVG path，以 M 开头 Z 结尾", () => {
    const path = computeNotchPath(EXPANDED);
    expect(path.startsWith("M")).toBe(true);
    expect(path.trim().endsWith("Z")).toBe(true);
  });

  it("路径含 4 段圆弧", () => {
    const path = computeNotchPath(EXPANDED);
    const arcs = path.match(/A\s/g) ?? [];
    expect(arcs.length).toBe(4);
  });

  it("不同尺寸生成不同路径", () => {
    expect(computeNotchPath(COLLAPSED)).not.toBe(computeNotchPath(EXPANDED));
  });

  it("零圆角退化为矩形（仍闭合）", () => {
    const rect: NotchGeometry = {
      width: 100,
      height: 50,
      topCornerRadius: 0,
      bottomCornerRadius: 0,
    };
    const path = computeNotchPath(rect);
    expect(path.startsWith("M")).toBe(true);
    expect(path.trim().endsWith("Z")).toBe(true);
  });
});
```

- [ ] **Step 2: 运行测试验证失败**

Run: `npm run test -- notch-shape`

预期：FAIL —— `Cannot find module './notch-shape'`。

- [ ] **Step 3: 实现 — `notch-shape.ts` + `notch.ts` store + `NotchShape.vue`**

创建 `src/utils/notch-shape.ts`：

```ts
export interface NotchGeometry {
  width: number;
  height: number;
  topCornerRadius: number;
  bottomCornerRadius: number;
}

export const COLLAPSED: NotchGeometry = {
  width: 220,
  height: 32,
  topCornerRadius: 6,
  bottomCornerRadius: 14,
};

export const EXPANDED: NotchGeometry = {
  width: 760,
  height: 520,
  topCornerRadius: 19,
  bottomCornerRadius: 24,
};

function clamp01(t: number): number {
  if (t <= 0) return 0;
  if (t >= 1) return 1;
  return t;
}

function lerp(a: number, b: number, t: number): number {
  return a + (b - a) * t;
}

export function interpolateGeometry(
  start: NotchGeometry,
  end: NotchGeometry,
  t: number,
): NotchGeometry {
  const k = clamp01(t);
  if (k === 0) return { ...start };
  if (k === 1) return { ...end };
  return {
    width: lerp(start.width, end.width, k),
    height: lerp(start.height, end.height, k),
    topCornerRadius: lerp(start.topCornerRadius, end.topCornerRadius, k),
    bottomCornerRadius: lerp(start.bottomCornerRadius, end.bottomCornerRadius, k),
  };
}

/**
 * 生成虚拟刘海 SVG path。
 *
 * 路径顺时针，从左上起点，依次经过：
 *   顶边 → 右上圆弧（外凸） → 右侧直线 → 右下反向圆弧（内凹） →
 *   底边 → 左下反向圆弧（内凹） → 左侧直线 → 左上圆弧（外凸） → Z
 */
export function computeNotchPath(g: NotchGeometry): string {
  const w = g.width;
  const h = g.height;
  const tr = Math.max(0, Math.min(g.topCornerRadius, Math.min(w / 2, h)));
  const br = Math.max(0, Math.min(g.bottomCornerRadius, Math.min(w / 2, h)));

  const startX = tr;
  const startY = 0;

  const parts: string[] = [];
  parts.push(`M ${startX} ${startY}`);
  parts.push(`L ${w - tr} 0`);
  if (tr > 0) parts.push(`A ${tr} ${tr} 0 0 1 ${w} ${tr}`);
  parts.push(`L ${w} ${h - br}`);
  if (br > 0) parts.push(`A ${br} ${br} 0 0 0 ${w - br} ${h}`);
  parts.push(`L ${br} ${h}`);
  if (br > 0) parts.push(`A ${br} ${br} 0 0 0 0 ${h - br}`);
  parts.push(`L 0 ${tr}`);
  if (tr > 0) parts.push(`A ${tr} ${tr} 0 0 1 ${tr} 0`);
  parts.push(`Z`);

  return parts.join(" ");
}
```

创建 `src/stores/notch.ts`：

```ts
import { defineStore } from "pinia";
import { computed, ref } from "vue";
import {
  COLLAPSED,
  EXPANDED,
  computeNotchPath,
  interpolateGeometry,
  type NotchGeometry,
} from "@/utils/notch-shape";

export const useNotchStore = defineStore("notch", () => {
  const expanded = ref(false);
  const geometry = ref<NotchGeometry>({ ...COLLAPSED });

  const path = computed(() => computeNotchPath(geometry.value));

  const viewBox = computed(
    () => `0 0 ${geometry.value.width} ${geometry.value.height}`,
  );

  function setGeometry(next: NotchGeometry) {
    geometry.value = { ...next };
  }

  function setExpanded(next: boolean) {
    expanded.value = next;
    geometry.value = next ? { ...EXPANDED } : { ...COLLAPSED };
  }

  function animateTo(target: NotchGeometry, durationMs = 280): Promise<void> {
    return new Promise((resolve) => {
      const start = { ...geometry.value };
      const startTs = performance.now();
      function tick(now: number) {
        const t = Math.min(1, (now - startTs) / durationMs);
        const eased = easeOutExpo(t);
        geometry.value = interpolateGeometry(start, target, eased);
        if (t < 1) requestAnimationFrame(tick);
        else resolve();
      }
      requestAnimationFrame(tick);
    });
  }

  function animateExpanded(next: boolean, durationMs = 280) {
    expanded.value = next;
    return animateTo(next ? { ...EXPANDED } : { ...COLLAPSED }, durationMs);
  }

  return {
    expanded,
    geometry,
    path,
    viewBox,
    setGeometry,
    setExpanded,
    animateTo,
    animateExpanded,
  };
});

function easeOutExpo(t: number): number {
  return t === 1 ? 1 : 1 - Math.pow(2, -10 * t);
}
```

创建 `src/components/notch/NotchShape.vue`：

```vue
<script setup lang="ts">
import { computed } from "vue";
import { useNotchStore } from "@/stores/notch";

const notch = useNotchStore();

const rootStyle = computed(() => ({
  width: `${notch.geometry.width}px`,
  height: `${notch.geometry.height}px`,
}));

const clipId = "notch-clip";
const shadowId = "notch-shadow";
</script>

<template>
  <div class="notch-root" :style="rootStyle">
    <svg class="notch-svg" :viewBox="notch.viewBox" :width="notch.geometry.width" :height="notch.geometry.height">
      <defs>
        <clipPath :id="clipId">
          <path :d="notch.path" />
        </clipPath>
        <filter :id="shadowId" x="-20%" y="-20%" width="140%" height="140%">
          <feDropShadow dx="0" dy="4" stdDeviation="12" flood-opacity="0.4" />
        </filter>
      </defs>
      <path :d="notch.path" fill="rgba(0,0,0,0.85)" :filter="`url(#${shadowId})`" />
    </svg>
    <div class="notch-content" :style="{ clipPath: `url(#${clipId})` }">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.notch-root {
  position: relative;
  pointer-events: none;
}
.notch-svg {
  position: absolute;
  top: 0;
  left: 0;
  pointer-events: none;
  overflow: visible;
}
.notch-content {
  position: absolute;
  inset: 0;
  pointer-events: auto;
}
</style>
```

替换 `src/views/NotchView.vue` 完整内容为：

```vue
<script setup lang="ts">
import { computed, onMounted, onUnmounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useSessionsStore } from "@/stores/sessions";
import { useNotchStore } from "@/stores/notch";
import { onSessionsUpdated } from "@/lib/tauri";
import StatusDot from "@/components/common/StatusDot.vue";
import SessionCard from "@/components/session-card/SessionCard.vue";
import NotchShape from "@/components/notch/NotchShape.vue";

const sessions = useSessionsStore();
const notch = useNotchStore();
let collapseTimer: number | null = null;
let unlisten: (() => void) | null = null;

const sessionCount = computed(() => sessions.list.length);
const countLabel = computed(
  () => `${sessionCount.value} session${sessionCount.value !== 1 ? "s" : ""}`,
);

function onEnter() {
  if (collapseTimer !== null) {
    window.clearTimeout(collapseTimer);
    collapseTimer = null;
  }
  void notch.animateExpanded(true);
}

function onLeave() {
  collapseTimer = window.setTimeout(() => {
    void notch.animateExpanded(false);
  }, 300);
}

async function onCloseClick(e: Event) {
  e.stopPropagation();
  await getCurrentWindow().hide();
}

onMounted(async () => {
  notch.setExpanded(false);
  await sessions.refresh();
  unlisten = await onSessionsUpdated(() => {
    void sessions.refresh();
  });
});

onUnmounted(() => {
  if (unlisten) unlisten();
  if (collapseTimer !== null) window.clearTimeout(collapseTimer);
});
</script>

<template>
  <div class="notch-wrapper" @mouseenter="onEnter" @mouseleave="onLeave">
    <NotchShape>
      <div v-if="!notch.expanded" class="island-collapsed" data-tauri-drag-region>
        <StatusDot :status="sessions.overallStatus" />
        <span class="session-count">{{ countLabel }}</span>
        <span class="status-label">{{ sessions.overallStatus }}</span>
      </div>
      <div v-else class="island-expanded">
        <div class="expanded-header" data-tauri-drag-region>
          <span class="header-title">Code Island</span>
          <button class="btn-close" title="隐藏窗口" @click="onCloseClick">×</button>
        </div>
        <div class="session-list">
          <div v-if="sessions.list.length === 0" class="empty-state">
            No active sessions
          </div>
          <SessionCard
            v-for="session in sessions.list"
            :key="session.session_id"
            :session="session"
          />
        </div>
      </div>
    </NotchShape>
  </div>
</template>

<style scoped>
.notch-wrapper {
  position: fixed;
  top: 0;
  left: 50%;
  transform: translateX(-50%);
}
</style>
```

修改 `src/styles/island.css`：删除旧的 `.island` 容器样式（`position: fixed; top: 8px; border-radius; backdrop-filter` 等），保留 `.island-collapsed` / `.island-expanded` / `.status-dot` / `.session-*` / `.btn-*` / `.empty-state` / `.btn-close` / 内部动画 keyframes。完整替换为：

```css
.island-collapsed {
  display: flex; align-items: center; gap: 8px;
  padding: 0 16px; height: 32px; white-space: nowrap;
  color: var(--text-primary);
}
.island-expanded { padding: 8px; color: var(--text-primary); }
.expanded-header {
  display: flex; align-items: center; justify-content: space-between;
  padding: 8px 12px 4px; cursor: grab;
}
.expanded-header:active { cursor: grabbing; }
.header-title { font-size: 12px; font-weight: 600; color: var(--text-secondary); }

.status-dot {
  width: 8px; height: 8px; border-radius: 50%;
  background: var(--accent-green); flex-shrink: 0;
}
.status-dot.processing { background: var(--accent-blue); animation: dot-pulse 1.2s ease-in-out infinite; }
.status-dot.attention { background: var(--accent-yellow); animation: dot-pulse 0.8s ease-in-out infinite; }
.status-dot.idle { background: var(--text-secondary); }

@keyframes dot-pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.4; } }

.session-count { font-size: 12px; font-weight: 500; color: var(--text-primary); }
.status-label { font-size: 11px; color: var(--text-secondary); margin-left: auto; }
.session-list { padding: 0 0 8px; max-height: 70vh; overflow-y: auto; }

.session-card {
  padding: 10px 12px; border-radius: var(--radius-sm);
  background: var(--bg-secondary); margin-bottom: 6px;
  cursor: pointer; transition: background 0.15s ease;
}
.session-card:hover { background: rgba(255, 255, 255, 0.08); }
.session-card:last-child { margin-bottom: 0; }
.session-header { display: flex; align-items: center; gap: 6px; margin-bottom: 4px; }
.session-name { font-size: 12px; font-weight: 600; color: var(--text-primary); }
.session-cwd {
  font-size: 10px; color: var(--text-secondary);
  overflow: hidden; text-overflow: ellipsis; white-space: nowrap; max-width: 180px;
}
.session-status { font-size: 11px; color: var(--text-secondary); margin-top: 2px; }
.session-status.attention { color: var(--accent-yellow); font-weight: 500; }

.approval-row { display: flex; gap: 8px; margin-top: 8px; }
.btn {
  flex: 1; padding: 6px 12px; border: none; border-radius: 8px;
  font-size: 12px; font-weight: 600; cursor: pointer;
  transition: transform 0.1s ease, opacity 0.1s ease;
}
.btn:active { transform: scale(0.96); }
.btn-allow { background: var(--accent-green); color: white; }
.btn-deny { background: var(--accent-red); color: white; }

.askuser-options { display: flex; flex-wrap: wrap; gap: 6px; margin-top: 6px; }
.btn-option {
  padding: 4px 10px; border: 1px solid rgba(255, 255, 255, 0.15);
  border-radius: 6px; background: transparent; color: var(--text-primary);
  font-size: 11px; cursor: pointer; transition: background 0.15s ease;
}
.btn-option:hover { background: rgba(255, 255, 255, 0.1); }

.tool-info {
  font-size: 11px; color: var(--text-secondary); margin-top: 4px;
  padding: 4px 8px; background: rgba(0, 0, 0, 0.2); border-radius: 6px;
  font-family: "Cascadia Code", "Consolas", monospace;
  white-space: pre-wrap; word-break: break-all; max-height: 80px; overflow-y: auto;
}

.empty-state { text-align: center; padding: 20px; color: var(--text-secondary); font-size: 12px; }

.btn-close {
  width: 20px; height: 20px; border: none; border-radius: 50%;
  background: rgba(255, 255, 255, 0.1); color: var(--text-secondary);
  font-size: 14px; line-height: 20px; text-align: center;
  cursor: pointer; transition: background 0.15s ease, color 0.15s ease;
  padding: 0; flex-shrink: 0;
}
.btn-close:hover { background: var(--accent-red); color: white; }
```

- [ ] **Step 4: 运行测试验证通过**

Run:

```bash
npm run test -- notch-shape
```

预期：7 个 `notch-shape` 相关测试 PASS。

Run 完整：

```bash
npm run test
npm run build
```

预期：之前的 Task 4 / 6 测试仍 PASS；`vue-tsc --noEmit` 与 `vite build` 均无错误。

手工验证（可选，Windows）：`npm run tauri dev`，观察窗口显示黑色异形刘海，hover 展开时路径插值动画平滑，无闪烁；关闭按钮仍可用。

- [ ] **Step 5: Commit**

```bash
git add src/utils/notch-shape.ts src/utils/notch-shape.spec.ts src/stores/notch.ts src/components/notch/NotchShape.vue src/views/NotchView.vue src/styles/island.css
git commit -m "[feat|UI|notch][公共]虚拟刘海 SVG 形状 + 动画"
```

---

## Task 8: cursor 穿透与窗口尺寸命令

**Files:**
- Create: `src-tauri/src/commands/window_control.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/main.rs`
- Modify: `src-tauri/src/specta_export.rs`
- Modify: `src/lib/tauri.ts`
- Modify: `src/views/NotchView.vue`

- [ ] **Step 1: 写失败 Rust 单元测试**

创建 `src-tauri/src/commands/window_control.rs`（先写测试）：

```rust
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
```

在 `src-tauri/src/commands/mod.rs` 追加：

```rust
pub mod window_control;
```

整文件应为：

```rust
pub mod session;
pub mod approval;
pub mod settings;
pub mod window_control;
```

- [ ] **Step 2: 运行测试验证失败**

Run: `cargo test --manifest-path src-tauri/Cargo.toml commands::window_control::tests`

预期：编译失败 —— `tauri::test::mock_app` 需 `tauri` crate 的 `test` feature。错误示例：`cannot find function 'mock_app' in module 'tauri::test'`。

- [ ] **Step 3: 启用 tauri test feature + 在 main.rs 挂载命令 + 更新 specta_export**

修改 `src-tauri/Cargo.toml`，将 `tauri` 依赖行从：

```toml
tauri = { version = "2", features = ["tray-icon", "image-png"] }
```

改为：

```toml
tauri = { version = "2", features = ["tray-icon", "image-png", "test"] }
```

修改 `src-tauri/src/main.rs` 的 `invoke_handler!` 块，将：

```rust
        .invoke_handler(tauri::generate_handler![
            commands::session::get_sessions,
            commands::session::get_session_count,
            commands::session::send_to_terminal,
            commands::approval::approve_permission,
            commands::approval::deny_permission,
            commands::settings::get_sound_enabled,
            commands::settings::set_sound_enabled,
        ])
```

替换为：

```rust
        .invoke_handler(tauri::generate_handler![
            commands::session::get_sessions,
            commands::session::get_session_count,
            commands::session::send_to_terminal,
            commands::approval::approve_permission,
            commands::approval::deny_permission,
            commands::settings::get_sound_enabled,
            commands::settings::set_sound_enabled,
            commands::window_control::set_ignore_cursor_events,
            commands::window_control::set_window_size,
            commands::window_control::set_window_position,
        ])
```

修改 `src-tauri/src/specta_export.rs` 的 `collect_commands!` 列表，追加 3 个新命令：

```rust
            commands::window_control::set_ignore_cursor_events,
            commands::window_control::set_window_size,
            commands::window_control::set_window_position,
```

整块完整内容为：

```rust
//! 使用 tauri-specta 导出前端类型的测试入口。
//! 将 TypeScript 类型写入 ../src/types/generated.ts。

#[cfg(test)]
mod tests {
    use crate::commands;
    use specta_typescript::Typescript;
    use std::path::PathBuf;
    use tauri_specta::{collect_commands, Builder};

    #[test]
    fn generate_typescript_bindings() {
        let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
            commands::session::get_sessions,
            commands::session::get_session_count,
            commands::session::send_to_terminal,
            commands::approval::approve_permission,
            commands::approval::deny_permission,
            commands::settings::get_sound_enabled,
            commands::settings::set_sound_enabled,
            commands::window_control::set_ignore_cursor_events,
            commands::window_control::set_window_size,
            commands::window_control::set_window_position,
        ]);

        let out = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("src")
            .join("types")
            .join("generated.ts");

        std::fs::create_dir_all(out.parent().unwrap()).unwrap();

        builder
            .export(Typescript::default(), &out)
            .expect("导出 TypeScript 类型失败");

        let content = std::fs::read_to_string(&out).unwrap();
        assert!(
            content.contains("export type SessionSummary"),
            "generated.ts 未包含 SessionSummary: {}",
            content
        );
        assert!(
            content.contains("approve_permission"),
            "generated.ts 未包含命令: {}",
            content
        );
        assert!(
            content.contains("set_ignore_cursor_events"),
            "generated.ts 未包含 set_ignore_cursor_events: {}",
            content
        );
    }
}
```

- [ ] **Step 4: 运行测试验证通过 + 前端集成**

Run: `cargo test --manifest-path src-tauri/Cargo.toml`

预期：
- `commands::window_control::tests::test_set_window_size_rejects_zero` PASS
- `commands::window_control::tests::test_set_window_size_rejects_nan` PASS
- `commands::window_control::tests::test_set_window_position_rejects_nan` PASS
- `commands::window_control::tests::test_commands_report_missing_window` PASS
- `specta_export::tests::generate_typescript_bindings` 重新生成并 PASS，`generated.ts` 包含新命令

在 `src/lib/tauri.ts` 末尾追加：

```ts
export async function invokeSetIgnoreCursorEvents(enabled: boolean): Promise<void> {
  return invoke("set_ignore_cursor_events", { windowLabel: "island", enabled });
}

export async function invokeSetWindowSize(width: number, height: number): Promise<void> {
  return invoke("set_window_size", { windowLabel: "island", width, height });
}

export async function invokeSetWindowPosition(x: number, y: number): Promise<void> {
  return invoke("set_window_position", { windowLabel: "island", x, y });
}
```

修改 `src/views/NotchView.vue`，在 `<script setup>` 的 onMounted / onUnmounted 周围追加鼠标 hit-test 穿透逻辑。完整替换 `<script setup>` 为：

```ts
import { computed, onMounted, onUnmounted, watch } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useSessionsStore } from "@/stores/sessions";
import { useNotchStore } from "@/stores/notch";
import {
  onSessionsUpdated,
  invokeSetIgnoreCursorEvents,
  invokeSetWindowSize,
} from "@/lib/tauri";
import StatusDot from "@/components/common/StatusDot.vue";
import SessionCard from "@/components/session-card/SessionCard.vue";
import NotchShape from "@/components/notch/NotchShape.vue";

const sessions = useSessionsStore();
const notch = useNotchStore();
let collapseTimer: number | null = null;
let unlisten: (() => void) | null = null;
let hitTestEl: SVGPathElement | null = null;

const sessionCount = computed(() => sessions.list.length);
const countLabel = computed(
  () => `${sessionCount.value} session${sessionCount.value !== 1 ? "s" : ""}`,
);

function hitTest(clientX: number, clientY: number): boolean {
  if (!hitTestEl) return true;
  const rect = hitTestEl.getBoundingClientRect();
  const localX = clientX - rect.left;
  const localY = clientY - rect.top;
  const point = new DOMPoint(localX, localY);
  const svg = hitTestEl.ownerSVGElement;
  if (!svg) return true;
  const ctm = hitTestEl.getScreenCTM();
  if (!ctm) return true;
  const transformed = point.matrixTransform(ctm.inverse());
  return hitTestEl.isPointInFill(transformed);
}

function onGlobalPointerMove(e: PointerEvent) {
  const inside = hitTest(e.clientX, e.clientY);
  void invokeSetIgnoreCursorEvents(!inside).catch(() => {});
}

function onEnter() {
  if (collapseTimer !== null) {
    window.clearTimeout(collapseTimer);
    collapseTimer = null;
  }
  void notch.animateExpanded(true);
}

function onLeave() {
  collapseTimer = window.setTimeout(() => {
    void notch.animateExpanded(false);
  }, 300);
}

async function onCloseClick(e: Event) {
  e.stopPropagation();
  await getCurrentWindow().hide();
}

watch(
  () => ({ w: notch.geometry.width, h: notch.geometry.height }),
  async ({ w, h }) => {
    await invokeSetWindowSize(w, h);
  },
);

onMounted(async () => {
  notch.setExpanded(false);
  await invokeSetWindowSize(notch.geometry.width, notch.geometry.height);
  await invokeSetIgnoreCursorEvents(true);

  hitTestEl = document.querySelector<SVGPathElement>(".notch-svg path");
  window.addEventListener("pointermove", onGlobalPointerMove);

  await sessions.refresh();
  unlisten = await onSessionsUpdated(() => {
    void sessions.refresh();
  });
});

onUnmounted(() => {
  if (unlisten) unlisten();
  if (collapseTimer !== null) window.clearTimeout(collapseTimer);
  window.removeEventListener("pointermove", onGlobalPointerMove);
});
```

- [ ] **Step 5: Commit**

运行完整测试确保无回归：

```bash
cargo test --manifest-path src-tauri/Cargo.toml
npm run test
npm run build
```

预期：全绿。

```bash
git add src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/src/commands/window_control.rs src-tauri/src/commands/mod.rs src-tauri/src/main.rs src-tauri/src/specta_export.rs src/types/generated.ts src/lib/tauri.ts src/views/NotchView.vue
git commit -m "[feat|backend|window][公共]cursor 穿透与窗口尺寸命令"
```

---

## Task 9: `AppEvent` 总线与事件名规范化

**Files:**
- Modify: `src-tauri/src/app_state.rs`
- Modify: `src-tauri/src/main.rs`
- Modify: `src-tauri/src/hook/pipe_server.rs`
- Modify: `src-tauri/src/commands/approval.rs`
- Modify: `src/lib/tauri.ts`

- [ ] **Step 1: 写失败测试 — `AppEvent` 枚举与事件名映射**

替换 `src-tauri/src/app_state.rs` 完整内容为：

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::session::store::SessionStore;
use crate::hook::pipe_server::HookServer;
use crate::sound::manager::SoundManager;

/// 应用内事件总线载荷。
///
/// 所有后端子系统通过 `event_tx.send(AppEvent::...)` 广播状态变化，
/// 由 `main.rs` 的订阅协程转发为规范化的 Tauri emit 事件名。
#[derive(Debug, Clone)]
pub enum AppEvent {
    SessionsUpdated,
}

impl AppEvent {
    /// 将事件映射到规范化的 Tauri emit 名称：`codeisland:<domain>:<action>`。
    pub fn topic(&self) -> &'static str {
        match self {
            AppEvent::SessionsUpdated => "codeisland:sessions:updated",
        }
    }
}

pub struct AppState {
    pub store: Arc<Mutex<SessionStore>>,
    pub hook_server: Arc<HookServer>,
    pub sound: Arc<Mutex<SoundManager>>,
    pub event_tx: tokio::sync::broadcast::Sender<AppEvent>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sessions_updated_topic() {
        assert_eq!(
            AppEvent::SessionsUpdated.topic(),
            "codeisland:sessions:updated"
        );
    }

    #[tokio::test]
    async fn test_broadcast_delivers_event() {
        let (tx, mut rx) = tokio::sync::broadcast::channel::<AppEvent>(4);
        tx.send(AppEvent::SessionsUpdated).unwrap();
        let received = rx.recv().await.unwrap();
        assert_eq!(received.topic(), "codeisland:sessions:updated");
    }
}
```

- [ ] **Step 2: 运行测试验证失败**

Run: `cargo test --manifest-path src-tauri/Cargo.toml app_state::tests`

预期：
- 编译其它模块失败——`HookServer::new` 依然签名为 `broadcast::Sender<String>`，与 `AppState.event_tx: broadcast::Sender<AppEvent>` 不匹配。
- `main.rs` / `pipe_server.rs` 报类型不匹配。

- [ ] **Step 3: 实现 — 传导 AppEvent 到 pipe_server 与 main.rs**

修改 `src-tauri/src/hook/pipe_server.rs` 的 `HookServer`、`handle_connection` 签名：将所有 `tokio::sync::broadcast::Sender<String>` 改为 `tokio::sync::broadcast::Sender<crate::app_state::AppEvent>`，并把 `let _ = event_tx.send(session_id.clone());` 改为 `let _ = event_tx.send(crate::app_state::AppEvent::SessionsUpdated);`。

完整替换为：

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::windows::named_pipe::{NamedPipeServer, ServerOptions};
use log::{info, warn, error};
use crate::app_state::AppEvent;
use crate::hook::protocol::{HookEvent, HookResponse};
use crate::session::store::SessionStore;

pub const PIPE_NAME: &str = r"\\.\pipe\codeisland";

pub struct HookServer {
    store: Arc<Mutex<SessionStore>>,
    pending: Arc<Mutex<std::collections::HashMap<String, tokio::sync::oneshot::Sender<HookResponse>>>>,
    event_tx: tokio::sync::broadcast::Sender<AppEvent>,
}

impl HookServer {
    pub fn new(
        store: Arc<Mutex<SessionStore>>,
        event_tx: tokio::sync::broadcast::Sender<AppEvent>,
    ) -> Self {
        Self {
            store,
            pending: Arc::new(Mutex::new(std::collections::HashMap::new())),
            event_tx,
        }
    }

    pub async fn respond(&self, session_id: &str, response: HookResponse) -> bool {
        let mut pending = self.pending.lock().await;
        if let Some(tx) = pending.remove(session_id) {
            let _ = tx.send(response);
            true
        } else {
            warn!("[PipeServer] 无待审批连接: {}", session_id);
            false
        }
    }

    pub async fn start(&self) {
        info!("[PipeServer] 启动 Named Pipe: {}", PIPE_NAME);

        loop {
            let server = match ServerOptions::new()
                .first_pipe_instance(false)
                .create(PIPE_NAME)
            {
                Ok(s) => s,
                Err(e) => {
                    error!("[PipeServer] 创建 pipe 失败: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    continue;
                }
            };

            if let Err(e) = server.connect().await {
                error!("[PipeServer] 等待连接失败: {}", e);
                continue;
            }

            let store = self.store.clone();
            let pending = self.pending.clone();
            let event_tx = self.event_tx.clone();

            tokio::spawn(async move {
                handle_connection(server, store, pending, event_tx).await;
            });
        }
    }
}

async fn handle_connection(
    mut stream: NamedPipeServer,
    store: Arc<Mutex<SessionStore>>,
    pending: Arc<Mutex<std::collections::HashMap<String, tokio::sync::oneshot::Sender<HookResponse>>>>,
    event_tx: tokio::sync::broadcast::Sender<AppEvent>,
) {
    let mut buf = vec![0u8; 65536];
    let n = match stream.read(&mut buf).await {
        Ok(0) => return,
        Ok(n) => n,
        Err(e) => {
            warn!("[PipeServer] 读取失败: {}", e);
            return;
        }
    };

    let event: HookEvent = match serde_json::from_slice(&buf[..n]) {
        Ok(e) => e,
        Err(e) => {
            warn!("[PipeServer] JSON 解析失败: {}", e);
            return;
        }
    };

    let session_id = event.session_id.clone();
    let expects_response = event.expects_response();

    {
        let mut store = store.lock().await;
        store.process_hook_event(&event);
    }

    let _ = event_tx.send(AppEvent::SessionsUpdated);

    if expects_response {
        let (tx, rx) = tokio::sync::oneshot::channel::<HookResponse>();
        {
            let mut p = pending.lock().await;
            p.insert(session_id.clone(), tx);
        }

        info!("[PipeServer] 等待用户审批: {}", &session_id[..8.min(session_id.len())]);

        match tokio::time::timeout(std::time::Duration::from_secs(300), rx).await {
            Ok(Ok(response)) => {
                let json = serde_json::to_vec(&response).unwrap_or_default();
                let _ = stream.write_all(&json).await;
                info!(
                    "[PipeServer] 已回复审批: {} → {}",
                    &session_id[..8.min(session_id.len())],
                    response.decision
                );
            }
            _ => {
                warn!("[PipeServer] 审批超时: {}", &session_id[..8.min(session_id.len())]);
                let mut p = pending.lock().await;
                p.remove(&session_id);
            }
        }
    } else {
        let _ = stream.write_all(b"ok\n").await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_name_constant() {
        assert_eq!(PIPE_NAME, r"\\.\pipe\codeisland");
    }

    #[test]
    fn test_pipe_server_is_windows_only() {
        let src = include_str!("pipe_server.rs");
        assert!(!src.contains("UnixListener"), "Unix 分支残留 UnixListener");
        assert!(!src.contains("cfg(not(target_os"), "跨平台 cfg 残留");
        assert!(!src.contains("/tmp/codeisland.sock"), "Unix socket 路径残留");
    }
}
```

修改 `src-tauri/src/commands/approval.rs`，将内部手工 `app.emit("sessions-updated", ())` 改走 `event_tx.send(AppEvent::SessionsUpdated)`，完整替换为：

```rust
use tauri::State;
use crate::app_state::{AppEvent, AppState};
use crate::hook::protocol::HookResponse;

#[tauri::command]
#[specta::specta]
pub async fn approve_permission(
    state: State<'_, AppState>,
    session_id: String,
) -> Result<bool, String> {
    state
        .hook_server
        .respond(&session_id, HookResponse::allow())
        .await;

    let mut store = state.store.lock().await;
    let result = store.process_approval(&session_id);
    drop(store);

    let _ = state.event_tx.send(AppEvent::SessionsUpdated);
    Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn deny_permission(
    state: State<'_, AppState>,
    session_id: String,
    reason: Option<String>,
) -> Result<bool, String> {
    let reason_text = reason.unwrap_or_else(|| "Denied by user via CodeIsland".into());
    state
        .hook_server
        .respond(&session_id, HookResponse::deny(&reason_text))
        .await;

    let mut store = state.store.lock().await;
    let result = store.process_denial(&session_id);
    drop(store);

    let _ = state.event_tx.send(AppEvent::SessionsUpdated);
    Ok(result)
}
```

修改 `src-tauri/src/main.rs`：
1. import `AppEvent`
2. `broadcast::channel::<String>` → `broadcast::channel::<AppEvent>`
3. 订阅协程将接收到的 `AppEvent` 按 `topic()` 分派

替换 `main.rs` 的 `fn main()` 完整内容为：

```rust
fn main() {
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            let (event_tx, _) =
                tokio::sync::broadcast::channel::<app_state::AppEvent>(256);

            let store = Arc::new(Mutex::new(session::store::SessionStore::new()));
            let hook_server = Arc::new(hook::pipe_server::HookServer::new(
                store.clone(),
                event_tx.clone(),
            ));
            let sound = Arc::new(Mutex::new(sound::manager::SoundManager::new()));

            let state = app_state::AppState {
                store: store.clone(),
                hook_server: hook_server.clone(),
                sound: sound.clone(),
                event_tx: event_tx.clone(),
            };

            app.manage(state);

            let script_bytes = include_bytes!("../resources/codeisland-state.py");
            hook::installer::install_if_needed(script_bytes);

            let server = hook_server.clone();
            tauri::async_runtime::spawn(async move {
                server.start().await;
            });

            let store_for_jsonl = store.clone();
            let event_tx_for_jsonl = event_tx.clone();
            tauri::async_runtime::spawn(async move {
                let mut watcher = session::jsonl_watcher::JsonlWatcher::new(
                    store_for_jsonl,
                    event_tx_for_jsonl,
                );
                loop {
                    watcher.scan_all().await;
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            });

            let store_for_proc = store.clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    let mut store = store_for_proc.lock().await;
                    store.remove_ended_sessions();
                }
            });

            let app_handle = app.handle().clone();
            let mut event_rx = event_tx.subscribe();
            tauri::async_runtime::spawn(async move {
                while let Ok(event) = event_rx.recv().await {
                    let _ = app_handle.emit(event.topic(), ());
                }
            });

            if let Some(window) = app.get_webview_window("island") {
                if let Ok(Some(monitor)) = window.current_monitor() {
                    let screen_size = monitor.size();
                    let scale = monitor.scale_factor();
                    let window_width = 400.0;
                    let x = (screen_size.width as f64 / scale - window_width) / 2.0;
                    let _ = window.set_position(tauri::Position::Logical(
                        tauri::LogicalPosition::new(x, 0.0),
                    ));
                }
            }

            let show_item =
                MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(
                app,
                "quit",
                "退出 Code Island",
                true,
                None::<&str>,
            )?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            let tray_icon = Image::from_bytes(include_bytes!("../icons/32x32.png"))
                .expect("无法加载托盘图标");

            let _tray = TrayIconBuilder::new()
                .icon(tray_icon)
                .tooltip("Code Island")
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("island") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        hook::installer::uninstall();
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::DoubleClick { .. } = event {
                        if let Some(window) =
                            tray.app_handle().get_webview_window("island")
                        {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::session::get_sessions,
            commands::session::get_session_count,
            commands::session::send_to_terminal,
            commands::approval::approve_permission,
            commands::approval::deny_permission,
            commands::settings::get_sound_enabled,
            commands::settings::set_sound_enabled,
            commands::window_control::set_ignore_cursor_events,
            commands::window_control::set_window_size,
            commands::window_control::set_window_position,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Code Island 失败");
}
```

> 注意：`main.rs` 顶部的 `use` 行与 `mod` 声明需保持 Task 5 末的状态（含 `mod specta_export;`）。若 `jsonl_watcher::JsonlWatcher::new` 当前签名仍接 `broadcast::Sender<String>`，需同步把 `src-tauri/src/session/jsonl_watcher.rs` 里的类型也改为 `broadcast::Sender<AppEvent>` 且 `send(AppEvent::SessionsUpdated)`。改法：用编辑器搜索 `broadcast::Sender<String>` 与 `event_tx.send(` 替换，保持语义一致。Stage 0 该文件仅此两处类型需要调。

修改前端 `src/lib/tauri.ts`，将 `listen("sessions-updated", ...)` 改为规范化事件名：

```ts
export async function onSessionsUpdated(callback: () => void): Promise<UnlistenFn> {
  return listen("codeisland:sessions:updated", callback);
}
```

- [ ] **Step 4: 运行测试验证通过**

Run: `cargo test --manifest-path src-tauri/Cargo.toml`

预期：所有测试 PASS，包括新增的 `app_state::tests::test_sessions_updated_topic`、`app_state::tests::test_broadcast_delivers_event`。

Run:

```bash
npm run test
npm run build
```

预期：前端测试全绿；vite build 无错。

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/app_state.rs src-tauri/src/main.rs src-tauri/src/hook/pipe_server.rs src-tauri/src/commands/approval.rs src-tauri/src/session/jsonl_watcher.rs src/lib/tauri.ts
git commit -m "[feat|backend|state][公共]AppEvent 总线与事件名规范化"
```

---

## Task 10: Stage 1 并行契约文档

**Files:**
- Create: `docs/superpowers/specs/2026-04-17-windows-ui-alignment-contracts.md`
- Modify: `~/.claude/Vault/.meta/pending-docs.json`（更新记录）

- [ ] **Step 1: 写失败"文档存在性"测试**

该 Task 的验证以文档结构完整度为核心，不涉及代码单元测试；Step 1 退化为"创建前先确认目标路径不存在，执行后验证关键章节齐全"。

Run 前置检查：

```bash
test ! -f docs/superpowers/specs/2026-04-17-windows-ui-alignment-contracts.md && echo "OK: 文档尚未存在"
```

预期：输出 `OK: 文档尚未存在`。

- [ ] **Step 2: （N/A — 无代码测试）**

跳过。此 Task 的"失败 → 通过"通过文档 lint 体现（Step 4 用 grep 校验必选章节全部命中）。

- [ ] **Step 3: 编写契约文档**

创建 `docs/superpowers/specs/2026-04-17-windows-ui-alignment-contracts.md`：

```markdown
# Code Island Windows UI 对齐 · Stage 1 并行契约文档

- 日期：2026-04-17
- 关联设计：`docs/superpowers/specs/2026-04-17-windows-ui-alignment-design.md`
- 作用：冻结 Stage 1 的八个并行 agent 之间的接口 / 文件所有权 / 事件命名 / 错误类型，任何跨边界改动必须回到主会话走 Stage 2 整合路径。

## 1. Agent 清单与 worktree 绝对路径

| Agent | 分支 | worktree 绝对路径 |
|---|---|---|
| A. session-card | `feat/session-card` | `/Users/wj/Work/OpenSource/CodeIsland-win/.worktrees/agent-session-card` |
| B. tool-results | `feat/tool-results` | `/Users/wj/Work/OpenSource/CodeIsland-win/.worktrees/agent-tool-results` |
| C. settings | `feat/settings` | `/Users/wj/Work/OpenSource/CodeIsland-win/.worktrees/agent-settings` |
| D. pixel-art | `feat/pixel-art` | `/Users/wj/Work/OpenSource/CodeIsland-win/.worktrees/agent-pixel-art` |
| E. buddy | `feat/buddy` | `/Users/wj/Work/OpenSource/CodeIsland-win/.worktrees/agent-buddy` |
| F. usage | `feat/usage` | `/Users/wj/Work/OpenSource/CodeIsland-win/.worktrees/agent-usage` |
| G. presets | `feat/presets` | `/Users/wj/Work/OpenSource/CodeIsland-win/.worktrees/agent-presets` |
| H. live-edit | `feat/live-edit` | `/Users/wj/Work/OpenSource/CodeIsland-win/.worktrees/agent-live-edit` |

派发 agent 时：
1. prompt 第一行写明对应 worktree 绝对路径
2. agent 的首个动作必须 `pwd`，与预期路径逐字比对，不匹配立即 abort
3. prompt 末尾引用本文件与设计文档路径

## 2. 文件所有权边界

| Agent | 独占路径（可读可写） |
|---|---|
| A | `src/components/session-card/**` |
| B | `src/components/tool-results/**`、`src/components/common/MarkdownRenderer.vue` |
| C | `src/components/settings/**`、`src/views/SettingsView.vue`、`src-tauri/src/settings/**`、`src-tauri/src/autostart/**`、`src-tauri/src/screens/**` |
| D | `src/components/pixel/**`、`src/styles/pixel.css` |
| E | `src/views/BuddyCardView.vue`、`src-tauri/src/buddy/**` |
| F | `src/views/DailyReportCardView.vue`、`src-tauri/src/usage/**` |
| G | `src/views/LaunchPresetsView.vue`、`src/components/preset-editor/**`、`src-tauri/src/presets/**` |
| H | `src/views/NotchLiveEditView.vue`、`src/composables/useDragResize.ts`、`src-tauri/src/commands/window_control.rs`（扩展点） |

## 3. Stage 1 禁改共享文件清单

任何 agent 禁止改动：
- `src-tauri/src/main.rs`
- `src-tauri/src/app_state.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/lib.rs`（若存在）
- `src-tauri/Cargo.toml`
- `src/App.vue`
- `src/main.ts`
- `src/router/index.ts`（Stage 2 初始化）
- `src/types/generated.ts`
- `package.json`
- `CLAUDE.md`

若必须改共享文件，agent 在 PR 描述中显式列出改动目标文件与原因，主会话 Stage 2 整合时合并。

## 4. TypeScript 类型契约

所有 agent 必须 `import from '@/types/generated'`，禁止在 agent 自有代码中手写 `SessionSummary` / `Settings` / `Buddy` / `UsageReport` / `LaunchPreset` / `ScreenInfo` 等 domain 类型。UI 侧附加的 view-only 结构放 `src/types/ui.ts`。

类型字段定义以设计文档 §11.2 为准：

- `SessionSummary` —— `session_id / project_name / cwd / phase / needs_attention / last_message / tool_name / tool_input / tool_output / tool_status / launcher / pixel_cat_enabled / last_activity_ts`
- `SessionPhase` —— `"idle" | "processing" | "waitingForApproval" | "waitingForInput" | "compacting" | "ended"`
- `Settings` —— 见设计文档 §9.1 全字段
- `Buddy` / `BuddyStats` / `Rarity` —— 见设计文档 §8.2
- `UsageReport` / `UsageBucket` —— 见设计文档 §8.5
- `LaunchPreset` —— 见设计文档 §9.3
- `ScreenInfo` —— `id / name / width / height / is_primary`

## 5. Pinia Store 只读 API

| Store | 返回字段 | 方法 |
|---|---|---|
| `useSessionsStore` | `list` / `byId` / `hasAttention` / `overallStatus` | `refresh` / `approve` / `deny` / `sendToTerminal` |
| `useSettingsStore` | `value` | `load` / `update(patch)` |
| `useBuddyStore` | `current` / `roster` / `completedSessions` | `load` / `switchBuddy(id)` |
| `useUsageStore` | `report` | `load` / `refresh` |
| `usePresetsStore` | `list` | `load` / `save(preset)` / `remove(id)` / `launch(id)` |
| `useNotchStore` | `expanded` / `geometry` / `path` / `viewBox` | `setExpanded` / `animateExpanded` / `animateTo` / `setGeometry` |

Agent 不得定义同名 store。需要新增 domain 时，在 `src/stores/` 下以单数名创建，不得在组件局部重复管理 session 列表等全局状态。

## 6. 公共组件 Props 冻结

| 组件 | Props |
|---|---|
| `ActionButton` | `icon? / variant?: 'default'|'primary'|'danger'|'ghost' / active? / disabled? / loading?` |
| `StatusDot` | `status: 'idle'|'processing'|'attention'|'success'|'error' / size?: 'sm'|'md'|'lg'` |
| `Toggle` | `modelValue: boolean / disabled? / label?` |
| `Picker<T>` | `modelValue: T / options: {value: T; label: string}[] / disabled?` |
| `MarkdownRenderer` | `source: string / inline? / maxHeight?: number` |
| `NeonPixelCat` | `state: AnimationState / size? / tint?` |
| `BuddyASCII` | `frames: string[] / size? / colored?` |

任何 agent 修改以上 Props 必须在 PR 中注明"契约变更"，主会话 Stage 2 统一合并时评审向后兼容性。

## 7. 事件命名规范

统一 `codeisland:<domain>:<action>`，由后端 `AppEvent::topic()` 返回。Stage 1 新增事件必须扩展 `AppEvent` enum（由主会话 Stage 2 整合），不得 agent 自行 `app.emit("foo")`。

Stage 0 已有事件：
- `codeisland:sessions:updated`

Stage 1 预期新增（各 agent 必须在 PR 描述中声明需要主会话追加的 AppEvent 变体）：
- `codeisland:settings:changed`
- `codeisland:buddy:unlocked`
- `codeisland:usage:updated`
- `codeisland:presets:updated`
- `codeisland:notch:position-changed`

## 8. Rust 统一错误类型

Stage 2 将为 commands 层引入统一 `AppError`（见设计文档 §11.7）。Stage 1 期间：
- 新增命令签名 `Result<T, String>`，错误消息用人可读中文描述 + 识别性前缀
- 前缀推荐：`"[hook_install]"`、`"[registry]"`、`"[preset_launch]"`、`"[settings]"`、`"[io]"`、`"[json]"`，便于 Stage 2 按前缀映射为 `AppError` 变体
- 禁止在 Stage 1 agent 自行定义新的 `#[derive(thiserror::Error)]` 枚举

## 9. 设计 Token

Stage 1 所有 agent 引用 `src/styles/theme.css` 的 CSS 变量，禁止硬编码颜色/圆角值。关键变量：

- 颜色：`--bg-notch`、`--text-primary/secondary/tertiary`、`--accent-green/blue/yellow/red/purple/orange`
- 几何：`--notch-collapsed-w/h`、`--notch-expanded-w/h`、`--notch-top-radius`、`--notch-bottom-radius`
- 动画：`--ease-notch`、`--dur-notch: 280ms`、`--dur-fast: 150ms`
- 字体：`--font-sans`、`--font-mono: "Cascadia Code"`、`--font-pixel`

Stage 1 期间 D（pixel-art）负责 `src/styles/pixel.css` 专属 token；其它 token 若需新增，在 PR 描述标注并由 Stage 2 主会话合并。

## 10. Agent 提交规范

- 提交格式 `[类型|模块|功能][公共]中文说明`
- 每 agent 独立分支，最终由主会话按 spec §14.3 顺序合并
- 合并后如某 agent 存在冲突，主会话负责解决，agent 不再返工
- 每次 push 前跑 `cargo test` / `npm run test` / `npm run build`；测试红则不提交

## 11. 验收门槛

Stage 1 每 agent 完成判据：
1. 所属 worktree 内 `cargo test` / `npm run test` 全绿
2. `npm run build` 成功
3. 与共享文件无冲突（`git diff origin/master -- <禁改清单>` 为空）
4. PR 描述完整回答：使用了哪些新增类型 / Store / 事件，是否需要主会话 Stage 2 追加
5. 人工 smoke 测试：在 Windows 环境启动应用，验证本 agent 功能（引用 spec §14.4 对应条目）

Stage 2 主会话负责解冻 `main.rs` / `app_state.rs` 等共享文件，补齐 `AppEvent` / 路由注册 / 命令 invoke 挂载。
```

- [ ] **Step 4: 验证文档完整**

Run:

```bash
grep -c "^## " docs/superpowers/specs/2026-04-17-windows-ui-alignment-contracts.md
```

预期：输出 `11`（11 个一级章节）。

Run:

```bash
grep -E "worktrees/agent-(session-card|tool-results|settings|pixel-art|buddy|usage|presets|live-edit)" docs/superpowers/specs/2026-04-17-windows-ui-alignment-contracts.md | wc -l
```

预期：`8` 行（每个 agent worktree 路径各出现一次）。

更新 `~/.claude/Vault/.meta/pending-docs.json`：先用 Read 读当前内容，解析为 JSON 数组，push：

```json
{
  "path": "/Users/wj/Work/OpenSource/CodeIsland-win/docs/superpowers/specs/2026-04-17-windows-ui-alignment-contracts.md",
  "type": "spec",
  "context": "Code Island Windows UI 对齐 Stage 1 并行契约（文件所有权/类型/事件/Token）",
  "created": "2026-04-17T00:00:00+08:00"
}
```

若文件不存在或 JSON 解析失败，按 `[]` 初始化后 push。

- [ ] **Step 5: Commit**

```bash
git add docs/superpowers/specs/2026-04-17-windows-ui-alignment-contracts.md
git commit -m "[docs|spec|contracts][公共]Stage 1 并行契约文档"
```

---

## Task 11: CLAUDE.md 清理 + 删除 dev.sh

**Files:**
- Modify: `CLAUDE.md`
- Delete: `scripts/dev.sh`

- [ ] **Step 1: 写失败"静态合规性"测试**

本 Task 校验点在于 `CLAUDE.md` 不再描述跨平台、`scripts/dev.sh` 不再存在。用 bash 做静态检查。

Run 前置：

```bash
test -f scripts/dev.sh && grep -c "macOS" CLAUDE.md
```

预期：输出 `dev.sh` 存在且 `grep -c "macOS"` 输出 ≥ 1（当前至少 2 处出现 macOS 描述）。

- [ ] **Step 2: （静态验证）记录失败状态**

Run：

```bash
grep -c "macOS\|Unix\|Unix Socket\|target_os = \"windows\"" CLAUDE.md
```

预期：输出 > 0（当前至少 4 处跨平台描述）。

- [ ] **Step 3: 实现 — 替换 CLAUDE.md，删除 dev.sh**

替换 `CLAUDE.md` 完整内容为：

```markdown
# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

Code Island 是一个 Dynamic Island 风格的 Windows 桌面悬浮窗应用，用于实时监控和管理多个 Claude Code 会话。通过 Claude Code Hooks 机制接收事件，在显示器顶部以虚拟刘海形态显示会话状态、权限审批、工具调用等信息。

**目标平台：Windows 10+ 专属。** 非 Windows 分支已在 Stage 0 清理；如需 macOS 体验，请参考独立的 macOS 原生工程。

## 技术栈

- **后端**：Rust + Tauri 2（tokio 异步运行时）
- **前端**：Vue 3 `<script setup>` + Pinia 2 + Vue Router 4，Vite 6 构建，Vitest 2 单测
- **类型共享**：`specta` + `tauri-specta` 从 Rust struct 自动生成 `src/types/generated.ts`
- **IPC**：Windows Named Pipe `\\.\pipe\codeisland`
- **Hook 桥接**：Python 脚本 `src-tauri/resources/codeisland-state.py`，编译时通过 `include_bytes!` 嵌入二进制

## 常用命令

```powershell
# 开发
.\scripts\dev.ps1                   # 或直接: npm run tauri dev

# 前端构建
npm run build                       # vue-tsc --noEmit + vite build

# 前端单测
npm run test                        # Vitest run
npm run test:watch                  # Vitest watch

# Rust 单测
cd src-tauri; cargo test

# 完整构建（生成 exe / msi / nsis）
npx tauri build

# 重新生成前端类型
cd src-tauri; cargo test specta_export::tests::generate_typescript_bindings
```

## 架构

### 数据流

```
Claude Code 进程
  → Hook 触发 Python 脚本 (codeisland-state.py)
    → 通过 Named Pipe 发送 JSON 事件
      → Rust HookServer 接收并更新 SessionStore
        → broadcast<AppEvent> 通知订阅者
          → main.rs 订阅协程 emit 规范化事件 (codeisland:<domain>:<action>)
            → 前端 Pinia store 刷新 → Vue 组件重渲染
```

### 后端模块 (`src-tauri/src/`)

- **`hook/`** — IPC 服务与 Hook 安装
  - `pipe_server.rs`: Named Pipe IPC 服务器（常量 `PIPE_NAME = \\.\pipe\codeisland`）
  - `installer.rs`: 启动时自动安装 hook 脚本到 `%USERPROFILE%\.claude\hooks\` 并注册到 `settings.json`
  - `protocol.rs`: `HookEvent` / `HookResponse` / `PendingPermission` 协议定义
- **`session/`** — 会话状态管理
  - `phase.rs`: 会话阶段状态机（Idle → Processing → WaitingForApproval → Compacting → Ended），含完整转换规则
  - `state.rs`: `SessionState` 单会话完整状态，`ConversationInfo` 对话摘要
  - `store.rs`: `SessionStore` 多会话管理，事件处理、审批流程、摘要生成
  - `jsonl_watcher.rs`: 定时扫描 `%USERPROFILE%\.claude\projects\<encoded-cwd>\<session-id>.jsonl` 提取对话信息
  - `process_scanner.rs`: 基于 Win32 `OpenProcess` 的进程存活检查
- **`terminal/`** — Win32 clipboard + SendInput 终端写入
- **`sound/`** — 音效管理（rodio，WAV 文件通过 `include_bytes!` 嵌入）
- **`commands/`** — Tauri 命令层：`session` / `approval` / `settings` / `window_control`
- **`app_state.rs`** — `AppState` 全局依赖容器 + `AppEvent` 枚举
- **`specta_export.rs`** — `#[cfg(test)]` 入口，`cargo test` 即生成 `src/types/generated.ts`

### 前端结构 (`src/`)

Vue 3 `<script setup>` + Pinia 单例 store。入口 `main.ts` 挂载 `App.vue` 下的 `NotchView.vue`。

- `App.vue` — 根组件
- `views/NotchView.vue` — 虚拟刘海主容器，hover 展开/收起
- `components/notch/NotchShape.vue` — SVG path 外形容器
- `components/session-card/*` — 会话卡片、审批按钮、AskUserQuestion 选项
- `components/common/StatusDot.vue` — 状态指示点
- `stores/sessions.ts` — 会话列表与审批动作
- `stores/notch.ts` — 刘海展开状态与几何插值
- `utils/notch-shape.ts` — SVG path 生成与几何插值
- `lib/tauri.ts` — Tauri `invoke` 封装，统一通过 `@/types/generated` 保持类型
- `types/generated.ts` — 由 specta 自动生成，禁止手改

### 窗口特性

- 透明、无边框、始终置顶、不显示在任务栏、不可调整大小
- 初始尺寸由 `src/utils/notch-shape.ts` `COLLAPSED = {220×32}` 定义
- 启动时顶部居中定位，支持拖拽（`data-tauri-drag-region`）
- 通过 `set_ignore_cursor_events` 实现 SVG path 外区域鼠标穿透

## CI

GitHub Actions `build-windows.yml` 仅在 Windows 上运行：`cargo test` → `npm run test` → `npm run build` → `tauri build`，上传 exe / msi / nsis 产物。
```

删除 `scripts/dev.sh`：

```bash
rm scripts/dev.sh
```

- [ ] **Step 4: 验证通过**

Run：

```bash
grep -c "macOS\|Unix Socket\|cfg(not(target_os\|cfg(unix)" CLAUDE.md
test -f scripts/dev.sh; echo "dev.sh exists: $?"
```

预期：
- `grep -c` 输出 `0`
- `test -f scripts/dev.sh; echo` 输出 `dev.sh exists: 1`（1 表示不存在）

Run 再跑一次完整测试套件，确保改动不触发任何代码回归（CLAUDE.md 与 dev.sh 不影响运行时）：

```bash
cargo test --manifest-path src-tauri/Cargo.toml
npm run test
npm run build
```

预期：全绿。

- [ ] **Step 5: Commit**

```bash
git add CLAUDE.md
git rm scripts/dev.sh
git commit -m "[refactor|docs|claude_md][公共]移除跨平台说明"
```

---

## Self-Review

### Spec 覆盖核对（逐项）

| Spec §14.1 提交 | 对应 Task | 备注 |
|---|---|---|
| 1. 清理非 Windows IPC 分支 | Task 1 | pipe_server + process_scanner + Cargo.toml |
| 2. 路径简化为 Windows 专属 | Task 2 | installer.rs + python_candidates |
| 3. hook 脚本 Windows 化 | Task 3 | codeisland-state.py + pytest |
| 4. 前端引入 Vue 3 + Pinia | Task 4 | package.json + vite/vitest 配置 + App.vue |
| 5. 类型导出 specta | Task 5 | Cargo 依赖 + derive Type + specta_export.rs |
| 6. 现有组件迁移 Vue | Task 6 | 5 旧组件 → Vue，删除旧 TS |
| 7. 虚拟刘海 SVG 形状 + 动画 | Task 7 | notch-shape util + NotchShape.vue + 插值动画 |
| 8. cursor 穿透与窗口尺寸命令 | Task 8 | window_control.rs 三命令 + 前端 hit-test |
| 9. AppEvent 总线与事件名规范化 | Task 9 | AppEvent enum + topic 映射 |
| 10. Stage 1 并行契约文档 | Task 10 | contracts.md 11 章节 |
| 11. 移除跨平台说明 | Task 11 | CLAUDE.md 重写 + 删除 dev.sh |

**结论：** 11 个原子提交一一对应，无遗漏。

### Spec 其它章节覆盖

- §4.2 跨平台清理范围（pipe_server / process_scanner / terminal Unix stub / installer 路径 / Python / dev.sh / CLAUDE.md）—— 分布在 Task 1 / 2 / 3 / 11；**`terminal/` 的 Unix stub 文件**当前项目实际无独立 Unix 文件（`mod.rs` 仅 `pub mod writer/window_finder/process_tree`，writer.rs 已 Windows-only 实现），因此 Task 1 不涉及删除；若后续发现有 stub 需清理，由主会话补一次 hotfix commit。
- §4.3 前后端类型共享 —— Task 5 打通；Stage 1 agent 由 Task 10 契约锁定必须 `import from '@/types/generated'`。
- §4.4 事件与配置 —— 事件名规范化 Task 9 落地；配置/Buddy/Usage 持久化路径留给 Stage 1 settings/buddy/usage agent。
- §6.1–6.6 虚拟刘海 —— Task 7 覆盖 `clip-path` + 插值 + hover 展开；Task 8 补齐 `setIgnoreCursorEvents` 与 `setWindowSize`。
- §11 Stage 1 公共契约 —— Task 10 contracts.md 逐节落地。

### Placeholder 扫描

- 文件路径、提交信息、命令均为绝对具体。
- 所有 code block 为可直接 copy 的完整文件内容或追加片段，无 `TODO`/`TBD`/`...` 省略号，无"类似 Task N"等回指。
- Step 4 验证通过步骤均给出具体 `Run:` 命令与预期输出断言。

### 类型一致性核对

- `SessionSummary` 字段在 Task 5 `session/store.rs` / Task 6 `sessions.spec.ts` / `SessionCard.spec.ts` / `SessionCard.vue` / Task 10 契约文档中一致（`session_id / project_name / cwd / phase / needs_attention / last_message / tool_name / tool_input`）。Stage 1 设计文档 §7.6 扩展字段（`tool_output / tool_status / launcher / pixel_cat_enabled / last_activity_ts`）留给 Stage 1 session-card agent 补齐，Task 10 契约文档已明示。
- `NotchGeometry` 在 Task 7 util / store / NotchShape.vue / Task 8 NotchView.vue watch 中一致（`width / height / topCornerRadius / bottomCornerRadius`）。
- `AppEvent::SessionsUpdated` 与 topic `"codeisland:sessions:updated"` 在 Task 9 后端、Task 9 `src/lib/tauri.ts` 的 listen 调用、Task 10 契约文档中一致。
- Tauri 命令名 `set_ignore_cursor_events` / `set_window_size` / `set_window_position` 在 Task 8 `window_control.rs` / `main.rs` invoke_handler / `lib/tauri.ts` / `specta_export.rs` 中一致。
- `PIPE_NAME = \\.\pipe\codeisland` 在 Task 1 Rust、Task 3 Python 中一致。

**结论：** 无类型漂移。Stage 0 plan 自洽。
