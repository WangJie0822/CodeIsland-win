# Code Island Windows 版 UI/UE 对齐 macOS 参考项目 · 设计文档

- 日期：2026-04-17
- 状态：设计 · 待用户审查
- 目标版本：Windows 纯平台专属发行（去除所有跨平台分支）
- 参考项目：macOS Swift 原生版（位置由用户本地持有，不再引用绝对路径）

## 1. 背景与目标

Windows 版当前是 Tauri + 原生 TS/DOM 的 MVP，前端 287 行 / 9 个文件，仅实现「状态显示 + 权限审批」的最小闭环；macOS 参考项目是 SwiftUI 原生版，11,353 行 UI / 27 个视图组件，包含刘海形状、像素艺术、Buddy 卡牌收集、Daily Report 使用率、Live Edit 模式、完整设置菜单、Launch Presets、工具结果专用视图、Markdown 渲染等高度定制化体验。

本文档定义 Windows 版将 UI/UE 完整对齐参考项目的实现方案。**完全针对 Windows 优化，不再考虑跨平台**，所有 Unix 专属分支（Unix Socket / ps / macOS terminal stub / Python hook Unix 路径等）在第一阶段清理。

## 2. 决策记录（brainstorming 阶段确认）

| 议题 | 决策 |
|---|---|
| 对齐深度 | **完整复刻**（像素猫、Buddy、Daily Report、Live Edit、设置、Launch Presets 全量移植） |
| 外形语言 | **虚拟刘海**（Windows 显示器顶端悬浮异形黑色区域，复刻 macOS NotchShape 曲线） |
| 技术栈 | **保留 Tauri**；前端从裸 DOM 升级到 **Vue 3 + Pinia** |
| Pair iPhone | **暂时去掉**（独立子系统，延后迭代） |
| Accessibility 横幅 | **去掉**（Windows SendInput 无需额外权限） |
| Daily Report 数据源 | **自扫 JSONL**（基于现有 `jsonl_watcher.rs` 扩展聚合） |
| Launch Presets | **保留**（调用 `claude` CLI + 参数） |
| Hooks UI | **保留**（提供开关 + 状态指示） |
| 日常开关 | **Pixel Cat / Smart Suppression / Auto-Collapse / Launch at Login** 全保留 |

## 3. 差距对比速览（设计前基线）

| 维度 | 参考项目 | 当前 Windows 版 | 需补齐 |
|---|---|---|---|
| UI 代码量 | 11,353 行 / 27 组件 | 287 行 / 5 组件 | 架构级重构 |
| 外形 | NotchShape 异形 | 圆角矩形 | 虚拟刘海形状 |
| 像素艺术 | NeonPixelCat / BuddyASCII / EmojiPixel / PixelCharacter | 无 | 全部 |
| Buddy 卡牌 | LEGENDARY 等级 + 5 属性 | 无 | 新建 |
| Daily Report | 5h / 7d 使用率进度条 | 无 | 新建 |
| Live Edit | 拖动 / 贴合 / 复位 / 尺寸指示 | 仅基础拖动 | 扩展 |
| 设置菜单 | 屏幕 / 音效 / 语言 / 多项开关 | 仅托盘显示/退出 | 新建 |
| 会话卡片 | 像素头像 + AI 摘要 + cmux 标签 + 终端按钮 | 状态点 + 项目名 + cwd | 重构 |
| 工具结果视图 | 按工具类型专用视图（1123 行） | 纯 JSON 拼接 | 新建 |
| Markdown | 专用 renderer | 无 | 新建 |
| 跨平台代码 | 仅 macOS | 大量 `#[cfg(not(target_os = "windows"))]` | 清理 |

## 4. 总体架构

### 4.1 分层示意

```
┌─────────────────────────────────────────────────────────────┐
│  Vue 3 前端（src/）                                         │
│  - views/  刘海视图容器、设置页、Buddy/Daily Report 等大视图 │
│  - components/  会话卡片、工具结果视图、像素艺术组件         │
│  - stores/  Pinia 状态（sessions/settings/buddy/usage）      │
│  - composables/  useTauri/useHookEvents/useAnimations        │
│  - types/  前后端共享的 TypeScript 类型（specta 生成）      │
├─────────────────────────────────────────────────────────────┤
│  Tauri IPC（@tauri-apps/api）                               │
│  - invoke: get_sessions / approve_permission / ...           │
│  - listen: codeisland:sessions:updated 等规范化事件名        │
├─────────────────────────────────────────────────────────────┤
│  Rust 后端（src-tauri/src/）                                 │
│  - hook/  Named Pipe + installer（已有，清理非 Windows 分支） │
│  - session/  SessionStore + jsonl_watcher + process_scanner  │
│  - buddy/  【新】Buddy 属性生成与持久化                      │
│  - usage/  【新】JSONL 聚合出 5h/7d 使用率                   │
│  - presets/  【新】Launch Presets 与 claude CLI 启动         │
│  - settings/  【新】用户设置持久化                            │
│  - autostart/  【新】Windows 注册表开机自启                  │
│  - screens/  【新】显示器枚举                                 │
│  - terminal/  Windows SendInput（已有，保留）                │
│  - sound/  rodio 音效（已有，保留）                          │
│  - commands/  Tauri 命令层（扩展）                           │
│  - paths.rs  【新】Windows 路径常量集中                       │
└─────────────────────────────────────────────────────────────┘
```

