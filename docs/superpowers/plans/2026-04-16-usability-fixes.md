# Code Island 可用性修复实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 修复 Code Island 四个阻塞性可用性问题，使其在 Windows 和 macOS 上均可正常使用。

**Architecture:** 四个独立修复点：(1) installer.rs 使用绝对路径写入 hook 命令；(2) Python hook 脚本启动时探测 IPC 可达性，不可达立即退出；(3) 窗口初始位置改为顶部居中，支持 drag region 拖拽；(4) 创建系统托盘 + UI 关闭按钮。

**Tech Stack:** Rust (Tauri 2), TypeScript (vanilla DOM), Python 3, CSS

---

### Task 1: installer.rs 使用绝对路径写入 hook 命令

**Files:**
- Modify: `src-tauri/src/hook/installer.rs:37-51`（`update_settings` 函数中的 command 构建逻辑）

- [ ] **Step 1: 修改 `update_settings` 使用绝对路径**

将 `update_settings` 函数的 command 构建从硬编码 `~` / `%USERPROFILE%` 改为 `dirs::home_dir()` 获取的绝对路径。同时将 `detect_python()` 和路径构建提到函数参数外面，让 `update_settings` 接收已解析的绝对命令字符串。

修改 `src-tauri/src/hook/installer.rs` 中 `update_settings` 函数内 command 构建部分（第 45-50 行）：

```rust
// 替换原有的 command 构建（第 45-50 行）
// 原代码：
//   let python = detect_python();
//   #[cfg(target_os = "windows")]
//   let command = format!("{} %USERPROFILE%\\.claude\\hooks\\codeisland-state.py", python);
//   #[cfg(not(target_os = "windows"))]
//   let command = format!("{} ~/.claude/hooks/codeisland-state.py", python);

// 新代码：
    let python = detect_python();
    let home = dirs::home_dir().expect("无法获取 home 目录");
    let script_path = home.join(".claude").join("hooks").join("codeisland-state.py");
    let command = format!("{} {}", python, script_path.display());
```

这样 Windows 上生成 `python C:\Users\wj\.claude\hooks\codeisland-state.py`，macOS 上生成 `python3 /Users/wj/.claude/hooks/codeisland-state.py`。

- [ ] **Step 2: 本地验证**

运行应用，检查 `~/.claude/settings.json` 中写入的 hook 命令是否为绝对路径：

```bash
cd src-tauri && cargo build 2>&1 | tail -3
cat ~/.claude/settings.json | python3 -c "import json,sys; d=json.load(sys.stdin); print(json.dumps(d.get('hooks',{}).get('SessionStart','N/A'), indent=2))"
```

预期输出：command 字段包含绝对路径（如 `/Users/wj/.claude/hooks/codeisland-state.py`），不含 `~` 或 `%USERPROFILE%`。

- [ ] **Step 3: 提交**

```bash
git add src-tauri/src/hook/installer.rs
git commit -m "[fix|hook|installer][公共]hook 命令使用绝对路径，修复 Windows 路径展开问题"
```

---

### Task 2: Python hook 脚本 IPC 可达性探测

**Files:**
- Modify: `src-tauri/resources/codeisland-state.py:1-16`（文件头部常量区域后）

- [ ] **Step 1: 添加 IPC 探测函数并在 main() 入口调用**

在 `codeisland-state.py` 的常量定义之后、`send_event` 函数之前，添加 `is_server_running()` 函数。在 `main()` 函数的 `json.load(sys.stdin)` 之前调用它，不可达则立即退出。

在第 16 行（`SOCKET_PATH = "/tmp/codeisland.sock"` 之后）插入：

```python


def is_server_running():
    """探测 Code Island IPC 服务是否可达"""
    if IS_WINDOWS:
        try:
            # Windows Named Pipe 存在性检查
            handle = open(PIPE_PATH, "r+b", buffering=0)
            handle.close()
            return True
        except OSError:
            return False
    else:
        return os.path.exists(SOCKET_PATH)
```

修改 `main()` 函数（第 79-83 行），在 `json.load` 之前添加探测：

```python
def main():
    if not is_server_running():
        sys.exit(0)

    try:
        data = json.load(sys.stdin)
    except json.JSONDecodeError:
        sys.exit(1)
```

同时修复第 4 行的 docstring 中的 `\\.\pipe\codeisland` 转义警告——将 docstring 改为原始字符串或去掉反斜杠路径：

```python
"""
Code Island Hook (Windows + macOS)
- Windows: Named Pipe
- macOS/Linux: Unix Socket (/tmp/codeisland.sock)
- PermissionRequest 时阻塞等待用户决策
"""
```

- [ ] **Step 2: 本地验证——Code Island 未运行时脚本应立即退出**

确保 Code Island 没有运行（Unix socket 不存在）：

