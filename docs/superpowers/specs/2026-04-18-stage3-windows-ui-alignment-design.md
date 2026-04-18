# Code Island Windows UI 对齐 · Stage 3 设计文档

- 日期：2026-04-18
- 关联设计：`docs/superpowers/specs/2026-04-17-windows-ui-alignment-design.md`（Stage 0/1/2）
- 关联契约：`docs/superpowers/specs/2026-04-17-windows-ui-alignment-contracts.md`
- 前置状态：Stage 0~2 收官；Windows CI run `24564421890` 首次全绿；5 个新 View 已在 Stage 1 实现但未接入入口。
- 作用：把 Stage 1 产物的 5 个 View 通过 **多窗口独立 + vue-router 分发 + tray/刘海双入口** 接入可达路径，同时消化 Stage 2-3 级 follow-ups（F5/F8/F13/F14）与 autostart 真实实现。

---

## 1. 目标与范围

### 1.1 目标

1. 让 5 个 Stage 1 产出的 View 通过用户可见入口（tray 菜单 + 刘海「⋯」按钮）打开
2. 消费掉 `package.json` 声明但未使用的 `vue-router` 依赖，关闭 F14
3. autostart 由 placeholder 升级为真实注册表读写，Settings 的「开机启动」开关可落地
4. 修 F5 / F8 / F13 三项 Stage 2-3 级 follow-up

### 1.2 范围内

- 窗口拓扑：`tauri.conf.json` 静态声明 6 个窗口
- 前端：`src/router/index.ts` 新增；`src/main.ts` 按 window label 分发；`src/App.vue` 简化为 `<RouterView/>`
- Rust：`open_view_window` 命令、tray 扩展、`close_requested` 隐藏策略、`AppEvent::NotchPositionChanged`、`get_window_position` 命令
- autostart：`windows` crate `Win32_System_Registry` feature，HKCU\Run 真实读写
- NotchView 展开态右上「⋯」按钮 + View 菜单 popover
- CI：`build-windows.yml` 增加 `npm run test` 步骤；CLAUDE.md CI 段落同步

### 1.3 范围外（非本 Stage 任务）

- `MarkdownRenderer.vue` 升级（保持 placeholder；D2 推迟）
- preset id 从 timestamp 切 uuid（D3 推迟）
- `@tauri-apps/plugin-shell`（不需要；`presets/launcher.rs` 已用 `std::process::Command`）
- `winreg` crate（用 `windows` crate 原生 API，无新 crate）
- F3 / F4 / F6 / F7 / F10 / F11 / F12 / F16 follow-ups（标记为低优，本 Stage 不做）

### 1.4 不变式

- Stage 0 契约 §3 禁改共享文件清单 **在本 Stage 允许改动**（因为 Stage 3 是整合阶段，不再是并行 agent 阶段）
- Stage 1-2 后端既有 API 不回归；specta 四类约束（Runtime 泛型 / BigInt / `Option<serde_json::Value>` 递归 / `Option<&String>`）继续遵守
- 错误前缀规范：新命令按契约 §8 选用 `[window]` / `[autostart]` 前缀

---

## 2. 窗口拓扑

`src-tauri/tauri.conf.json` 的 `app.windows` 数组由当前 1 个扩为 6 个。所有窗口加载同一 `index.html`（不设 `url`），Router 在前端按 `label` 分发。

| label | 角色 | 初始尺寸 | 透明 | 无边框 | 置顶 | 跳过任务栏 | 可缩放 | 初始可见 |
|---|---|---|---|---|---|---|---|---|
| `island` | 主刘海 | 220×32 | ✓ | ✓ | ✓ | ✓ | ✗ | ✓ |
| `settings` | 偏好设置面板 | 720×520 | ✗ | ✓ | ✗ | ✗ | ✗ | ✗ |
| `buddy` | Buddy 信息卡片 | 480×640 | ✗ | ✓ | ✗ | ✗ | ✗ | ✗ |
| `usage` | Usage 报告 | 720×520 | ✗ | ✓ | ✗ | ✗ | ✗ | ✗ |
| `presets` | 启动预设列表 | 720×560 | ✗ | ✓ | ✗ | ✗ | ✗ | ✗ |
| `notch-live-edit` | 刘海 Live Edit 控制面板 | 320×200 | ✓ | ✓ | ✓ | ✓ | ✗ | ✗ |

