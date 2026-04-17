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