### 4.2 跨平台清理范围

- `pipe_server.rs` 删除 Unix Socket 分支
- `process_scanner.rs` 删除 `ps + kill -0` 实现
- `terminal/` 删除 Unix stub 文件
- `hook/installer.rs` 删除 macOS 路径分支，固定 `%USERPROFILE%\.claude\hooks\`
- `codeisland-state.py` 删除 Unix Socket 分支
- `scripts/dev.sh` 删除（保留 `scripts/dev.ps1`）
- `CLAUDE.md` 删除跨平台差异表，更新为纯 Windows 描述
- **保留**：Tauri 本身作为 Windows 应用运行时

### 4.3 前后端类型共享

使用 `specta` + `tauri-specta` 从 Rust struct 自动导出 TypeScript 类型到 `src/types/generated.ts`。CI 里加 `cargo test generate_types`，类型漂移即阻断。

### 4.4 事件与配置

- Tauri emit 事件名统一：`codeisland:<domain>:<action>`
- 用户设置：`%APPDATA%\codeisland\settings.json`
- Buddy 进度：`%APPDATA%\codeisland\buddy.json`
- Launch Presets：`%APPDATA%\codeisland\presets.json`
- 日志：`%LOCALAPPDATA%\codeisland\logs\`（7 天滚动）

## 5. 前端目录结构（Vue 3 + Pinia）

```
src/
├── main.ts                          # 应用入口
├── App.vue                          # 根组件
├── router/index.ts                  # Vue Router（设置页子路由）
├── stores/                          # Pinia
│   ├── sessions.ts
│   ├── settings.ts
│   ├── buddy.ts
│   ├── usage.ts
│   ├── presets.ts
│   └── notch.ts
├── composables/
│   ├── useHookEvents.ts             # Tauri listen 封装
│   ├── useTauri.ts                  # invoke 封装 + 类型化
│   ├── useAutoCollapse.ts
│   ├── useDragResize.ts
│   └── useAnimations.ts
├── views/
│   ├── NotchView.vue                # 刘海主容器
│   ├── SessionListView.vue
│   ├── SettingsView.vue
│   ├── DailyReportCardView.vue
│   ├── BuddyCardView.vue
│   ├── LaunchPresetsView.vue
│   └── NotchLiveEditView.vue
├── components/
│   ├── notch/
│   │   ├── NotchShape.vue
│   │   ├── NotchHeader.vue
│   │   └── NotchMenu.vue
│   ├── session-card/
│   │   ├── SessionCard.vue
│   │   ├── SessionCardHeader.vue
│   │   ├── SessionCardStatus.vue
│   │   ├── ApprovalButtons.vue
│   │   └── AskUserOptions.vue
│   ├── tool-results/
│   │   ├── index.ts                 # 按 tool_name 路由
│   │   ├── ReadResult.vue
│   │   ├── EditResult.vue
│   │   ├── BashResult.vue
│   │   ├── GrepResult.vue
│   │   ├── GlobResult.vue
│   │   ├── WriteResult.vue
│   │   ├── TaskResult.vue
│   │   ├── TodoWriteResult.vue
│   │   ├── WebSearchResult.vue
│   │   ├── WebFetchResult.vue
│   │   └── GenericResult.vue
│   ├── pixel/
│   │   ├── NeonPixelCat.vue
│   │   ├── BuddyASCII.vue
│   │   ├── EmojiPixel.vue
│   │   ├── PixelCharacter.vue
│   │   └── ProcessingSpinner.vue
│   ├── settings/
│   │   ├── SystemSettings.vue
│   │   ├── SoundSettings.vue
│   │   ├── NotchCustomizationSettings.vue
│   │   └── HooksSettings.vue
│   ├── preset-editor/
│   │   └── PresetEditor.vue
│   └── common/
│       ├── ActionButton.vue
│       ├── StatusDot.vue
│       ├── Toggle.vue
│       ├── Picker.vue
│       └── MarkdownRenderer.vue
├── types/
│   ├── generated.ts                 # specta 导出
│   └── ui.ts
├── utils/
│   ├── time.ts
│   ├── phase.ts
│   ├── notch-shape.ts               # 刘海几何路径生成
│   └── markdown.ts
└── styles/
    ├── theme.css                    # 设计 token
    ├── notch.css
    ├── pixel.css
    └── main.css