设计说明：

- 5 个非主刘海窗口默认 `visible: false`，由 tray 或刘海按钮触发时 `window.show()` + `window.set_focus()`
- `island` 初始尺寸由 400×48 改为 220×32（消除 F5 race，见 §7）
- 非 `island` / `notch-live-edit` 的 View 是常规面板窗口（不透明、不置顶、出现在任务栏），拖动依赖 View 内已有的 `data-tauri-drag-region`
- `notch-live-edit` 形态仍为透明悬浮（需要贴近主刘海做拖拽 hint）

## 3. 前端路由分发

### 3.1 `src/router/index.ts`（新增）

```ts
import { createRouter, createMemoryHistory } from "vue-router";
import NotchView from "@/views/NotchView.vue";
import SettingsView from "@/views/SettingsView.vue";
import BuddyCardView from "@/views/BuddyCardView.vue";
import DailyReportCardView from "@/views/DailyReportCardView.vue";
import LaunchPresetsView from "@/views/LaunchPresetsView.vue";
import NotchLiveEditView from "@/views/NotchLiveEditView.vue";

export const router = createRouter({
  history: createMemoryHistory(),
  routes: [
    { path: "/island", component: NotchView },
    { path: "/settings", component: SettingsView },
    { path: "/buddy", component: BuddyCardView },
    { path: "/usage", component: DailyReportCardView },
    { path: "/presets", component: LaunchPresetsView },
    { path: "/notch-live-edit", component: NotchLiveEditView },
    { path: "/:pathMatch(.*)*", redirect: "/island" },
  ],
});
```

- `createMemoryHistory`：Tauri webview 没有浏览器 URL 栏，memory history 足够；不需要 hash / history mode
- 通配回退到 `/island`：异常情况下降级为主刘海，不白屏

### 3.2 `src/main.ts`（重写）

```ts
import { createApp } from "vue";
import { createPinia } from "pinia";
import { getCurrentWindow } from "@tauri-apps/api/window";
import App from "./App.vue";
import { router } from "./router";

const label = getCurrentWindow().label;

const app = createApp(App);
app.use(createPinia());
app.use(router);
await router.push(`/${label}`);
app.mount("#app");
```

- 取 `window.label` 是同步的（Tauri 2 的 `getCurrentWindow()` 在 webview 初始化后立即可用）
- 用 `await router.push` 而非 `replace`，首次加载走正常路由流程

### 3.3 `src/App.vue`（重写）

```vue
<script setup lang="ts"></script>

<template>
  <RouterView />
</template>

<style scoped></style>
```

顶层去 `<slot/>`，改由 RouterView 承载。已有样式（`.app-root`）移除，因为各 View 自己负责布局。

### 3.4 类型与测试影响

- `src/types/generated.ts`：无结构变化（除 §5 新增 `get_window_position` 命令和 `NotchPosition` 相关类型会 regen）
- 新增 `src/main.spec.ts`：测试分发逻辑（mock `getCurrentWindow` 断言路由 push）

---

## 4. 入口机制

### 4.1 tray 菜单（main.rs 扩展）

tray 菜单项顺序（main.rs 的 `Menu::with_items` 数组）：

```
- 显示主刘海         (id: "show-island")
- ─────────
- 偏好设置           (id: "open-settings")
- 伙伴               (id: "open-buddy")
- 用量报告           (id: "open-usage")
- 启动预设           (id: "open-presets")
- 调整刘海位置       (id: "open-notch-live-edit")
- ─────────
- 退出 Code Island   (id: "quit")
```

分隔符用 `PredefinedMenuItem::separator(app)`。点击处理：

```rust
.on_menu_event(move |app, event| match event.id.as_ref() {
    "show-island" => show_window(app, "island"),
    "open-settings" => show_window(app, "settings"),
    "open-buddy" => show_window(app, "buddy"),
    "open-usage" => show_window(app, "usage"),
    "open-presets" => show_window(app, "presets"),
    "open-notch-live-edit" => show_window(app, "notch-live-edit"),
    "quit" => {
        hook::installer::uninstall();
        app.exit(0);
    }
    _ => {}
})
```

`show_window` 抽为私有函数：`get_webview_window(label).show() + set_focus()`，若窗口不存在记 `log::warn!` 不 panic。

