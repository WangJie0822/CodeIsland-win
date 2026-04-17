# Code Island Windows 对齐项目 · 会话交接

- 最后更新：2026-04-17
- 原因：Stage 0 执行至 Task 3 后主会话 context 接近上限，转新会话继续推进剩余 8 个 task

## 项目背景

Windows 版 Code Island 对齐 macOS 参考项目（Dynamic Island 风格桌面悬浮窗）。实施分三阶段：**Stage 0 基建串行 → Stage 1 八 agent 并行 → Stage 2 主会话整合**。

核心文档：
- Spec：`docs/superpowers/specs/2026-04-17-windows-ui-alignment-design.md`
- Plan：`docs/superpowers/plans/2026-04-17-windows-ui-alignment-stage0.md`

## 当前工作分支

**`feat/stage0-windows-alignment`**（从 `master` 切出）

## Stage 0 进度（共 11 个 task）

| # | Task | Commit | 状态 |
|---|------|--------|------|
| 0 | 分支 + plan 提交 | `b2d5fc8` | ✅ |
| 1 | 清理非 Windows IPC 分支 | `e6aac4a` | ✅ |
| 2 | hook 安装器路径简化 | `cb066f2` | ✅ |
| 3 | hook 脚本 Windows 化 | `decdb7a` | ✅ |
| 4 | 前端引入 Vue 3 + Pinia | — | ⏳ |
| 5 | specta 类型导出 | — | ⏳ |
| 6 | 现有组件迁移 Vue | — | ⏳ |
| 7 | 虚拟刘海 SVG 形状 + 动画 | — | ⏳ |
| 8 | cursor 穿透 & 窗口命令 | — | ⏳ |
| 9 | AppEvent 总线 | — | ⏳ |
| 10 | Stage 1 并行契约文档 | — | ⏳ |
| 11 | CLAUDE.md 清理 + 删 dev.sh | — | ⏳ |
| Final | 整体代码审查 | — | ⏳ |

## 执行流程（subagent-driven-development skill）

每个 task 按 **两阶段 review** 执行：
1. 读 plan 取 task 完整内容（plan 文件 4013 行，用 offset/limit 分段读）
2. 派 **implementer subagent**（general-purpose）——提供完整 task 文本 + 当前 commit 上下文
3. 派 **spec reviewer subagent**（general-purpose）——验证实现对齐 plan（verify code, don't trust report）
4. 派 **code quality reviewer**（subagent_type: `superpowers:code-reviewer`）——standard review dimensions
5. 若 review 发现问题，implementer 修 → re-review
6. 标记 TaskUpdate 完成，进入下一 task

Plan 任务边界索引（grep 结果）：
- Task 4: line 1016
- Task 5: line 1213
- Task 6: line 1527
- Task 7: line 2222
- Task 8: line 2727
- Task 9: line 3123
- Task 10: line 3586
- Task 11: line 3799

## 关键环境约束

- **主机**：macOS (Darwin 24.6.0)
- **Rust 测试**：本机无 `rustup`（cargo via Homebrew），无法跑 Windows `cargo test` / `cargo check --target x86_64-pc-windows-msvc`。**Rust 改动的测试验证完全交给 GitHub Actions `build-windows.yml` runner**（CI 仅 Windows 构建）
- **前端测试**：Vitest + @vue/test-utils 可在 macOS 本地运行（Task 4 起）
- **Python 测试**：pytest 本地可跑（Task 3 已全绿，6/6）
- **内网环境**：`rustup target add`、`pip install` 可能受阻，遇到装不上就 defer

## include_str! / `__file__` 自引用陷阱（Task 1/2 教训）

**问题**：Rust 里 `include_str!("同文件.rs") + assert!(!src.contains("needle"))`，needle 字符串作为 literal 出现在源文件（作为 assertion 参数、error message、变量名），导致 contains 必然命中。