```bash
rm -f /tmp/codeisland.sock
echo '{"session_id":"test","hook_event_name":"SessionStart","cwd":"/tmp"}' | python3 ~/.claude/hooks/codeisland-state.py 2>&1; echo "exit: $?"
```

预期：无输出，exit code 为 0，耗时 < 0.1 秒。

- [ ] **Step 3: 本地验证——Code Island 运行时脚本应正常工作**

启动 Code Island（`npm run tauri dev`），然后：

```bash
echo '{"session_id":"test","hook_event_name":"SessionStart","cwd":"/tmp"}' | python3 ~/.claude/hooks/codeisland-state.py 2>&1; echo "exit: $?"
```

预期：exit code 为 0，Code Island UI 显示新会话。

- [ ] **Step 4: 提交**

```bash
git add src-tauri/resources/codeisland-state.py
git commit -m "[fix|hook|script][公共]hook 脚本启动时探测 IPC 可达性，未运行时静默退出"
```

---

### Task 3: 窗口顶部居中 + 拖拽支持

**Files:**
- Modify: `src-tauri/tauri.conf.json:13-20`（windows 配置）
- Modify: `src-tauri/src/main.rs:17-81`（setup 回调）
- Modify: `index.html:11`（drag region 属性调整）
- Modify: `src/components/Island.ts:26-69`（render 函数，展开时添加 drag header）
- Modify: `src/styles/island.css`（添加 drag header 样式）

- [ ] **Step 1: 修改 tauri.conf.json 移除 center**

修改 `src-tauri/tauri.conf.json` 的 windows 配置，移除 `center: true`：

```json
    "windows": [
      {
        "label": "island",
        "title": "Code Island",
        "width": 400,
        "height": 48,
        "decorations": false,
        "transparent": true,
        "alwaysOnTop": true,
        "skipTaskbar": true,
        "resizable": false
      }
    ],
```

- [ ] **Step 2: main.rs setup 中设置窗口初始位置为顶部居中**

在 `src-tauri/src/main.rs` 的 `setup` 回调中，所有现有初始化代码之后、`Ok(())` 之前，添加窗口定位逻辑：

```rust
            // 窗口定位：顶部居中
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
```

需要在文件顶部添加 `use tauri::WebviewWindowExt;`——但 Tauri 2 中 `get_webview_window` 来自 `Manager` trait（已经 `use tauri::Manager`），`current_monitor` / `set_position` 是 `WebviewWindow` 自带方法，无需额外 import。

- [ ] **Step 3: 调整 drag region 到折叠栏和展开时的 header**

修改 `index.html`，将 `data-tauri-drag-region` 从外层 `#island` 移到 `#island-collapsed`，并添加一个展开时的 drag header：

```html
<body>
  <div id="island" class="island collapsed">
    <div id="island-collapsed" class="island-collapsed" data-tauri-drag-region>
      <span class="status-dot"></span>
      <span class="session-count">0 sessions</span>
      <span class="status-label">idle</span>
    </div>
    <div id="island-expanded" class="island-expanded">
      <div id="expanded-header" class="expanded-header" data-tauri-drag-region>
        <span class="header-title">Code Island</span>
      </div>
      <div id="session-list"></div>
    </div>
  </div>
  <script type="module" src="/src/main.ts"></script>
</body>
```

- [ ] **Step 4: 更新 Island.ts render 函数适配新 DOM 结构**

`src/components/Island.ts` 的 `render()` 函数中，`collapsedEl` 的内容渲染不变（它仍然是 `#island-collapsed`）。新增的 `#expanded-header` 和 `#session-list` 是静态结构，无需在 JS 中创建。

唯一需要调整的是：确保 `render()` 中不会误操作 `#expanded-header`。当前代码只操作 `collapsedEl` 和 `sessionList`，两者的 ID 与新结构一致，无需修改 `Island.ts` 的逻��。

- [ ] **Step 5: 添加 expanded-header 样式**

在 `src/styles/island.css` 中，`.island-expanded` 规则之后添加：

```css
.expanded-header {
  display: flex; align-items: center; justify-content: space-between;
  padding: 8px 12px 4px; cursor: grab;
}
.expanded-header:active { cursor: grabbing; }
.header-title { font-size: 12px; font-weight: 600; color: var(--text-secondary); }
```

- [ ] **Step 6: 本地验证**

```bash
npm run tauri dev
```

验证：
1. 窗口启动后出现在屏幕顶部居中位置
2. 折叠状态下可以拖拽整个条
3. 展开状态下可以通过顶部 header 区域拖拽
4. 展开状态下会话列表区域可以滚动，不触发拖拽

- [ ] **Step 7: 提交**

```bash
git add src-tauri/tauri.conf.json src-tauri/src/main.rs index.html src/styles/island.css
git commit -m "[fix|UI|window][公共]窗口初始位置改为顶部居中，支持拖拽移动"
```

---

### Task 4: 系统托盘图标 + 右键菜单