### 4.2 刘海「⋯」按钮 + popover

`NotchView.vue` 展开态右上角新增 `ActionButton`（已有组件），点击触发本地 `popoverOpen = true`。popover 实现用 Vue `<Teleport to="body">` 脱出刘海 SVG 裁剪，绝对定位锚定到按钮下方（`position: fixed; top: <button bottom>; right: <viewport - button right>`）。popover 内渲染与 tray 同样的 5 条 View 菜单项（不含"显示主刘海"和"退出"，因为此时刘海已可见且退出是系统级操作）。

```vue
<div v-if="notch.expanded && !popoverOpen" class="notch-menu-button">
  <ActionButton icon="…" variant="ghost" @click="popoverOpen = true" />
</div>
<NotchMenuPopover
  v-if="popoverOpen"
  @close="popoverOpen = false"
  @open-view="onOpenView"
/>
```

`onOpenView(label: string)` 调 `invoke('open_view_window', { label })`，随后 `popoverOpen = false`。

`NotchMenuPopover.vue` 是本 Stage 新增组件，放 `src/components/notch/NotchMenuPopover.vue`。列表项复用 tray 的标签字串。

### 4.3 `open_view_window` 命令

`src-tauri/src/commands/window_control.rs` 扩展：

```rust
const KNOWN_WINDOW_LABELS: &[&str] = &[
    "island", "settings", "buddy", "usage", "presets", "notch-live-edit",
];

pub fn validate_window_label(label: &str) -> Result<(), String> {
    if KNOWN_WINDOW_LABELS.contains(&label) {
        Ok(())
    } else {
        Err(format!("[window] 未知窗口 label: {}", label))
    }
}

#[tauri::command]
#[specta::specta]
pub async fn open_view_window(app: AppHandle, label: String) -> Result<(), String> {
    validate_window_label(&label)?;
    let window = app
        .get_webview_window(&label)
        .ok_or_else(|| format!("[window] 窗口未创建: {}", label))?;
    window.show().map_err(|e| format!("[window] show 失败: {}", e))?;
    window.set_focus().map_err(|e| format!("[window] focus 失败: {}", e))?;
    Ok(())
}
```

`validate_window_label` 抽纯函数便于单测（规避 Stage 1-2 经验中的 specta Runtime 泛型约束）。

### 4.4 `close_requested` 隐藏策略

`main.rs` `setup` 里遍历 `KNOWN_WINDOW_LABELS[1..]`（即跳过 `island`）为每个窗口注册 `on_window_event`：

```rust
for label in &["settings", "buddy", "usage", "presets", "notch-live-edit"] {
    if let Some(window) = app.get_webview_window(label) {
        let w = window.clone();
        window.on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = w.hide();
            }
        });
    }
}
```

`island` 窗口关闭走系统默认（但 `skipTaskbar: true` + 无任务栏关闭按钮，实际只能从 tray 退出）。

---

## 5. AppEvent 扩展 + get_window_position

### 5.1 `AppEvent::NotchPositionChanged`

`src-tauri/src/app_state.rs`：

```rust
pub enum AppEvent {
    SessionsUpdated,
    SettingsChanged,
    BuddyUnlocked,
    UsageUpdated,
    PresetsUpdated,
    NotchPositionChanged,   // 新增
}

impl AppEvent {
    pub fn topic(&self) -> &'static str {
        match self {
            Self::SessionsUpdated => "codeisland:sessions:updated",
            Self::SettingsChanged => "codeisland:settings:changed",
            Self::BuddyUnlocked => "codeisland:buddy:unlocked",
            Self::UsageUpdated => "codeisland:usage:updated",
            Self::PresetsUpdated => "codeisland:presets:updated",
            Self::NotchPositionChanged => "codeisland:notch:position-changed",
        }
    }
}
```

无 payload（与现有 Stage 1-2 事件一致）；消费方（主 `NotchView`）收到事件后调 `get_window_position('island')` 自取坐标。

### 5.2 `get_window_position` 命令

`src-tauri/src/commands/window_control.rs`：