```

## 6. 虚拟刘海形状 + 展开动画

### 6.1 技术选型

SVG `clip-path` + 路径插值动画。Tauri 窗口 `decorations: false` + `transparent: true`，窗口内 SVG path 裁剪出刘海轮廓。WebView2 对 SVG path d 属性动画平滑度良好。

### 6.2 几何参数

```ts
interface NotchGeometry {
  width: number;
  height: number;
  topCornerRadius: number;     // 贴屏幕侧圆角
  bottomCornerRadius: number;  // 下垂侧圆角
}
const COLLAPSED = { width: 220, height: 32, topCornerRadius: 6, bottomCornerRadius: 14 };
const EXPANDED  = { width: 760, height: 520, topCornerRadius: 19, bottomCornerRadius: 24 };
```

路径由 8 段组成（顺时针，从左上角起）：顶边直线 → 右上圆弧 → 右侧直线 → 右下反向圆弧 → 底边 → 左下反向圆弧 → 左侧直线 → 左上圆弧闭合。中间帧通过线性插值 4 个几何参数生成。

### 6.3 容器结构

```vue
<template>
  <div class="notch-root" :style="rootStyle">
    <svg class="notch-svg" :viewBox="viewBox">
      <defs>
        <clipPath :id="clipId"><path :d="currentPath" /></clipPath>
        <filter :id="shadowId">
          <feDropShadow dx="0" dy="4" stdDeviation="12" flood-opacity="0.4" />
        </filter>
      </defs>
      <path :d="currentPath" fill="rgba(0,0,0,0.85)" :filter="`url(#${shadowId})`" />
    </svg>
    <div class="notch-content" :style="{ clipPath: `url(#${clipId})` }">
      <NotchHeader v-if="!notchStore.expanded" />
      <SessionListView v-else />
    </div>
  </div>
</template>
```

### 6.4 动画策略

- Vue `<Transition>` + `requestAnimationFrame` 驱动 path d 逐帧插值
- 默认时长 280ms，曲线 `cubic-bezier(0.32, 0.72, 0, 1)`
- 动画同步调用 `getCurrentWindow().setSize(new PhysicalSize(w, h))` 避免裁剪外区域点击穿透异常
- Live Edit 模式禁用动画（拖动要实时）

### 6.5 窗口定位与点击穿透

- 启动读主显示器 `availableMonitors()`，X 居中，Y=0 贴顶
- 默认 `setIgnoreCursorEvents(true)`，前端 `pointermove` 计算鼠标是否在 SVG path 内，动态切换穿透状态
- 新增 Rust 命令 `set_cursor_passthrough(enabled: bool)`

### 6.6 Hover 触发展开

保留 mouseenter/mouseleave 逻辑；mouseleave 后设置可配置延时（默认 300ms）收起，延时内回到刘海取消计时器。

## 7. 会话卡片与工具结果视图

### 7.1 SessionCard 布局

```
🐱  CodeLight · 加上                    cmux  9m  [⎕]  ×
    AI 加上: ✅ 推送成功到 CodeIsland `main` 分支！...
    [Allow] [Deny]         ← 审批状态时显示
    ┌──────────────────────────────────────────────────┐
    │ 工具调用结果视图（按 tool_name 路由）             │
    └──────────────────────────────────────────────────┘
