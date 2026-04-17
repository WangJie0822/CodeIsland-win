# Code Island Windows 对齐项目 · 会话交接

- 创建时间：2026-04-17
- 原因：上一轮 brainstorming 会话 context 超载，转新会话继续

## 已完成

### 1. Brainstorming 已完成并形成决策
- 对齐深度：**完整复刻**（像素猫/Buddy/Daily Report/Live Edit/设置/Presets 全量）
- 外形：**虚拟刘海**（Windows 显示器顶端悬浮异形黑色区域）
- 技术栈：**保留 Tauri + 前端升 Vue 3 + Pinia**
- Pair iPhone：**暂去掉**
- Accessibility 横幅：**去掉**
- Daily Report：自扫 JSONL
- Launch Presets / Hooks UI / Pixel Cat / Smart Suppression / Auto-Collapse / Launch at Login：**全保留**
- 实施路径：**Stage 0 基建串行 → Stage 1 八 agent 并行 → Stage 2 主会话整合**

### 2. 设计文档已写入并 commit
- 路径：`docs/superpowers/specs/2026-04-17-windows-ui-alignment-design.md`（800 行，17 节）
- Commit：`e0ed748 [docs|spec|ui-alignment][公共]Windows 版 UI/UE 对齐 macOS 参考项目设计文档`
- 自审通过：无 placeholder、内部一致、范围聚焦、无歧义、路径脱敏

### 3. pending-docs.json 状态
- 我曾创建 `~/Vault/.meta/pending-docs.json`，用户提示"路径已修改"——Vault 正确路径待用户确认
- 该 JSON 当前条目指向上述 spec 文件
- 新会话可重新按当时 Vault 位置维护

## 下一步（新会话要做）

### 1. 调用 writing-plans skill
已经加载过 `superpowers:writing-plans`，新会话可直接调用。

### 2. 写 Stage 0 的 plan
按 brainstorming 产出的拆分建议：
- 本次 plan 仅覆盖 **Stage 0 基建**（spec §14.1 的 11 个原子提交）
- 每个 Stage 1 agent 的 plan 留到 Stage 0 执行完、契约锁定后再单独生成
- Stage 2 整合的 plan 留到 Stage 1 产出后再写

Plan 保存路径：`docs/superpowers/plans/2026-04-17-windows-ui-alignment-stage0.md`

### 3. Stage 0 的 11 个 task 对应 spec §14.1

| # | 提交信息 |
|---|---|
| 1 | `[refactor\|backend\|pipe][公共]清理非 Windows IPC 分支` |
| 2 | `[refactor\|hook\|installer][公共]路径简化为 Windows 专属` |
| 3 | `[refactor\|script\|python][公共]hook 脚本 Windows 化` |
| 4 | `[build\|frontend\|vite][公共]前端引入 Vue 3 + Pinia` |
| 5 | `[build\|rust\|specta][公共]类型导出 specta` |
| 6 | `[refactor\|UI\|migrate][公共]现有组件迁移 Vue` |
| 7 | `[feat\|UI\|notch][公共]虚拟刘海 SVG 形状 + 动画` |
| 8 | `[feat\|backend\|window][公共]cursor 穿透与窗口尺寸命令` |
| 9 | `[feat\|backend\|state][公共]AppEvent 总线与事件名规范化` |
| 10 | `[docs\|spec\|contracts][公共]Stage 1 并行契约文档` |
| 11 | `[refactor\|docs\|claude_md][公共]移除跨平台说明` |

### 4. 执行选项（写完 plan 后）
根据 writing-plans skill 末尾说明：
- **Subagent-Driven（推荐）**：每个 task 派 subagent，两阶段 review
- **Inline Execution**：本会话用 executing-plans 批执行

## 新会话启动指令

复制下面这段进新会话即可无缝继续：

> 接力上个会话的 Code Island Windows 对齐项目。请读 `docs/superpowers/HANDOFF.md` 了解进度。当前任务：调用 `superpowers:writing-plans` skill 生成 Stage 0 的 plan，写入 `docs/superpowers/plans/2026-04-17-windows-ui-alignment-stage0.md`，依据 spec `docs/superpowers/specs/2026-04-17-windows-ui-alignment-design.md` §14.1 的 11 个原子提交。每个 task 按 bite-sized step 拆（写测试 → 验证失败 → 实现 → 验证通过 → commit），code block 写完整代码不要占位。写完 self-review 后提供 Subagent-Driven / Inline 两种执行选项。

## 关键参考

- Spec：`docs/superpowers/specs/2026-04-17-windows-ui-alignment-design.md`
- 参考项目源码：用户本地 `CodeIsland`（Swift/macOS 版，路径用户本地持有，不写入此文档）
- 参考项目 UI 代码量 11,353 行 / 27 组件；当前 Windows 版 287 行 / 5 组件
- 项目提交规范：`[类型|模块|功能][影响范围]具体修改说明` 中文描述
- 项目 CLAUDE.md 强制要求：实证优先、变更影响评审、完整链路验证