```rust
#[derive(serde::Serialize, specta::Type)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
}

#[tauri::command]
#[specta::specta]
pub async fn get_window_position(
    app: AppHandle,
    label: String,
) -> Result<WindowPosition, String> {
    validate_window_label(&label)?;
    let window = app
        .get_webview_window(&label)
        .ok_or_else(|| format!("[window] 窗口未创建: {}", label))?;
    let pos = window
        .outer_position()
        .map_err(|e| format!("[window] 读取位置失败: {}", e))?;
    Ok(WindowPosition {
        x: pos.x,
        y: pos.y,
    })
}
```

specta 约束：
- `WindowPosition.x/y` 用 `i32`（避免 BigIntForbidden）
- `outer_position()` 返 `PhysicalPosition<i32>`，天然匹配

### 5.3 NotchLiveEditView 接入

`src/views/NotchLiveEditView.vue` 现有本地 `interface NotchPosition { x, y, screen_id }`，Stage 3 改为 import specta 生成的 `WindowPosition`（screen_id 字段由前端从 `availableMonitors` 取，不再进 Rust struct）。

---

## 6. autostart 真实实现

### 6.1 依赖扩展

`src-tauri/Cargo.toml`：

```toml
windows = { version = "0.58", features = [
    /* 现有 feature ... */
    "Win32_System_Registry",
]}
```

无新 crate。

### 6.2 `src-tauri/src/autostart/windows.rs` 重写

```rust
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Foundation::ERROR_FILE_NOT_FOUND;
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegDeleteValueW, RegOpenKeyExW,
    RegQueryValueExW, RegSetValueExW, HKEY, HKEY_CURRENT_USER,
    KEY_READ, KEY_WRITE, REG_SZ,
};

const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const VALUE_NAME: &str = "CodeIsland";

pub fn get_autostart() -> Result<bool, String> {
    unsafe {
        let mut hkey = HKEY::default();
        let subkey = HSTRING::from(RUN_KEY);
        let open_status = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR::from_raw(subkey.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        if open_status.is_err() {
            return Ok(false); // Run 子键理论上一定存在，保守返 false
        }
        let value_name = HSTRING::from(VALUE_NAME);
        let query_status = RegQueryValueExW(
            hkey,
            PCWSTR::from_raw(value_name.as_ptr()),
            None,
            None,
            None,
            None,
        );
        let _ = RegCloseKey(hkey);
        Ok(query_status.is_ok())
    }
}

pub fn set_autostart(enabled: bool) -> Result<bool, String> {
    let exe = std::env::current_exe()
        .map_err(|e| format!("[autostart] current_exe: {}", e))?;
    let exe_str = exe
        .to_str()
        .ok_or_else(|| "[autostart] exe 路径非 UTF-8".to_string())?;

    unsafe {
        let mut hkey = HKEY::default();
        let subkey = HSTRING::from(RUN_KEY);
        let status = RegCreateKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR::from_raw(subkey.as_ptr()),
            0,
            PCWSTR::null(),
            Default::default(),
            KEY_WRITE,
            None,
            &mut hkey,
            None,
        );
        if status.is_err() {
            return Err(format!("[autostart] RegCreateKeyExW: {:?}", status));
        }

        let value_name = HSTRING::from(VALUE_NAME);
        let result = if enabled {
            let data = HSTRING::from(exe_str);
            let data_bytes: &[u8] = std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                (data.len() + 1) * 2, // UTF-16 + null terminator
            );
            RegSetValueExW(
                hkey,
                PCWSTR::from_raw(value_name.as_ptr()),
                0,
                REG_SZ,
                Some(data_bytes),
            )
            .map(|_| true)
            .map_err(|e| format!("[autostart] RegSetValueExW: {:?}", e))
        } else {
            let s = RegDeleteValueW(hkey, PCWSTR::from_raw(value_name.as_ptr()));
            if s.is_ok() || s.0 as u32 == ERROR_FILE_NOT_FOUND.0 {
                Ok(false)
            } else {
                Err(format!("[autostart] RegDeleteValueW: {:?}", s))
            }
        };
        let _ = RegCloseKey(hkey);
        result
    }
}
```

错误前缀统一 `[autostart]`（契约 §8）。

### 6.3 测试策略

`#[cfg(all(test, target_os = "windows"))]` 单测里用 `CodeIsland-test-{pid}` 临时键名：

