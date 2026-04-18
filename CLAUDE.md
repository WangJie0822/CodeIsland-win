# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

Code Island 是一个 Dynamic Island 风格的 Windows 桌面悬浮窗应用，用于实时监控和管理多个 Claude Code 会话。通过 Claude Code Hooks 机制接收事件，在显示器顶部以虚拟刘海形态显示会话状态、权限审批、工具调用等信息。

**目标平台：Windows 10+ 专属。** 非 Windows 分支已在 Stage 0 清理；其他操作系统不在本工程支持范围。

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

## specta 类型导出注意事项

`src/types/generated.ts` 由 `cargo test specta_export::tests::generate_typescript_bindings` 自动生成。向 Tauri command / 共享 struct 写代码时避开以下四类签名，否则 CI 会挂在 `cargo test` 或 `vue-tsc --noEmit` 阶段：

1. **Runtime 泛型**：`#[specta::specta]` 的命令函数**不可带 `<R: Runtime>` 泛型**，`collect_commands!` 无法推断类型（E0283）。用默认 `AppHandle`（`Wry`）即可；validator 若需单测可抽为纯函数
2. **BigInt 类型**：返回 `usize / u64 / i64` 触发 `BigIntForbidden`（TS 默认禁 bigint）。对外 API 明确收敛为 `u32 / i32 / f64`，用 `as u32` 显式转换
3. **`Option<serde_json::Value>` 字段**：specta 会生成递归 `JsonValue` 类型，下游 TS 消费端若用 `Map<K, Self>` 或 `Record<string, unknown>` prop 会触发 TS2589 深度实例化。消费端改用 `unknown`（自己断言）或显式 `new Map<K, V>()` 构造
4. **跨 feature 的 Rust stdlib 约束**：`Option<&String>` 不满足 `T: Default`，需 `.cloned().unwrap_or_default()`

详见 [[CodeIsland-win Stage 1-2 整合与 CI 收敛经验]]（Vault）。

## CI

GitHub Actions `build-windows.yml` 仅在 Windows 上运行：`npm ci` → `npx tauri icon` → `cargo test`（含 specta 生成 `src/types/generated.ts`） → 上传 `generated-ts` artifact → `npm run test`（Vitest） → `npm run build`（`vue-tsc --noEmit` + `vite build`） → `npx tauri build`（Rust release build + 打包产物），上传 `CodeIsland-win-exe` / `msi` / `nsis` 产物。

Mac 本地若需更新 `src/types/generated.ts`（避免与 CI 漂移）：

```bash
gh run download <latest-green-run-id> -n generated-ts -D /tmp/gts
cp /tmp/gts/generated.ts src/types/generated.ts
```
