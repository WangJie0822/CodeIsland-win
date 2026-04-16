# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

Code Island 是一个 Dynamic Island 风格的桌面悬浮窗应用，用于实时监控和管理多个 Claude Code 会话。通过 Claude Code Hooks 机制接收事件，在屏幕顶部显示会话状态、权限审批、工具调用等信息。

## 技术栈

- **后端**: Rust + Tauri 2（tokio 异步运行时）
- **前端**: 原生 TypeScript（无框架），Vite 6 构建
- **IPC**: Windows Named Pipe (`\\.\pipe\codeisland`) / Unix Socket (`/tmp/codeisland.sock`)
- **Hook 桥接**: Python 脚本 (`src-tauri/resources/codeisland-state.py`)，编译时通过 `include_bytes!` 嵌入二进制

## 常用命令

```bash
# 开发（Mac/Linux）
./scripts/dev.sh          # 或直接: npm run tauri dev

# 开发（Windows PowerShell）
.\scripts\dev.ps1         # 或直接: npm run tauri dev

# 前端构建
npm run build             # tsc + vite build

# Rust 测试
cd src-tauri && cargo test

# 完整构建（生成安装包）
npx tauri build
```

## 架构

### 数据流

```
Claude Code 进程
  → Hook 触发 Python 脚本 (codeisland-state.py)
    → 通过 Named Pipe / Unix Socket 发送 JSON 事件
      → Rust HookServer 接收并更新 SessionStore
        → broadcast channel 通知前端
          → 前端 Store 刷新渲染 Island UI
```

### 后端模块 (`src-tauri/src/`)

- **`hook/`** — IPC 服务与 Hook 安装
  - `pipe_server.rs`: Named Pipe (Windows) / Unix Socket (macOS) 双模 IPC 服务器
  - `installer.rs`: 启动时自动安装 hook 脚本到 `~/.claude/hooks/` 并注册到 `~/.claude/settings.json`
  - `protocol.rs`: `HookEvent` / `HookResponse` / `PendingPermission` 协议定义
- **`session/`** — 会话状态管理
  - `phase.rs`: 会话阶段状态机（Idle → Processing → WaitingForApproval → Compacting → Ended），含完整转换规则
  - `state.rs`: `SessionState` 单会话完整状态，`ConversationInfo` 对话摘要
  - `store.rs`: `SessionStore` 多会话管理，事件处理、审批流程、摘要生成
  - `jsonl_watcher.rs`: 定时扫描 `~/.claude/projects/<encoded-cwd>/<session-id>.jsonl` 提取对话信息
  - `process_scanner.rs`: 进程存活检查（Windows 用 Win32 API，Unix 用 ps/kill -0）
- **`terminal/`** — 终端交互（Windows 用 Win32 clipboard+SendInput，Unix 为 stub）
- **`sound/`** — 音效管理（rodio，WAV 文件通过 `include_bytes!` 嵌入）
- **`commands/`** — Tauri 命令层（`get_sessions`, `approve_permission`, `deny_permission`, `send_to_terminal`, `get/set_sound_enabled`）

### 前端结构 (`src/`)

原生 DOM 操作，无框架。`Store` 为单例状态管理（发布-订阅模式）。

- `store.ts`: 全局状态（sessions 列表、展开状态），通过 `subscribe()` 驱动 UI 更新
- `lib/events.ts`: Tauri `invoke` 封装和 `listen` 事件监听
- `components/Island.ts`: 主容器，鼠标 hover 展开/收起
- `components/SessionCard.ts`: 单会话卡片，展示状态和审批按钮
- `components/ApprovalButtons.ts`: Allow/Deny 按钮
- `components/AskUserOptions.ts`: AskUserQuestion 选项按钮

### 窗口特性

透明无边框、始终置顶、不显示在任务栏、不可调整大小、400×48 默认尺寸、居中显示。

## 平台差异

代码大量使用 `#[cfg(target_os = "windows")]` / `#[cfg(not(target_os = "windows"))]` 进行条件编译：

| 功能 | Windows | macOS/Linux |
|------|---------|-------------|
| IPC | Named Pipe | Unix Socket |
| 进程扫描 | Win32 ToolHelp32 | ps + kill -0 |
| 终端写入 | Win32 clipboard + SendInput | stub（未实现） |
| Python 检测 | python3 → python → py | python3 → python |

新增平台相关功能时必须同时处理两个分支。

## CI

GitHub Actions 仅在 Windows 上构建 (`build-windows.yml`)：运行 Rust 测试 → 构建 Tauri 应用 → 上传 exe/msi/nsis 产物。