```

拆分：`SessionCard.vue` / `SessionCardHeader.vue` / `SessionCardMeta.vue` / `SessionCardBody.vue` / `ApprovalButtons.vue` / `AskUserOptions.vue`。

### 7.2 头像动画

NeonPixelCat 8 帧 SVG 雪碧图，状态驱动动画映射：
- `idle` → 静止
- `processing` → 眨眼 + 摆尾
- `waitingForApproval` → 警觉 + 闪烁
- `waitingForInput` → 伸懒腰

用户可关闭 Pixel Cat，fallback 到首字母圆形头像。

### 7.3 工具结果视图路由器

`components/tool-results/index.ts`：
```ts
const registry: Record<string, Component> = {
  Read: defineAsyncComponent(() => import('./ReadResult.vue')),
  Edit: defineAsyncComponent(() => import('./EditResult.vue')),
  MultiEdit: defineAsyncComponent(() => import('./EditResult.vue')),
  Write: defineAsyncComponent(() => import('./WriteResult.vue')),
  Bash: defineAsyncComponent(() => import('./BashResult.vue')),
  Grep: defineAsyncComponent(() => import('./GrepResult.vue')),
  Glob: defineAsyncComponent(() => import('./GlobResult.vue')),
  Task: defineAsyncComponent(() => import('./TaskResult.vue')),
  TodoWrite: defineAsyncComponent(() => import('./TodoWriteResult.vue')),
  WebSearch: defineAsyncComponent(() => import('./WebSearchResult.vue')),
  WebFetch: defineAsyncComponent(() => import('./WebFetchResult.vue')),
};
export function resolveToolView(toolName: string) { return registry[toolName] ?? GenericResult; }
```

### 7.4 各视图规格

| 工具 | 展示 |
|---|---|
| Read | 文件路径缩写 + 行数范围 `L1-50` + 总行数 |
| Edit / MultiEdit | 文件路径 + 改动段数 + 折叠 diff（前 20 行，可展开） |
| Write | 文件路径 + 字节数 + "NEW" 徽章 |
| Bash | 命令单行等宽 + stdout 前 10 行 + exit code |
| Grep | pattern + 匹配文件数 + 前 5 路径 |
| Glob | glob + 匹配数 + 前 5 路径 |
| Task | subagent 名 + 描述 + 状态 |
| TodoWrite | todo 列表带完成/进行/待处理样式 |
| WebSearch | 查询词 + 结果数 |
| WebFetch | URL 缩短 + 字节数 |
| Generic | 兜底：tool_name + JSON 预览 500 字符 |

所有视图：最大高 160px 溢出滚动，等宽字体 `Cascadia Code`，可展开/折叠，pending/running/success/error 4 种状态样式。

### 7.5 MarkdownRenderer

`markdown-it` + `shiki` 代码高亮。禁用 `html`/`breaks`。链接用 Tauri `shell.open` 新窗口打开。代码块 hover 显示复制按钮。

### 7.6 `SessionSummary` 字段扩展

```rust
pub struct SessionSummary {
    // existing...
    pub tool_output: Option<serde_json::Value>,
    pub tool_status: Option<ToolStatus>,      // pending/running/success/error
    pub launcher: Option<String>,             // "cmux" / "tmux" / "native"
    pub pixel_cat_enabled: bool,
    pub last_activity_ts: i64,
}
```

## 8. 像素艺术 / Buddy / Daily Report

### 8.1 像素艺术组件

全部 SVG + `image-rendering: pixelated`，`animation-timing-function: steps(N, end)` 逐帧切换。

| 组件 | 实现 |
|---|---|
| NeonPixelCat | SVG 雪碧图 8 帧 `<use>` 切换 |
| BuddyASCII | `<pre>` 等宽字体 + `<span>` 分段上色 |
| EmojiPixel | emoji 栅格化 16×16 + SVG `<rect>` |
| PixelCharacter | 通用像素角色渲染器 |
| ProcessingSpinner | 像素风格加载动画 4 帧 |

### 8.2 Buddy 数据模型

```rust
pub struct Buddy {
    pub id: String, pub name: String, pub display_name: String,
    pub rarity: Rarity, pub ascii_art: Vec<String>, pub description: String,
    pub stats: BuddyStats, pub unlocked_at: Option<i64>,
}
pub struct BuddyStats { pub debug: u8, pub patience: u8, pub chaos: u8, pub wisdom: u8, pub sneak: u8 }
pub enum Rarity { Common, Rare, Epic, Legendary }
```

名册从参考项目 `BuddyASCIIView.swift` 提取 10+ 角色，存编译时常量 `src-tauri/src/buddy/registry.rs`。

### 8.3 解锁机制

- 首次运行：解锁 Common "Kris"
- 累计完成 N 个会话（`Stop` hook 事件）解锁下一个：Common 1+ / Rare 10+ / Epic 50+ / Legendary 100+
- 解锁时前端 toast 通知

### 8.4 持久化

`%APPDATA%\codeisland\buddy.json`：
```json
{
  "current": "octopus",
  "unlocked": ["kris", "octopus"],
  "stats_seeds": { "octopus": 42 },
  "completed_sessions": 57
}
```

### 8.5 Daily Report 数据源

扫描 `%USERPROFILE%\.claude\projects\*\*.jsonl`，每行 JSON 含 `message.usage`（input/output/cache tokens）。累加到时间桶：
- 5h 滑动窗口（百分比基于 `max_tokens_per_5h` 配额常量）
- 7d 滑动窗口

### 8.6 扫描策略

- 启动扫一次
- 后台 5 分钟间隔增量扫描（mtime 过滤）
- `Stop` hook 事件触发立即重扫

### 8.7 UI

对齐参考项目截图底部：进度条 5h + 7d + 剩余时间 + 刷新按钮。颜色：<50% 绿 / 50-80% 黄 / >80% 红闪烁。

## 9. 设置菜单 / Launch Presets / Live Edit

### 9.1 设置菜单项

| 项 | 控件 | 存储键 |
|---|---|---|
| Screen | ScreenPicker | `targetScreen` |
| Notification Sound | SoundPicker | `notificationSound` |
| Language | 下拉 | `language` |
| Pixel Cat | Toggle | `pixelCatEnabled` |
| Group by Project | Toggle | `groupByProject` |
| Smart Suppression | Toggle | `smartSuppression` |
| Auto-Collapse on Leave | Toggle + ms 输入 | `autoCollapseOnLeave` / `autoCollapseMs` |
| Launch at Login | Toggle | `launchAtLogin` |
| Hooks | Toggle + 状态 | `hooksEnabled` |
| Launch Presets | 进入子页 | - |
| Star / Feedback | 外链 | - |
| Version | 只读 | - |

**移除**：Accessibility（Windows 无需），Pair iPhone（延后）。

### 9.2 Launch at Login 实现

```rust
// src-tauri/src/autostart/windows.rs
pub fn set_launch_at_login(enabled: bool) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey(r"Software\Microsoft\Windows\CurrentVersion\Run")?;
    if enabled {
        let exe = std::env::current_exe()?;
        key.set_value("CodeIsland", &exe.to_str().unwrap())?;
    } else {
        let _ = key.delete_value("CodeIsland");
    }
    Ok(())
}
```

### 9.3 Launch Presets 数据模型

```rust
pub struct LaunchPreset {
    pub id: String, pub name: String, pub icon: String,
    pub cwd: String, pub initial_prompt: Option<String>,
    pub model: Option<String>, pub agent: Option<String>,
    pub mcp_servers: Vec<String>, pub sort_order: i32,
}
```

### 9.4 Preset 启动策略

终端优先级：Windows Terminal `wt.exe` → PowerShell `pwsh.exe` → `cmd.exe`。检测 `claude` CLI 路径：`where claude` → fallback `%APPDATA%\npm\claude.cmd`。启动命令示例：
```
wt.exe -d <cwd> pwsh -NoExit -Command "claude --cwd <cwd> --model <model> <initial_prompt>"
```

### 9.5 Live Edit 模式

- 进入：展开态右上「⋯」按钮
- 视觉：黄色虚线边框 + 实时尺寸指示器 + 底部 3 按钮（贴合顶部 / 拖动模式 / 复位）+ 左右微调箭头 + 右下保存/取消
- 拖动：`useDragResize.ts` 用 pointerdown/move/up + `setPosition`
- 保存：写 `settings.json` 的 `notchPosition { x, y, screen_id }`
- 快捷键：数字 1-9 快速对齐预设位置

## 10. Rust 后端新增模块

### 10.1 目录

```
src-tauri/src/
├── main.rs                   # 改
├── app_state.rs              # 改：扩展 AppState 与 AppEvent
├── commands/
│   ├── mod.rs                # 改
│   ├── sessions.rs           # 已有
│   ├── buddy.rs              # 新
│   ├── usage.rs              # 新
│   ├── presets.rs            # 新
│   ├── settings.rs           # 新
│   ├── screens.rs            # 新
│   ├── autostart.rs          # 新
│   ├── window_control.rs     # 新
│   └── shell.rs              # 新（pick_folder / open_url）
├── buddy/
├── usage/
├── presets/
├── settings/
├── autostart/
├── screens/
├── terminal/                 # 已有
├── sound/                    # 已有
└── paths.rs                  # 新
```

### 10.2 Tauri 命令全量表

| 命令 | 输入 | 输出 |
|---|---|---|
| `get_sessions` | - | `Vec<SessionSummary>` |
| `approve_permission` | `session_id, tool_use_id` | - |
| `deny_permission` | `session_id, tool_use_id, reason?` | - |
| `send_to_terminal` | `session_id, text` | - |
| `get_sound_enabled` / `set_sound_enabled` | - / bool | bool |
| `get_settings` / `update_settings` | - / patch | `Settings` |
| `get_screens` | - | `Vec<ScreenInfo>` |
| `get_buddy` / `switch_buddy` | - / id | `BuddyState` |
| `get_usage_report` / `refresh_usage_report` | - | `UsageReport` |
| `list_presets` / `save_preset` / `delete_preset` / `launch_preset` | id / preset / id / id | - |
| `get_autostart` / `set_autostart` | - / bool | bool |
| `set_ignore_cursor_events` | bool | - |
| `set_window_size` / `set_window_position` | w,h / x,y | - |
| `pick_folder` | - | `Option<String>` |
| `open_url` | url | - |
| `play_sound` | name | - |

### 10.3 新依赖

```toml
specta = "2"
specta-typescript = "0"
tauri-specta = { version = "2", features = ["typescript"] }
winreg = "0.52"
dirs = "5"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "serde"] }
notify = "6"
```

### 10.4 `AppEvent` 总线

```rust
pub enum AppEvent {
    SessionsUpdated,
    SettingsChanged(Settings),
    BuddyUnlocked(Buddy),
    UsageUpdated(UsageReport),
    PresetsUpdated,
    NotchPositionChanged { x: i32, y: i32 },
}
```

`main.rs` 单协程订阅 event_bus，转发为 Tauri `emit` 规范化事件名。

## 11. Stage 1 公共契约

Stage 1 派发 8 个并行 agent，所有接口必须在 Stage 0 末尾冻结。

### 11.1 文件所有权

| Agent | 独占目录 |
|---|---|
| A. session-card | `src/components/session-card/**` |
| B. tool-results | `src/components/tool-results/**` + `components/common/MarkdownRenderer.vue` |
| C. settings | `src/components/settings/**` + `src/views/SettingsView.vue` + `src-tauri/src/settings/**` + `autostart/**` + `screens/**` |
| D. pixel-art | `src/components/pixel/**` + `src/styles/pixel.css` |
| E. buddy | `src/views/BuddyCardView.vue` + `src-tauri/src/buddy/**` |
| F. usage | `src/views/DailyReportCardView.vue` + `src-tauri/src/usage/**` |
| G. presets | `src/views/LaunchPresetsView.vue` + `src/components/preset-editor/**` + `src-tauri/src/presets/**` |
| H. live-edit | `src/views/NotchLiveEditView.vue` + `src/composables/useDragResize.ts` + `src-tauri/src/commands/window_control.rs` |

**Stage 1 禁改共享文件**（`main.rs` / `app_state.rs` / `commands/mod.rs` / `router/index.ts` / `App.vue` / `types/generated.ts` / `Cargo.toml` / `package.json`），集中 Stage 2 整合。

### 11.2 TypeScript 类型契约（摘要）

```ts
export interface SessionSummary { /* 见 §7.6 扩展字段 */ }
export type SessionPhase = "idle" | "processing" | "waitingForApproval" 
                         | "waitingForInput" | "compacting" | "ended";