```rust
#[test]
fn roundtrip_set_get_delete() {
    let pid = std::process::id();
    let test_value_name = format!("CodeIsland-test-{}", pid);
    // 1. 把 VALUE_NAME 临时替换（通过 const generic 或测试专用函数）
    //    实际实现：把 get/set 接受一个 value_name 参数，公开函数封死为 VALUE_NAME
    // 2. set(true) → get() == true
    // 3. set(false) → get() == false
    // 4. cleanup: RegDeleteValueW test_value_name
}
```

实现上把 `get_autostart` / `set_autostart` 内部拆为 `get_autostart_inner(value_name)` / `set_autostart_inner(value_name, enabled)`，对外公开函数硬编码 `VALUE_NAME`，测试调 inner。

---

## 7. F5 / F8 / F13 修复

### 7.1 F5：`island` 窗口初始 COLLAPSED 尺寸

**当前**：`tauri.conf.json` 声明 `width: 400, height: 48`；`main.rs` 基于 `window_width = 400.0` 居中；`NotchView` onMount 调 `invokeSetWindowSize(220, 32)`，位置残留在 400 宽度对应的 x。

**修复**：
1. `tauri.conf.json` 改 `width: 220, height: 32`
2. `main.rs` 居中计算：`let window_width = 220.0; let window_height = 32.0;` 然后 `set_position` 同时考虑 y=0
3. `NotchView.vue` 移除 onMount 时的首次 `invokeSetWindowSize(220, 32)` 调用（`tauri.conf.json` 已声明该尺寸），仅保留 `animateExpanded` watcher 内的尺寸同步调用
4. 测试：`main.rs` 的 `center_island_window(screen_w, screen_h, scale) -> (f64, f64)` 抽纯函数做单测

### 7.2 F8：订阅协程 Lagged 容错

`main.rs` 订阅协程改写（见设计块 B §7）：

```rust
use tokio::sync::broadcast::error::RecvError;

tauri::async_runtime::spawn(async move {
    loop {
        match event_rx.recv().await {
            Ok(event) => {
                let _ = app_handle.emit(event.topic(), ());
            }
            Err(RecvError::Lagged(n)) => {
                log::warn!("[event-bus] 订阅滞后，丢失 {} 条事件", n);
            }
            Err(RecvError::Closed) => break,
        }
    }
});
```

抽 `handle_event_bus_recv(recv: Result<AppEvent, RecvError>) -> EventBusAction` 为纯函数可测。`EventBusAction` 枚举：`Emit(topic)` / `Warn(n)` / `Break`。

### 7.3 F13：CI 增加前端测试步骤

`.github/workflows/build-windows.yml` 当前步骤：

```yaml
- run: npm ci
- run: npx tauri icon
- name: Run Rust tests
  working-directory: src-tauri
  run: cargo test
- uses: actions/upload-artifact@v4
  with:
    name: generated-ts
    path: src/types/generated.ts
- run: npx tauri build
```

修复：在 `cargo test` 之后、`npx tauri build` 之前插入：

```yaml
- name: Run frontend tests
  run: npm run test

- name: Type check + vite build (frontend)
  run: npm run build
```

`CLAUDE.md` 的 CI 段落同步更新：`npm ci → tauri icon → cargo test → npm run test → npm run build → tauri build`。

---

## 8. 数据流

Stage 3 新增的事件 + 调用链：

```
[用户打开 Settings 窗口]
  tray 菜单点击 open-settings
    → main.rs on_menu_event → show_window(app, "settings")
      → settings 窗口 show + set_focus
        → webview 加载 index.html
          → main.ts 读 getCurrentWindow().label = "settings"
            → router.push("/settings")
              → SettingsView 挂载

[用户拖动 NotchLiveEdit 窗口调整刘海位置]
  NotchLiveEditView 里 useDragResize 组合 invoke('set_window_position')
    → window_control::set_window_position
      → event_tx.send(AppEvent::NotchPositionChanged)
        → main.rs 订阅协程 app_handle.emit("codeisland:notch:position-changed")
          → 前端 NotchView 监听 → invoke('get_window_position', {label: 'island'})
            → 持久化到 useSettingsStore（Stage 1 已有）
```

---

## 9. 错误处理

### 9.1 Rust 命令错误前缀

- `[window]`：`open_view_window` / `get_window_position` 错误
- `[autostart]`：`get_autostart` / `set_autostart` 错误
- `[event-bus]`：订阅协程 warn 日志前缀

### 9.2 窗口未创建的降级

