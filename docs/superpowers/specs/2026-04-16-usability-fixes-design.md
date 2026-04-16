# Code Island 可用性修复设计

## 问题概述

用户在实际使用中发现四个阻塞性问题：

1. **Windows 上 Python 命令找不到**：hook 命令硬编码 `python3`，Windows 通常没有此命令
2. **Code Island 未运行时 hook 阻塞 Claude**：IPC 连接失败时脚本行为不确定，可能导致 Claude 卡住
3. **窗口无法拖拽**：启动在屏幕正中央，无法移动到期望位置
4. **无法关闭应用**：没有托盘菜单，没有关闭按钮

---

## 修复 1：Windows Python 命令检测

### 现状

`installer.rs` 的 `detect_python()` 在 Code Island **运行时**检测可用的 Python 命令，但存在问题：
- Windows 通常只有 `python` 或 `py`，没有 `python3`
- 写入 `settings.json` 的路径使用 `%USERPROFILE%` 环境变量，需确认 Claude Code hook 执行时能展开

### 方案

`detect_python()` 逻辑已经按优先级探测 `python3` → `python` → `py`，这部分无需修改。问题在于 Windows 上环境变量展开：将 `%USERPROFILE%\.claude\hooks\codeisland-state.py` 改为使用 Rust 运行时获取的实际 home 目录绝对路径。

### 修改文件

- `src-tauri/src/hook/installer.rs`
  - `update_settings()` 中的 command 模板：不再用 `%USERPROFILE%` 或 `~`，改为 `dirs::home_dir()` 获取的绝对路径
  - 路径分隔符：Windows 用 `\`，Unix 用 `/`

### 示例

```
# 修改前 (Windows)
python3 %USERPROFILE%\.claude\hooks\codeisland-state.py

# 修改后 (Windows)
python C:\Users\wj\.claude\hooks\codeisland-state.py

# 修改后 (macOS)
python3 /Users/wj/.claude/hooks/codeisland-state.py
```

---

## 修复 2：Code Island 未运行时 hook 安全退出

### 现状

`codeisland-state.py` 在 IPC 连接失败时：
- 非 PermissionRequest 事件：`send_event()` 返回 `None`，脚本正常退出——**已经安全**
- PermissionRequest 事件：`send_event()` 返回 `None`，脚本 `sys.exit(0)` 且不输出 `hookSpecificOutput`——Claude Code 可能回退到终端提示，也可能卡住

### 方案

在脚本入口处先探测 IPC 是否可达，不可达则立即 `sys.exit(0)`：

- Windows：尝试 `os.path.exists(r'\\.\pipe\codeisland')`（Named Pipe 存在性检查）
- Unix：检查 `/tmp/codeisland.sock` 文件是否存在

探测失败时立即退出，不执行任何逻辑，不输出任何内容。这样 Claude Code 对所有事件类型（包括 PermissionRequest）都会回退到自身默认行为。

### 修改文件

- `src-tauri/resources/codeisland-state.py`
  - `main()` 开头增加 IPC 可达性探测
  - Windows pipe 探测：`ctypes.windll.kernel32.WaitNamedPipeW` 或简单 `open()` 尝试
  - Unix socket 探测：`os.path.exists(SOCKET_PATH)`

---

## 修复 3：窗口拖拽 + 初始位置

### 现状

- `tauri.conf.json`：`center: true`，窗口启动后在屏幕正中央
- `index.html`：`#island` div 有 `data-tauri-drag-region`，但 Tauri 2 的拖拽需要正确配置
- 窗口 `decorations: false`，无系统标题栏，拖拽完全依赖 `data-tauri-drag-region`

### 方案

1. **初始位置**：移除 `tauri.conf.json` 的 `center: true`，改在 `main.rs` 的 `setup` 回调中用 Tauri API 获取屏幕尺寸，手动设置窗口位置为 `(屏幕宽度/2 - 窗口宽度/2, 0)` 即顶部居中
2. **拖拽**：确认 `data-tauri-drag-region` 在 Tauri 2 中正常工作。需要注意展开状态下会话列表区域应该可滚动而非触发拖拽，只有头部区域作为拖拽区域

### 修改文件

- `src-tauri/tauri.conf.json`：移除 `center`，可选设置初始 `x`/`y`
- `src-tauri/src/main.rs`：`setup` 中获取主显示器尺寸，设置窗口位置为顶部居中
- `index.html`：确认 `data-tauri-drag-region` 放置正确（仅在折叠条/头部区域）
- `src/components/Island.ts`：展开状态下添加一个拖拽区域（header bar）

---

## 修复 4：关闭功能

### 现状

- `tauri.conf.json` 配置了 `skipTaskbar: true`，窗口不在任务栏显示
- `Cargo.toml` 启用了 `tray-icon` feature，但代码中未创建系统托盘
- 无任何关闭/退出入口

### 方案

**托盘图标**：
- 在 `main.rs` setup 中创建系统托盘（`TrayIcon`）
- 右键菜单：显示/隐藏窗口、退出
- 退出时调用 `hook::installer::uninstall()` 清理 hooks

**UI 关闭按钮**：
- 展开状态下在右上角显示关闭按钮（X）
- 点击后隐藏窗口（最小化到托盘），不直接退出
- 双击托盘图标重新显示窗口

### 修改文件

- `src-tauri/src/main.rs`：创建 `TrayIcon`，注册菜单事件
- `src/components/Island.ts`：展开时渲染关闭按钮
- `src/styles/island.css`：关闭按钮样式
- `src/lib/events.ts`：可能需要新增 `hide_window` Tauri 命令（或直接用 `@tauri-apps/api/window`）

---

## 影响范围

| 修改 | 影响 |
|------|------|
| installer.rs 路径改绝对路径 | 已安装用户的 settings.json 下次启动会被更新覆盖 |
| hook 脚本增加探测 | 向后兼容，不影响已运行的 Code Island |
| 窗口位置改顶部 | 纯 UI 变更，无功能影响 |
| 托盘图标 + 关闭 | 新增功能，需要 Tauri capability 配置 |