**修复模式**（已验证）：
1. **Needle 拆成两段字面量**：`concat!("Unix", "Listener")` — 编译期合并，源文件不含连续子串
2. **变量名不含 needle 完整子串**（大小写敏感）：`let unix_cfg = ...` ✓；`let scan_unix = ...` ✗（变量名 `scan_unix` 本身就是 needle `scan_unix` 的子串）
3. **错误消息不含 needle 完整子串**：`"Unix 分支残留"` ✓；`"cfg(unix) 残留"` ✗

**Task 3 不受影响**：Python 测试跨文件扫描（`Path(__file__).with_name("codeisland-state.py")`），测试文件的 needle 不污染被测文件。

**Task 4 起的前端测试**类似——用 Vitest 的组件/store 测试，正常断言行为，无 include_str! 陷阱。

## Atomic commit 保持策略

每个 Task 一笔 commit。若 review 发现 bug 需要修：
1. Edit 修正代码
2. `git reset --soft HEAD~1` 撤回 HEAD commit（保留 staging + working tree）
3. `git add` 新修改文件
4. `git commit` 相同的 message

**禁止** `git commit --amend`（CLAUDE.md 要求）。禁止 `git reset --hard`。

## 已知已 defer 的 follow-up（非阻塞，Stage 0+ 处理）

Reviewer 都给 Approved，以下是 Minor/Important follow-ups：

- **Task 1**：`pipe_server.rs` 多处 `std::collections::HashMap` 全限定名、`8.min(session_id.len())` 重复、`process_scanner.rs` 的 `!handle.is_invalid()` 冗余 guard——纯 polish
- **Task 2**：`installer.rs` 有 3 处 `.expect(...)` panic（`home_dir`, `hooks 应是 object`）——Stage 0 plan 原样，按"installer 不 crash app"原则后续可改为 `warn!; return`。`detect_python` 候选顺序从 `[python3, python, py]` 改为 `[py, python, python3]`，需在 release notes 说明
- **Task 3**：`send_event()` 裸 `open` 未用 context manager → 资源泄漏风险；裸 `except Exception` 可能陷入 300s 超时死循环；`waiting_for_approval` 超时无 stderr 日志。测试覆盖可补 deny 路径和 response=None 路径

## 预检查（未跟踪/未处理文件）

工作区持续存在：
- `src-tauri/Cargo.lock`（pre-existing 修改，可能来自以前的 `cargo` 操作）
- `src-tauri/resources/__pycache__/*.pyc`（Python 字节码缓存，应加 .gitignore）
- `src-tauri/icons/android/`、`src-tauri/icons/ios/`（未跟踪目录，来源不明）

每个 task commit 前确认只 stage 相关文件，不夹带这些。

## 新会话启动指令

复制下面这段进新会话即可无缝继续：

> 接力 Code Island Windows 对齐项目 Stage 0 的 subagent-driven 执行。请读 `docs/superpowers/HANDOFF.md` 了解已完成进度、执行流程和关键约束（include_str! 陷阱、macOS 环境限制、atomic commit 策略）。当前分支：`feat/stage0-windows-alignment`。下一个 task：**Task 4（前端引入 Vue 3 + Pinia）**——从 plan `docs/superpowers/plans/2026-04-17-windows-ui-alignment-stage0.md` line 1016 开始读取完整内容，按 implementer → spec reviewer → code quality reviewer 两阶段 review 流程派发 subagent 执行。完成后依次推进 Task 5→Task 11 + Final review，每个 task 一笔 atomic commit。

## 关键参考

- Plan：`docs/superpowers/plans/2026-04-17-windows-ui-alignment-stage0.md`
- Spec：`docs/superpowers/specs/2026-04-17-windows-ui-alignment-design.md`
- Skill：`superpowers:subagent-driven-development`（已加载过，新会话直接调用）
- Plugin prompt 模板：`/Users/wj/.claude/plugins/cache/claude-plugins-official/superpowers/5.0.7/skills/subagent-driven-development/*.md`
- 项目提交规范：`[类型|模块|功能][影响范围]具体修改说明` 中文描述
- 项目 CLAUDE.md 强制要求：实证优先、变更影响评审、完整链路验证、禁止 amend / reset --hard