`open_view_window` 若 `get_webview_window` 返 `None`（理论上不可能，但窗口被强制销毁时可能发生）：返 `Err("[window] 窗口未创建: <label>")`，前端 `ActionButton` 会 toast。

### 9.3 autostart 的 Windows API 失败

`get_autostart` 若 Run 子键无法打开（权限不足等）→ 保守返 `Ok(false)`，不 panic；`set_autostart` 失败返带 `[autostart]` 前缀的 `Err(String)`，Settings 里 Toggle 回滚。

### 9.4 路由未匹配

Router 通配回退到 `/island`，避免白屏。同时 `log::warn!` 提示 label 异常（Rust 端无法直接 log，但前端可 `console.warn` + `invoke('log_warn', msg)` —— 但本 Stage 不引入此基础设施，仅靠通配回退足够）。

---

## 10. 测试

### 10.1 Rust 单测

| 文件 | 测试内容 |
|---|---|
| `app_state.rs` | `AppEvent::NotchPositionChanged.topic()` 返 `"codeisland:notch:position-changed"` |
| `commands/window_control.rs` | `validate_window_label("island")` Ok；`validate_window_label("unknown")` Err 且前缀 `[window]` |
| `autostart/windows.rs` | `roundtrip_set_get_delete` 用 `CodeIsland-test-{pid}` 键名完整验证 set/get/delete |
| `main.rs` 或 `app_state.rs` | `handle_event_bus_recv(Ok(...))` 返 `Emit`；`Err(Lagged(3))` 返 `Warn(3)`；`Err(Closed)` 返 `Break` |
| `main.rs` | `center_island_window(1920, 1080, 1.0)` 返 `(850.0, 0.0)`（(1920-220)/2）等 |

### 10.2 前端单测

| 文件 | 测试内容 |
|---|---|
| `src/main.spec.ts` | mock `getCurrentWindow().label`，断言 `router.push` 以正确 path 调用 |
| `src/router/index.spec.ts` | 通配路由 `/unknown` 重定向到 `/island` |
| `src/components/notch/NotchMenuPopover.spec.ts` | 点击菜单项触发 `open-view` 事件且 label 正确 |
| `src/views/NotchView.spec.ts`（扩展） | 展开态「⋯」按钮存在；点击切换 popoverOpen；popover 关闭回调 |

### 10.3 CI smoke（自动）

Windows CI `build-windows.yml` 跑满：`cargo test → npm run test → npm run build → tauri build`。任一环节红则 CI 红。

### 10.4 人工 smoke（Windows 环境）

1. 启动 exe，主刘海显示在屏幕顶部居中 220×32
2. tray 图标右键 → 菜单显示 5 条 View 入口
3. 点击「偏好设置」→ Settings 窗口打开；关闭按钮 → 窗口隐藏；再次点击 tray 菜单 → 同一窗口 show（单例化生效）
4. 依次测试 Buddy / Usage / Presets / Live Edit 同上
5. 主刘海 hover 展开 → 右上「⋯」按钮可见 → 点击 popover 显示 5 条 → 点击任一条打开对应窗口
6. Settings 里切换「开机启动」Toggle → 重启 Windows → Code Island 自动启动
7. 打开多个 tray 菜单项 → 切换焦点 → 无窗口丢失

---

## 11. 提交计划

10 笔 atomic commit，主会话串行推进（路径 1）：

| # | 类型 | 主题 | 关键文件 |
|---|---|---|---|
| 1 | `[build\|config\|windows][公共]` | tauri.conf.json 声明 5 个独立 View 窗口（initial hidden） | `tauri.conf.json` |
| 2 | `[feat\|UI\|router][公共]` | 引入 vue-router 按 window.label 分发 6 个 View 路由 | `src/router/index.ts` + `src/main.ts` + `src/App.vue` |
| 3 | `[feat\|core\|window-control][公共]` | open_view_window 命令 + close_requested 隐藏策略 | `commands/window_control.rs` + `main.rs` |
| 4 | `[feat\|all\|tray][公共]` | tray 菜单扩展 5 View 入口 | `main.rs` |
| 5 | `[feat\|UI\|notch][公共]` | 刘海右上「⋯」按钮 + NotchMenuPopover | `NotchView.vue` + `NotchMenuPopover.vue` |
| 6 | `[feat\|all\|app-event][公共]` | AppEvent::NotchPositionChanged + get_window_position 命令 | `app_state.rs` + `window_control.rs` + `main.rs` + NotchLiveEditView |
| 7 | `[feat\|core\|autostart][公共]` | 用 windows crate Win32_Registry 实现 HKCU Run 真实读写 | `autostart/windows.rs` + `Cargo.toml` |
| 8 | `[fix\|core\|main][公共]` | F5 窗口初始 COLLAPSED 尺寸居中，消除 watcher race | `main.rs` + `NotchView.vue` |
| 9 | `[fix\|core\|event-bus][公共]` | F8 订阅协程 loop match Lagged warn 防退出 | `main.rs` |
| 10 | `[ci\|build\|workflow][公共]` | F13 Windows CI 增加 npm run test + npm run build 步骤 + CLAUDE.md 同步 | `.github/workflows/build-windows.yml` + `CLAUDE.md` |