**Files:**
- Modify: `src-tauri/src/main.rs:1-93`（添加托盘创建逻辑）
- Modify: `src-tauri/Cargo.toml:7`（确认 tray-icon feature 已启用）

- [ ] **Step 1: 确认 Cargo.toml tray-icon feature**

当前 `src-tauri/Cargo.toml` 第 7 行已有 `tauri = { version = "2", features = ["tray-icon"] }`，无需修改。

- [ ] **Step 2: 在 main.rs setup 中创建系统托盘**

在 `src-tauri/src/main.rs` 顶部添加必要的 import，然后在 `setup` 回调中（窗口定位代码之后、`Ok(())` 之前）创建系统托盘。

在文件顶部 `use tauri::{Emitter, Manager};` 后添加：

```rust
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::image::Image;
```

在 `setup` 回调中添加托盘创建：

```rust
            // 系统托盘
            let show_item = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出 Code Island", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            let tray_icon = Image::from_bytes(include_bytes!("../icons/32x32.png"))
                .expect("无法加载托盘图标");

            let _tray = TrayIconBuilder::new()
                .icon(tray_icon)
                .tooltip("Code Island")
                .menu(&menu)
                .on_menu_event(move |app, event| {
                    match event.id.as_ref() {
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
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::DoubleClick { .. } = event {
                        if let Some(window) = tray.app_handle().get_webview_window("island") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;
```

- [ ] **Step 3: 本地验证托盘图标**

```bash
npm run tauri dev
```

验证：
1. 系统托盘出现 Code Island 图标
2. 右键托盘图标显示菜单：「显示窗口」「退出 Code Island」
3. 点击「退出」后应用退出，`~/.claude/settings.json` 中 codeisland 相关 hooks 被清除
4. 双击托盘图标显示窗口

- [ ] **Step 4: 提交**

```bash
git add src-tauri/src/main.rs
git commit -m "[feat|app|tray][公共]系统托盘图标 + 右键菜单（显示/退出）+ 退出时清理 hooks"
```

---

### Task 5: UI 关闭按钮（隐藏到托盘）

**Files:**
- Modify: `src/components/Island.ts:1-69`（render 函数中添加关闭按钮）
- Modify: `src/styles/island.css`（关闭按钮样式）

- [ ] **Step 1: 在 Island.ts 中添加关闭按钮渲染逻辑**

修改 `src/components/Island.ts`，在文件顶部添加 Tauri window API import：

```typescript
import { getCurrentWindow } from "@tauri-apps/api/window";
```

在 `render()` 函数中，渲染 `#expanded-header` 的关闭按钮。因为 header 是静态 HTML 元素，我们在 `initIsland()` 中一次性添加关闭按钮：

在 `initIsland()` 函数中（`store.subscribe(render)` 之前）添加：

```typescript
  const expandedHeader = document.getElementById("expanded-header")!;
  const closeBtn = document.createElement("button");
  closeBtn.className = "btn-close";
  closeBtn.textContent = "\u00d7";
  closeBtn.title = "隐藏窗口";
  closeBtn.onclick = async (e) => {
    e.stopPropagation();
    await getCurrentWindow().hide();
  };
  expandedHeader.appendChild(closeBtn);
```

- [ ] **Step 2: 添加关闭按钮样式**

在 `src/styles/island.css` 末尾添加：

```css
.btn-close {
  width: 20px; height: 20px; border: none; border-radius: 50%;
  background: rgba(255, 255, 255, 0.1); color: var(--text-secondary);
  font-size: 14px; line-height: 20px; text-align: center;
  cursor: pointer; transition: background 0.15s ease, color 0.15s ease;
  padding: 0; flex-shrink: 0;
}
.btn-close:hover { background: var(--accent-red); color: white; }
```

- [ ] **Step 3: 本地验证**

```bash
npm run tauri dev
```

验证：
1. 鼠标悬停展开后，header 右侧显示 × 按钮
2. 点击 × 按钮窗口隐藏
3. 双击托盘图标或右键菜单「显示窗口」可重新显示
4. × 按钮不会触发拖拽

- [ ] **Step 4: 提交**

```bash
git add src/components/Island.ts src/styles/island.css
git commit -m "[feat|UI|close][公共]展开状态显示关闭按钮，点击隐藏到系统托盘"
```

---

### Task 6: 端到端验证 + 推送

- [ ] **Step 1: Rust 测试**

```bash
cd src-tauri && cargo test
```

预期：所有现有测试通过（protocol、phase、store、jsonl_watcher 的测试）。

- [ ] **Step 2: TypeScript 类型检查**

```bash
npx tsc --noEmit
```

预期：无类型错误。

- [ ] **Step 3: 完整构建验证**

```bash
npm run build
```

预期：Vite 构建成功，输出到 `dist/`。

- [ ] **Step 4: 推送并触发 CI**

```bash
git push origin master
```

在 GitHub Actions 页面确认 Windows 构建通过。