export interface Settings { /* 见 §9.1 全字段 */ }
export interface BuddyState { current: string; roster: Buddy[]; completed_sessions: number; }
export interface Buddy { /* 见 §8.2 */ }
export interface UsageReport { window_5h: UsageBucket; window_7d: UsageBucket; last_refresh_ts: number; }
export interface UsageBucket { tokens: number; quota: number; percent: number; resets_at: number | null; }
export interface LaunchPreset { /* 见 §9.3 */ }
export interface ScreenInfo { id: number; name: string; width: number; height: number; is_primary: boolean; }
```

所有 agent 只能 `import from '@/types/generated'`，禁止定义 domain 类型。

### 11.3 Pinia Store 只读 API

```ts
export const useSessionsStore = defineStore('sessions', () => {
  const list = ref<SessionSummary[]>([]);
  const byId = computed(() => new Map(list.value.map(s => [s.session_id, s])));
  const hasAttention = computed(() => list.value.some(s => s.needs_attention));
  async function refresh(): Promise<void>;
  async function approve(sessionId: string, toolUseId: string): Promise<void>;
  async function deny(sessionId: string, toolUseId: string, reason?: string): Promise<void>;
  async function sendToTerminal(sessionId: string, text: string): Promise<void>;
  return { list, byId, hasAttention, refresh, approve, deny, sendToTerminal };
});
```

同规格定义 `settings` / `buddy` / `usage` / `presets` / `notch` 每个 store。

### 11.4 公共组件 Props

```ts
interface ActionButtonProps { icon?: string; variant?: 'default'|'primary'|'danger'|'ghost'; active?: boolean; disabled?: boolean; loading?: boolean; }
interface StatusDotProps { status: 'idle'|'processing'|'attention'|'success'|'error'; size?: 'sm'|'md'|'lg'; }
interface ToggleProps { modelValue: boolean; disabled?: boolean; label?: string; }
interface PickerProps<T> { modelValue: T; options: { value: T; label: string }[]; disabled?: boolean; }
interface MarkdownRendererProps { source: string; inline?: boolean; maxHeight?: number; }
interface NeonPixelCatProps { state: AnimationState; size?: number; tint?: string; }
interface BuddyASCIIProps { frames: string[]; size?: number; colored?: boolean; }
```

### 11.5 设计 Token（摘要）

见 `src/styles/theme.css` 完整定义，关键变量：
- 颜色：`--bg-notch`, `--text-primary/secondary/tertiary`, `--accent-green/blue/yellow/red/purple/orange`
- 几何：`--notch-collapsed-w/h`, `--notch-expanded-w/h`, `--notch-*-radius-*`
- 动画：`--ease-notch`, `--dur-notch: 280ms`, `--dur-fast: 150ms`
- 字体：`--font-sans`, `--font-mono`, `--font-pixel`

### 11.6 事件命名规范

`codeisland:<domain>:<action>`，如 `codeisland:sessions:updated` / `codeisland:buddy:unlocked` / `codeisland:notch:position-changed`。

### 11.7 Rust 统一错误类型

```rust
#[derive(Debug, thiserror::Error, serde::Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum AppError {
    Io(String), Json(String), HookInstall(String),
    SessionNotFound(String), PresetLaunch(String),
    SettingsInvalid(String), Registry(String), Internal(String),
}
```

所有命令 `Result<T, AppError>`。前端 `useTauri.ts` 按 `kind` 路由展示。

### 11.8 子代理 Prompt 模板约束

每个 Stage 1 agent 的 prompt 必须包含：
1. **绝对 worktree 路径**（由主会话生成后填入）
2. 执行的**第一个动作**是 `pwd` 校验，与预期路径逐字比对，不匹配立即 abort
3. 明示**文件所有权列表**与**禁改共享文件清单**
4. 引用本设计文档路径
5. **完成判据**：代码可编译、单元测试通过、可视化验证通过
6. 提交信息格式：`[类型|模块|功能][公共]xxx`

## 12. 测试策略

### 12.1 测试矩阵

| 层级 | 工具 | 何时运行 |
|---|---|---|
| Rust 单元 | `cargo test` | push + CI |
| Rust 集成 | `cargo test --test integration` | push + CI |
| Rust Windows 专项 | `cargo test --test windows` | CI Windows runner |
| 前端单元 | Vitest | push + CI |
| 组件测试 | Vue Test Utils + Vitest | push + CI |
| E2E | `tauri-driver` + WebDriver | Release 前 |
| 视觉回归 | Playwright 截图比对 | Release 前手动 |

### 12.2 每 agent 测试清单（摘要）

- A: SessionCard × 5 phase / ApprovalButtons / AskUserOptions
- B: 每个 `*Result.vue` 一个 spec（input/output 两种模式）+ registry 测试
- C: `settings/persist_test.rs` / `autostart/windows_test.rs` / `screens_test.rs` / `SystemSettings.spec.ts`
- D: 各像素组件 spec（帧数/尺寸/tint）+ CSS 快照
- E: `buddy/registry_test.rs` + `unlock_test.rs` + `BuddyCardView.spec.ts`
- F: `usage/scanner_test.rs`（fixture JSONL）+ `quota_test.rs` + `DailyReportCardView.spec.ts`
- G: `presets/persist_test.rs` + `launcher_test.rs`（mock `Command`）+ `LaunchPresetsView.spec.ts` + `PresetEditor.spec.ts`
- H: `useDragResize.spec.ts` + `NotchLiveEditView.spec.ts` + `window_control_test.rs`

### 12.3 Fixture 策略

- Rust usage/scanner：`src-tauri/tests/fixtures/projects/` 放脱敏 JSONL 样本
- 前端组件 spec：`test/fixtures/sessions.ts` 提供 `createMockSession(phase)` 工厂
- E2E：`test/e2e/mock-hook-client.ts` 模拟 Claude Code 向 Named Pipe 发送预录事件序列

## 13. 错误处理

### 13.1 前端错误路由

- `HookInstall` / `Registry` / `SettingsInvalid` → 刘海顶部红色横幅 + 重试按钮
- `SessionNotFound` → 静默刷新列表
- `PresetLaunch` → Toast + 可展开详情
- `Io` / `Json` / `Internal` → 闪红 + 错误 ID + 复制到剪贴板

### 13.2 日志

- Rust `env_logger` → `%LOCALAPPDATA%\codeisland\logs\app-YYYY-MM-DD.log`
- 前端 `console.error` → Tauri `log_error` 命令 → 同一文件
- 滚动保留 7 天

### 13.3 关键容错场景

| 场景 | 策略 |
|---|---|
| Named Pipe 启动失败 | 重试 3 次，失败进入只读降级模式 |
| JSONL 读错 | 跳过该文件，连续 5 次同文件失败告警 |
| 注册表写入被拒 | Launch at Login 开关禁用 + 显示 "受策略限制" |
| Claude 未登录 | Daily Report 显示 "No usage data yet" |
| `claude` CLI 不存在 | Toast 引导安装 `npm i -g @anthropic-ai/claude-code` |
| 窗口失焦 | `always_on_top: true` 保持顶层 |

## 14. 迁移与回归

### 14.1 Stage 0 原子提交序列

| # | 提交信息 | 验证 |
|---|---|---|
| 1 | `[refactor\|backend\|pipe][公共]清理非 Windows IPC 分支` | `cargo test` + 启动 |
| 2 | `[refactor\|hook\|installer][公共]路径简化为 Windows 专属` | hook 安装 |
| 3 | `[refactor\|script\|python][公共]hook 脚本 Windows 化` | hook 完整链路 |
| 4 | `[build\|frontend\|vite][公共]前端引入 Vue 3 + Pinia` | `npm run build` |
| 5 | `[build\|rust\|specta][公共]类型导出 specta` | 类型生成 |
| 6 | `[refactor\|UI\|migrate][公共]现有组件迁移 Vue` | UI 回归 |
| 7 | `[feat\|UI\|notch][公共]虚拟刘海 SVG 形状 + 动画` | 外观与动画 |
| 8 | `[feat\|backend\|window][公共]cursor 穿透与窗口尺寸命令` | 鼠标穿透 |
| 9 | `[feat\|backend\|state][公共]AppEvent 总线与事件名规范化` | 订阅 |
| 10 | `[docs\|spec\|contracts][公共]Stage 1 并行契约文档` | 文档校验 |
| 11 | `[refactor\|docs\|claude_md][公共]移除跨平台说明` | - |

### 14.2 Stage 1 worktree 布局

```
.worktrees/agent-session-card   (feat/session-card)
.worktrees/agent-tool-results   (feat/tool-results)
.worktrees/agent-settings       (feat/settings)
.worktrees/agent-pixel-art      (feat/pixel-art)
.worktrees/agent-buddy          (feat/buddy)
.worktrees/agent-usage          (feat/usage)
.worktrees/agent-presets        (feat/presets)
.worktrees/agent-live-edit      (feat/live-edit)
```

### 14.3 Stage 2 合并顺序

1. pixel-art
2. settings
3. usage
4. presets
5. live-edit
6. tool-results
7. session-card（依赖 pixel-art）
8. buddy（依赖 pixel-art）

每个合并后：冲突解决 → 端到端烟测 → `cargo test` + `npm run test` → 集成 commit。

### 14.4 端到端回归清单（Stage 2 必过）

- [ ] 应用冷启动刘海出现在屏幕顶部
- [ ] hover 刘海展开成大卡片，mouseleave 折叠
- [ ] 启动 Claude 会话卡片自动出现，像素猫动画正确
- [ ] Bash 审批：Allow/Deny 按钮可用，Enter/Esc 生效
- [ ] Read/Edit/Grep 等工具结果视图渲染正确
- [ ] Markdown 消息渲染正常，代码块高亮
- [ ] 设置菜单所有开关可切换并持久化
- [ ] Launch at Login 切换改变注册表键值
- [ ] 屏幕选择切换后刘海移到目标屏
- [ ] Buddy 卡牌显示当前角色，属性条颜色正确
- [ ] 会话累计触发 Buddy 解锁 toast
- [ ] Daily Report 5h/7d 显示合理数值，刷新按钮工作
- [ ] Launch Preset 新建/编辑/删除 + 启动 claude CLI 成功
- [ ] Live Edit 拖动刘海位置，保存后重启保留
- [ ] 托盘图标右键菜单可用

## 15. 回滚与风险

### 15.1 回滚机制

- Stage 0 每提交原子，`git revert <hash>` 单独回退
- Stage 1 每 agent 分支独立，失败单独放弃
- Stage 2 合并后若某模块问题严重：`git revert -m 1 <merge-hash>` 回退该模块合并
- 发版前 `git tag v0.x.0-pre` 标签便于整体回退

### 15.2 风险汇总

| 风险 | 概率 | 影响 | 对策 |
|---|---|---|---|
| specta 类型与 Vue runtime 漂移 | 中 | 编译红 | CI 检查 `git diff --exit-code src/types/generated.ts` 后跑 `cargo test generate_types` |
| SVG clip-path 动画不平滑 | 中 | 视觉瑕疵 | Stage 0 末期 benchmark demo，降级方案双层 div（底 path + 顶 overflow:hidden） |
| Agent 并行误修共享文件 | 中 | Stage 2 冲突爆炸 | Prompt 强约束 + worktree 隔离 + pre-commit hook 检查禁改文件 |
| JSONL 聚合性能（大量历史） | 低 | 启动慢 | 增量扫描（mtime 过滤），仅聚合过去 7 天 |
| `claude` CLI 路径不一致 | 中 | 启动失败 | `where claude` 检测，fallback `%APPDATA%\npm\claude.cmd` |
| Windows Defender 拦截 Named Pipe | 低 | IPC 不通 | 首次启动引导用户确认 + 签名 msi 降低概率 |
| Buddy ASCII art 字体错位 | 中 | 视觉丑 | 锁定 Cascadia Code + 自带 webfont |

## 16. 执行顺序总结

```
Stage 0（主会话串行）
  └─ 11 个原子提交 → 交付 Stage 1 基建 + 契约文档
       │
       ▼
Stage 1（agent-teams 并行派发 8 worktree）
  ├─ A. session-card    ─┐
  ├─ B. tool-results     │
  ├─ C. settings         │
  ├─ D. pixel-art        ├─ 各自独立完成 + 测试通过
  ├─ E. buddy            │
  ├─ F. usage            │
  ├─ G. presets          │
  └─ H. live-edit       ─┘
       │
       ▼
Stage 2（主会话串行整合）
  └─ 按依赖顺序 merge → 解决冲突 → 端到端回归 → 发版候选
```

## 17. 待办：Stage 0 结束时需要产出的产出物

- [ ] 本设计文档（本文件）
- [ ] Stage 1 契约文档（可并入本文件 §11，或单独拆文件 `2026-04-17-windows-ui-alignment-contracts.md` 供 agent 直接引用）
- [ ] `src/types/generated.ts` 首版（由 specta 自动生成）
- [ ] Stage 0 所有 11 个提交通过 CI
- [ ] 虚拟刘海形状在 WebView2 实测截图（验证动画平滑度）