每笔 commit 前跑 `cargo test` + `npm run test` + `npm run build` 验证。commit 消息规范遵循全局 CLAUDE.md。

每批（例如 3-4 笔）后推送一次到 `origin feat/stage0-windows-alignment`（分支保持不变，或按需新开 `feat/stage3-view-routing`），触发 Windows CI 验证。

---

## 12. 风险与回滚

### 12.1 风险

- **R1. `Win32_System_Registry` API 使用的 unsafe 块复杂度**：缓解措施为先写单测覆盖 roundtrip，CI 验证后再合入主分支
- **R2. `tauri.conf.json` 从 1 窗口改为 6 窗口可能触发首次启动异常**：缓解措施为 Commit 1 独立提交后立即本地/CI 冒烟，失败则 revert
- **R3. Router 切换后首次渲染若 `router.push` async 未完成可能白屏**：`await router.push` 再 `mount` 避免
- **R4. specta regen 在 Stage 3 改动 AppEvent enum 与新增命令后，本地 `generated.ts` 与 CI 产物漂移**：按既定模式 `gh run download -n generated-ts` 回灌

### 12.2 回滚单位

每笔 commit 独立 revertable。若 Commit 7（autostart 真实实现）在真实 Windows 环境出 bug，`git revert` 后仅回到 placeholder 状态，不影响其他 9 笔。

---

## 13. 非目标明示（拒绝绕道）

以下内容本 Stage 明确不做，避免扩张：

- 不引入 `markdown-it` / `shiki`（MarkdownRenderer 保持正则 placeholder）
- 不引入 `uuid` crate（preset id 保持 timestamp）
- 不引入 `@tauri-apps/plugin-shell`（`std::process::Command` 够用）
- 不引入 `winreg` crate（用 `windows` crate 原生 API）
- 不做 SessionSummary 字段扩展（F16）
- 不做 F3 installer `.expect()` 加固 / F4 hook 脚本 `with open` / F6 rAF race / F7 `.notch-svg path` 选择器 / F10 markdown 表格 `|` 转义 / F11 BuddyState 契约补 / F12 StatusDot size prop
- 不做全局快捷键注册（tray 菜单提示 `Ctrl+Shift+\` 的 UX 暂缓）

未来 Stage 4 或 Stage 3.x 可按需消化。

---

## 14. 验收

Stage 3 完成判据：

1. 10 笔 commit 全部 push 到 `feat/stage0-windows-alignment`（或新分支）
2. Windows CI 全绿（`cargo test → npm run test → npm run build → tauri build` 全部成功）
3. 人工 smoke §10.4 全部通过
4. follow-up F5 / F8 / F13 / F14 在 Vault 笔记标记为 ✅
5. CLAUDE.md 同步当前 CI 描述与 Stage 3 状态
6. `docs/superpowers/plans/2026-04-18-stage3-*.md` 实施计划已 commit（writing-plans skill 产出）

---

## 引用

- Stage 0/1/2 设计：`docs/superpowers/specs/2026-04-17-windows-ui-alignment-design.md`
- Stage 1 并行契约：`docs/superpowers/specs/2026-04-17-windows-ui-alignment-contracts.md`
- follow-ups 清单：Vault `项目笔记/CodeIsland-win Stage 0 follow-ups.md`
- CI 经验：Vault `技术笔记/CodeIsland-win Stage 1-2 整合与 CI 收敛经验.md`
- 项目导航：`CLAUDE.md`
