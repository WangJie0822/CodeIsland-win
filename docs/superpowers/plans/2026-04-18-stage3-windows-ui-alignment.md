# CodeIsland-win Stage 3 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 把 Stage 1 产出的 5 个 View 通过多窗口独立 + vue-router 分发 + tray/刘海双入口接入可达路径；autostart 升级为真实 HKCU\Run 读写；修 F5/F8/F13/F14 follow-ups。

**Architecture:** 6 个独立 Tauri 窗口共用 `index.html`，前端 vue-router 按 `getCurrentWindow().label` 分发到对应 View。tray 菜单与刘海右上「⋯」按钮均通过新增 `open_view_window(label)` 命令 show/focus 已存在的窗口；关闭走 `close_requested` + `hide()` 隐藏策略。autostart 用 `windows` crate 的 `Win32_System_Registry` feature 真实读写 HKCU\Software\Microsoft\Windows\CurrentVersion\Run 键，无新 crate 依赖。

**Tech Stack:** Tauri 2 / vue-router 4 / `windows` crate 0.58 (Win32_System_Registry feature) / Vitest 2 / cargo test + specta-typescript。

**关联文档：**
- Spec: `docs/superpowers/specs/2026-04-18-stage3-windows-ui-alignment-design.md`
- Stage 1 契约: `docs/superpowers/specs/2026-04-17-windows-ui-alignment-contracts.md`
- Vault 经验: `~/.claude/Vault/技术笔记/CodeIsland-win Stage 1-2 整合与 CI 收敛经验.md`

**重要约束：**
1. specta 四类禁忌：`<R: Runtime>` 泛型、`usize/u64/i64` 返回、`Option<serde_json::Value>` 字段 + Map/Record 消费、`Option<&String>` + `unwrap_or_default`
2. 错误前缀：`[window]` / `[autostart]` / `[event-bus]`
3. 每 Task 结束一笔 atomic commit，commit message 严格按全局 CLAUDE.md 规范 `[类型|模块|功能][公共]描述`
4. 每 Task 前务必先 `pwd` 验证为 `/Users/wj/Work/OpenSource/CodeIsland-win`

---

## 前置：工作目录验证

- [ ] **Step 0.1: 验证 CWD**

```bash
pwd
```

Expected: `/Users/wj/Work/OpenSource/CodeIsland-win`

- [ ] **Step 0.2: 验证分支**

```bash
git branch --show-current
```

Expected: `feat/stage0-windows-alignment`（或新开 `feat/stage3-view-routing`——由执行者决定，但本 plan 假设在 `feat/stage0-windows-alignment` 上继续）

- [ ] **Step 0.3: 确认 working tree 干净（除 CLAUDE.md/icons）**

```bash
git status --short
```

Expected: 仅 ` M CLAUDE.md`、`?? src-tauri/icons/android/`、`?? src-tauri/icons/ios/`

---

## Task 1: tauri.conf.json 声明 6 窗口拓扑

**Files:**
- Modify: `src-tauri/tauri.conf.json`

**目标：** 把 `app.windows` 数组从 1 窗口扩到 6 窗口。主刘海初始尺寸从 400×48 改为 220×32（F5 修复的第 1 步）。其余 5 窗口初始 `visible: false`。

**验证方式：** Rust 的 `cargo test` 会触发 `tauri-build` 解析 `tauri.conf.json`，解析失败即编译失败。无单独单测。

- [ ] **Step 1.1: 读取当前 tauri.conf.json**

```bash
cat src-tauri/tauri.conf.json
```

Expected: 确认当前 `windows` 数组只有 1 个 `island` 条目，尺寸 400×48。

- [ ] **Step 1.2: 替换 windows 数组为 6 条声明**

完整替换 `src-tauri/tauri.conf.json` 为：

```json
{
  "$schema": "https://raw.githubusercontent.com/tauri-apps/tauri/dev/crates/tauri-config-schema/schema.json",
  "productName": "Code Island",
  "version": "0.1.0",
  "identifier": "com.codeisland.win",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:5173",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  },
  "app": {
    "windows": [
      {
        "label": "island",
        "title": "Code Island",
        "width": 220,
        "height": 32,
        "decorations": false,
        "transparent": true,
        "alwaysOnTop": true,
        "skipTaskbar": true,
        "resizable": false
      },
      {
        "label": "settings",
        "title": "Code Island - 偏好设置",
        "width": 720,
        "height": 520,
        "decorations": false,
        "transparent": false,
        "alwaysOnTop": false,
        "skipTaskbar": false,
        "resizable": false,
        "visible": false,
        "center": true
      },
      {
        "label": "buddy",
        "title": "Code Island - 伙伴",
        "width": 480,
        "height": 640,
        "decorations": false,
        "transparent": false,
        "alwaysOnTop": false,
        "skipTaskbar": false,
        "resizable": false,
        "visible": false,
        "center": true
      },
      {
        "label": "usage",
        "title": "Code Island - 用量报告",
        "width": 720,
        "height": 520,
        "decorations": false,
        "transparent": false,
        "alwaysOnTop": false,
        "skipTaskbar": false,
        "resizable": false,
        "visible": false,
        "center": true
      },
      {
        "label": "presets",
        "title": "Code Island - 启动预设",
        "width": 720,
        "height": 560,
        "decorations": false,
        "transparent": false,
        "alwaysOnTop": false,
        "skipTaskbar": false,
        "resizable": false,
        "visible": false,
        "center": true
      },
      {
        "label": "notch-live-edit",
        "title": "Code Island - Live Edit",
        "width": 320,
        "height": 200,
        "decorations": false,
        "transparent": true,
        "alwaysOnTop": true,
        "skipTaskbar": true,
        "resizable": false,
        "visible": false,
        "center": true
      }
    ],
    "security": {
      "csp": null
    }
  }
}
```

- [ ] **Step 1.3: 验证 JSON 合法**

```bash
node -e "JSON.parse(require('fs').readFileSync('src-tauri/tauri.conf.json','utf8'));console.log('ok')"
```

Expected: `ok`

- [ ] **Step 1.4: 提交**

```bash
git add src-tauri/tauri.conf.json
git commit -m "[build|config|windows][公共]tauri.conf.json 声明 5 个独立 View 窗口（initial hidden），主刘海尺寸调整为 220×32"
```

---

## Task 2: vue-router 接入 + label 分发

**Files:**
- Create: `src/router/index.ts`
- Create: `src/router/index.spec.ts`
- Create: `src/main.spec.ts`
- Modify: `src/main.ts`
- Modify: `src/App.vue`

**目标：** 引入 vue-router 4，按 `getCurrentWindow().label` 自动路由到对应 View。TDD 先行。

- [ ] **Step 2.1: 写失败测试 · 路由通配回退**

创建 `src/router/index.spec.ts`：

```typescript
import { describe, it, expect } from "vitest";
import { router } from "./index";

describe("router", () => {
  it("/unknown-path 重定向到 /island", async () => {
    await router.push("/unknown-path");
    expect(router.currentRoute.value.path).toBe("/island");
  });

  it("/settings 路由匹配成功", async () => {
    await router.push("/settings");
    expect(router.currentRoute.value.path).toBe("/settings");
  });

  it("/buddy /usage /presets /notch-live-edit 均匹配成功", async () => {
    const labels = ["buddy", "usage", "presets", "notch-live-edit"] as const;
    for (const label of labels) {
      await router.push(`/${label}`);
      expect(router.currentRoute.value.path).toBe(`/${label}`);
    }
  });
});
```

- [ ] **Step 2.2: 运行测试验证失败**

```bash
npm run test -- src/router/index.spec.ts
```

Expected: FAIL，提示 `src/router/index` 未找到。

- [ ] **Step 2.3: 创建 `src/router/index.ts`**

```typescript
import { createRouter, createMemoryHistory, type RouteRecordRaw } from "vue-router";
import NotchView from "@/views/NotchView.vue";
import SettingsView from "@/views/SettingsView.vue";
import BuddyCardView from "@/views/BuddyCardView.vue";
import DailyReportCardView from "@/views/DailyReportCardView.vue";
import LaunchPresetsView from "@/views/LaunchPresetsView.vue";
import NotchLiveEditView from "@/views/NotchLiveEditView.vue";

const routes: RouteRecordRaw[] = [
  { path: "/island", component: NotchView },
  { path: "/settings", component: SettingsView },
  { path: "/buddy", component: BuddyCardView },
  { path: "/usage", component: DailyReportCardView },
  { path: "/presets", component: LaunchPresetsView },
  { path: "/notch-live-edit", component: NotchLiveEditView },
  { path: "/:pathMatch(.*)*", redirect: "/island" },
];

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
});
```

- [ ] **Step 2.4: 运行测试验证通过**

```bash
npm run test -- src/router/index.spec.ts
```

Expected: PASS (3 tests)

- [ ] **Step 2.5: 写失败测试 · main.ts 分发逻辑**

先把 `src/main.ts` 的核心逻辑抽为可测函数。创建 `src/main.spec.ts`：

```typescript
import { describe, it, expect, vi, beforeEach } from "vitest";

const pushMock = vi.fn();

vi.mock("./router", () => ({
  router: {
    push: pushMock,
  },
}));

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: vi.fn(),
}));

import { resolveRoutePath } from "./main";

describe("resolveRoutePath", () => {
  beforeEach(() => {
    pushMock.mockClear();
  });

  it("label 为 'island' 时返回 '/island'", () => {
    expect(resolveRoutePath("island")).toBe("/island");
  });

  it("label 为 'settings' 时返回 '/settings'", () => {
    expect(resolveRoutePath("settings")).toBe("/settings");
  });

  it("label 为 'notch-live-edit' 时返回 '/notch-live-edit'", () => {
    expect(resolveRoutePath("notch-live-edit")).toBe("/notch-live-edit");
  });

  it("空 label 兜底 '/island'", () => {
    expect(resolveRoutePath("")).toBe("/island");
  });

  it("未知 label 兜底 '/island'", () => {
    expect(resolveRoutePath("foobar")).toBe("/island");
  });
});
```

- [ ] **Step 2.6: 运行测试验证失败**

```bash
npm run test -- src/main.spec.ts
```

Expected: FAIL，`resolveRoutePath` 未导出。

- [ ] **Step 2.7: 重写 `src/main.ts`**

```typescript
import { createApp } from "vue";
import { createPinia } from "pinia";
import { getCurrentWindow } from "@tauri-apps/api/window";
import App from "./App.vue";
import { router } from "./router";

const KNOWN_LABELS = new Set([
  "island",
  "settings",
  "buddy",
  "usage",
  "presets",
  "notch-live-edit",
]);

export function resolveRoutePath(label: string): string {
  if (!label || !KNOWN_LABELS.has(label)) {
    return "/island";
  }
  return `/${label}`;
}

async function bootstrap(): Promise<void> {
  const label = getCurrentWindow().label;
  const path = resolveRoutePath(label);

  const app = createApp(App);
  app.use(createPinia());
  app.use(router);
  await router.push(path);
  app.mount("#app");
}

void bootstrap();
```

- [ ] **Step 2.8: 运行测试验证通过**

```bash
npm run test -- src/main.spec.ts
```

Expected: PASS (5 tests)

- [ ] **Step 2.9: 重写 `src/App.vue`**

```vue
<script setup lang="ts"></script>

<template>
  <RouterView />
</template>

<style scoped></style>
```

- [ ] **Step 2.10: 全量跑前端单测确认无回归**

```bash
npm run test
```

Expected: 所有前端测试 PASS（含新增 8 条）。

- [ ] **Step 2.11: Type check 验证**

```bash
npm run build
```

Expected: `vue-tsc --noEmit` 通过 + vite build 成功。

- [ ] **Step 2.12: 提交**

```bash
git add src/router/index.ts src/router/index.spec.ts src/main.ts src/main.spec.ts src/App.vue
git commit -m "[feat|UI|router][公共]引入 vue-router 按 window.label 分发 6 个 View 路由，App.vue 切换为 RouterView"
```

---

## Task 3: open_view_window 命令 + 窗口隐藏策略

**Files:**
- Modify: `src-tauri/src/commands/window_control.rs`
- Modify: `src-tauri/src/main.rs`（invoke_handler 注册 + close_requested handler）
- Modify: `src/lib/tauri.ts`

**目标：** 添加 `open_view_window` 命令（show + focus），抽 `validate_window_label` 纯函数可测；为 5 个非 island 窗口注册 `close_requested` → `api.prevent_close() + hide()`；前端暴露 invoke 封装。

- [ ] **Step 3.1: 写失败测试 · validate_window_label**

编辑 `src-tauri/src/commands/window_control.rs`，在文件末尾的 `#[cfg(test)]` 块（若无则新建）加测试：

先读当前文件：

```bash
cat src-tauri/src/commands/window_control.rs | head -50
```

找到现有 `#[cfg(test)] mod tests { ... }`（Stage 1 已为 validate_size 写过），在 `mod tests` 末尾新增：

```rust
    #[test]
    fn validate_window_label_accepts_known() {
        assert!(super::validate_window_label("island").is_ok());
        assert!(super::validate_window_label("settings").is_ok());
        assert!(super::validate_window_label("buddy").is_ok());
        assert!(super::validate_window_label("usage").is_ok());
        assert!(super::validate_window_label("presets").is_ok());
        assert!(super::validate_window_label("notch-live-edit").is_ok());
    }

    #[test]
    fn validate_window_label_rejects_unknown_with_prefix() {
        let err = super::validate_window_label("foobar").unwrap_err();
        assert!(
            err.starts_with("[window]"),
            "error should start with [window] prefix, got: {}",
            err
        );
        assert!(err.contains("foobar"), "error should contain label, got: {}", err);
    }

    #[test]
    fn validate_window_label_rejects_empty() {
        assert!(super::validate_window_label("").is_err());
    }
```

- [ ] **Step 3.2: 运行测试验证失败**

```bash
cd src-tauri && cargo test validate_window_label 2>&1 | tail -30
```

Expected: FAIL / 编译错误——`validate_window_label` 未定义。

- [ ] **Step 3.3: 添加 KNOWN_WINDOW_LABELS + validate_window_label**

在 `src-tauri/src/commands/window_control.rs` 顶部（use 语句之后、现有函数之前）新增：

```rust
pub const KNOWN_WINDOW_LABELS: &[&str] = &[
    "island",
    "settings",
    "buddy",
    "usage",
    "presets",
    "notch-live-edit",
];

pub fn validate_window_label(label: &str) -> Result<(), String> {
    if KNOWN_WINDOW_LABELS.contains(&label) {
        Ok(())
    } else {
        Err(format!("[window] 未知窗口 label: {}", label))
    }
}
```

- [ ] **Step 3.4: 运行测试验证通过**

```bash
cd src-tauri && cargo test validate_window_label 2>&1 | tail -30
```

Expected: PASS (3 tests)

- [ ] **Step 3.5: 添加 open_view_window 命令**

在 `src-tauri/src/commands/window_control.rs` 现有命令函数之后新增：

```rust
#[tauri::command]
#[specta::specta]
pub async fn open_view_window(app: tauri::AppHandle, label: String) -> Result<(), String> {
    validate_window_label(&label)?;
    let window = app
        .get_webview_window(&label)
        .ok_or_else(|| format!("[window] 窗口未创建: {}", label))?;
    window
        .show()
        .map_err(|e| format!("[window] show 失败: {}", e))?;
    window
        .set_focus()
        .map_err(|e| format!("[window] focus 失败: {}", e))?;
    Ok(())
}
```

- [ ] **Step 3.6: 注册命令到 main.rs**

编辑 `src-tauri/src/main.rs`，在 `invoke_handler!` 宏的列表中加入 `commands::window_control::open_view_window`。找到现有：

```rust
commands::window_control::set_ignore_cursor_events,
commands::window_control::set_window_size,
commands::window_control::set_window_position,
```

在其后紧跟：

```rust
commands::window_control::open_view_window,
```

- [ ] **Step 3.7: 添加 close_requested hide 策略**

在 `src-tauri/src/main.rs` 的 `setup(|app| { ... })` 块里，tray 代码之前（`let show_item = ...` 之前），插入：

```rust
// Stage 3: 为 5 个非 island 窗口注册 close_requested → hide()，保留实例
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

- [ ] **Step 3.8: 跑全量 cargo test 验证无回归**

```bash
cd src-tauri && cargo test 2>&1 | tail -30
```

Expected: 全部通过 + specta 生成 `src/types/generated.ts` 成功（内含 `openViewWindow` 命令）。

- [ ] **Step 3.9: 暴露前端 invoke 封装**

编辑 `src/lib/tauri.ts`，在文件现有的 invoke 封装后新增：

```typescript
import { commands } from "@/types/generated";

export async function invokeOpenViewWindow(label: string): Promise<void> {
  const result = await commands.openViewWindow(label);
  if (result.status === "error") {
    throw new Error(result.error);
  }
}
```

（若 `@/types/generated` 的 `commands` 结构与此不一致，按 generated.ts 实际导出调整——specta 生成结构参考 Stage 1-2 经验。）

- [ ] **Step 3.10: Type check 验证**

```bash
npm run build
```

Expected: `vue-tsc --noEmit` 通过。

- [ ] **Step 3.11: 提交**

```bash
git add src-tauri/src/commands/window_control.rs src-tauri/src/main.rs src/lib/tauri.ts src/types/generated.ts
git commit -m "[feat|core|window-control][公共]新增 open_view_window 命令 + 5 窗口 close_requested 隐藏策略，抽 validate_window_label 纯函数"
```

---

## Task 4: tray 菜单扩展 5 View 入口

**Files:**
- Modify: `src-tauri/src/main.rs`

**目标：** 将 tray 菜单从「显示窗口 / 退出」扩展为 8 条：显示主刘海 + 分隔符 + 5 View + 分隔符 + 退出。点击处理统一调 `commands::window_control::open_view_window` 逻辑（直接内联 `get_webview_window().show() + set_focus()`，避免 tokio runtime 约束）。

**验证方式：** tray handler 无法单测（依赖 AppHandle）。依赖 Windows CI 冒烟 + 人工验证。

- [ ] **Step 4.1: 读当前 main.rs tray 构建段**

```bash
grep -n "MenuItem::with_id\|Menu::with_items\|on_menu_event" src-tauri/src/main.rs
```

定位现有 tray 代码块（约 98-139 行）。

- [ ] **Step 4.2: 替换 tray 菜单构建**

编辑 `src-tauri/src/main.rs`，找到现有：

```rust
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
```

整块替换为：

```rust
use tauri::menu::PredefinedMenuItem;

let show_island =
    MenuItem::with_id(app, "show-island", "显示主刘海", true, None::<&str>)?;
let sep1 = PredefinedMenuItem::separator(app)?;
let open_settings =
    MenuItem::with_id(app, "open-settings", "偏好设置", true, None::<&str>)?;
let open_buddy = MenuItem::with_id(app, "open-buddy", "伙伴", true, None::<&str>)?;
let open_usage =
    MenuItem::with_id(app, "open-usage", "用量报告", true, None::<&str>)?;
let open_presets =
    MenuItem::with_id(app, "open-presets", "启动预设", true, None::<&str>)?;
let open_live_edit = MenuItem::with_id(
    app,
    "open-notch-live-edit",
    "调整刘海位置",
    true,
    None::<&str>,
)?;
let sep2 = PredefinedMenuItem::separator(app)?;
let quit_item = MenuItem::with_id(
    app,
    "quit",
    "退出 Code Island",
    true,
    None::<&str>,
)?;
let menu = Menu::with_items(
    app,
    &[
        &show_island,
        &sep1,
        &open_settings,
        &open_buddy,
        &open_usage,
        &open_presets,
        &open_live_edit,
        &sep2,
        &quit_item,
    ],
)?;
```

注意：`use tauri::menu::PredefinedMenuItem;` 需要放到文件顶部已有 `use tauri::menu::{Menu, MenuItem};` 同行改为 `use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};`，而不是函数内 use。移除内联 `use`，改顶部 import。

- [ ] **Step 4.3: 替换 on_menu_event 分派**

找到现有 `on_menu_event` 闭包：

```rust
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
```

替换为：

```rust
.on_menu_event(move |app, event| {
    fn show_window(app: &tauri::AppHandle, label: &str) {
        if let Some(window) = app.get_webview_window(label) {
            let _ = window.show();
            let _ = window.set_focus();
        } else {
            log::warn!("[tray] 窗口不存在: {}", label);
        }
    }

    match event.id.as_ref() {
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
    }
})
```

- [ ] **Step 4.4: 跑 cargo test 确认编译 + 无回归**

```bash
cd src-tauri && cargo test 2>&1 | tail -30
```

Expected: 全部通过。

- [ ] **Step 4.5: 提交**

```bash
git add src-tauri/src/main.rs
git commit -m "[feat|all|tray][公共]tray 菜单扩展 5 个 View 入口，点击统一 show+focus 目标窗口"
```

---

## Task 5: 刘海右上「⋯」按钮 + NotchMenuPopover

**Files:**
- Create: `src/components/notch/NotchMenuPopover.vue`
- Create: `src/components/notch/NotchMenuPopover.spec.ts`
- Modify: `src/views/NotchView.vue`

**目标：** 刘海展开态右上角加一个按钮，点击弹出与 tray 相同的 5 条 View 菜单。用 `<Teleport to="body">` 避免被刘海 SVG clipPath 裁掉。

- [ ] **Step 5.1: 写失败测试 · NotchMenuPopover**

创建 `src/components/notch/NotchMenuPopover.spec.ts`：

```typescript
import { describe, it, expect, vi } from "vitest";
import { mount, flushPromises } from "@vue/test-utils";
import NotchMenuPopover from "./NotchMenuPopover.vue";

describe("NotchMenuPopover", () => {
  it("渲染 5 个菜单项：settings / buddy / usage / presets / notch-live-edit", () => {
    const w = mount(NotchMenuPopover, { attachTo: document.body });
    const items = w.findAll("[data-popover-item]");
    expect(items.length).toBe(5);
    const labels = items.map((i) => i.attributes("data-popover-item"));
    expect(labels).toEqual([
      "settings",
      "buddy",
      "usage",
      "presets",
      "notch-live-edit",
    ]);
  });

  it("点击菜单项 emit 'open-view' 事件携带 label", async () => {
    const w = mount(NotchMenuPopover, { attachTo: document.body });
    await w.find("[data-popover-item='settings']").trigger("click");
    expect(w.emitted("open-view")).toBeTruthy();
    expect(w.emitted("open-view")?.[0]).toEqual(["settings"]);
  });

  it("点击外部 emit 'close' 事件", async () => {
    const w = mount(NotchMenuPopover, { attachTo: document.body });
    await flushPromises();
    // 点击 body（非 popover 内部）
    document.body.click();
    await flushPromises();
    expect(w.emitted("close")).toBeTruthy();
  });

  it("菜单项显示中文标签", () => {
    const w = mount(NotchMenuPopover, { attachTo: document.body });
    const texts = w.findAll("[data-popover-item]").map((i) => i.text());
    expect(texts).toEqual(["偏好设置", "伙伴", "用量报告", "启动预设", "调整刘海位置"]);
  });
});
```

- [ ] **Step 5.2: 运行测试验证失败**

```bash
npm run test -- src/components/notch/NotchMenuPopover.spec.ts
```

Expected: FAIL，`NotchMenuPopover.vue` 不存在。

- [ ] **Step 5.3: 创建 NotchMenuPopover.vue**

```vue
<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";

interface MenuItemDef {
  label: string;
  viewLabel: string;
}

const items: MenuItemDef[] = [
  { label: "偏好设置", viewLabel: "settings" },
  { label: "伙伴", viewLabel: "buddy" },
  { label: "用量报告", viewLabel: "usage" },
  { label: "启动预设", viewLabel: "presets" },
  { label: "调整刘海位置", viewLabel: "notch-live-edit" },
];

const emit = defineEmits<{
  "open-view": [label: string];
  close: [];
}>();

const rootRef = ref<HTMLDivElement | null>(null);

function onItemClick(viewLabel: string): void {
  emit("open-view", viewLabel);
}

function onDocClick(e: MouseEvent): void {
  if (!rootRef.value) return;
  if (!rootRef.value.contains(e.target as Node)) {
    emit("close");
  }
}

onMounted(() => {
  // 下一个 tick 注册，避免开启 popover 的那次点击立即触发 close
  setTimeout(() => {
    document.addEventListener("click", onDocClick, true);
  }, 0);
});

onUnmounted(() => {
  document.removeEventListener("click", onDocClick, true);
});
</script>

<template>
  <Teleport to="body">
    <div ref="rootRef" class="notch-popover">
      <button
        v-for="item in items"
        :key="item.viewLabel"
        class="notch-popover-item"
        :data-popover-item="item.viewLabel"
        type="button"
        @click="onItemClick(item.viewLabel)"
      >
        {{ item.label }}
      </button>
    </div>
  </Teleport>
</template>

<style scoped>
.notch-popover {
  position: fixed;
  top: 40px;
  right: 12px;
  min-width: 180px;
  background: var(--bg-notch, rgba(20, 20, 24, 0.95));
  border: 1px solid rgba(255, 255, 255, 0.12);
  border-radius: 8px;
  padding: 4px 0;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
  z-index: 1000;
  display: flex;
  flex-direction: column;
  font-family: var(--font-sans);
  font-size: 13px;
}

.notch-popover-item {
  background: transparent;
  border: none;
  color: var(--text-primary, #fff);
  padding: 8px 14px;
  text-align: left;
  cursor: pointer;
  transition: background 100ms;
}

.notch-popover-item:hover {
  background: rgba(255, 255, 255, 0.08);
}
</style>
```

- [ ] **Step 5.4: 运行测试验证通过**

```bash
npm run test -- src/components/notch/NotchMenuPopover.spec.ts
```

Expected: PASS (4 tests)

- [ ] **Step 5.5: 在 NotchView 中加入「⋯」按钮 + popover 控制**

读取 `src/views/NotchView.vue`：

```bash
cat src/views/NotchView.vue | head -100
```

在 `<script setup>` 顶部（其他 import 之后）加：

```typescript
import NotchMenuPopover from "@/components/notch/NotchMenuPopover.vue";
import { invokeOpenViewWindow } from "@/lib/tauri";
import { ref } from "vue";  // 若未 import

const popoverOpen = ref(false);

function togglePopover(): void {
  popoverOpen.value = !popoverOpen.value;
}

async function onOpenView(label: string): Promise<void> {
  popoverOpen.value = false;
  try {
    await invokeOpenViewWindow(label);
  } catch (e) {
    console.warn("[notch] 打开窗口失败:", e);
  }
}

function onPopoverClose(): void {
  popoverOpen.value = false;
}
```

（若 `ref` 已在 import，不重复加。）

- [ ] **Step 5.6: 在 NotchView 模板中加入按钮 + popover**

找到 `<template>` 中刘海展开态的布局（`notch.expanded` 分支，或 `.notch-content` 容器），在其右上角加一个按钮触发 `togglePopover`。具体 DOM 结构依赖当前 NotchView 模板（参考现有 `sessionCount` / `countLabel` 展开态渲染）。示例（假设存在 `.notch-expanded-content` 容器）：

```vue
<div v-if="notch.expanded" class="notch-expanded-content">
  <!-- 现有内容 ... -->

  <button
    class="notch-menu-btn"
    data-testid="notch-menu-button"
    type="button"
    aria-label="更多菜单"
    @click.stop="togglePopover"
  >
    ⋯
  </button>
</div>

<NotchMenuPopover
  v-if="popoverOpen"
  @open-view="onOpenView"
  @close="onPopoverClose"
/>
```

配套样式：

```css
.notch-menu-btn {
  position: absolute;
  top: 6px;
  right: 10px;
  width: 22px;
  height: 22px;
  background: transparent;
  border: none;
  color: var(--text-tertiary, rgba(255, 255, 255, 0.6));
  cursor: pointer;
  font-size: 16px;
  line-height: 1;
  padding: 0;
  border-radius: 4px;
}
.notch-menu-btn:hover {
  background: rgba(255, 255, 255, 0.12);
  color: var(--text-primary, #fff);
}
```

- [ ] **Step 5.7: 全量跑前端单测**

```bash
npm run test
```

Expected: 所有测试 PASS，含新增 NotchMenuPopover (4)。

- [ ] **Step 5.8: Type check 验证**

```bash
npm run build
```

Expected: `vue-tsc --noEmit` 通过。

- [ ] **Step 5.9: 提交**

```bash
git add src/components/notch/NotchMenuPopover.vue src/components/notch/NotchMenuPopover.spec.ts src/views/NotchView.vue
git commit -m "[feat|UI|notch][公共]刘海右上「⋯」按钮 + NotchMenuPopover 弹层，点击菜单项 invoke open_view_window"
```

---

## Task 6: AppEvent::NotchPositionChanged + get_window_position 命令

**Files:**
- Modify: `src-tauri/src/app_state.rs`
- Modify: `src-tauri/src/commands/window_control.rs`
- Modify: `src-tauri/src/main.rs`
- Modify: `src/views/NotchLiveEditView.vue`
- Modify: `src/lib/tauri.ts`

**目标：** 扩展 AppEvent + 新命令 get_window_position 返回刘海当前坐标；NotchLiveEdit 切换使用 specta 生成的 WindowPosition 类型。

- [ ] **Step 6.1: 写失败测试 · AppEvent::NotchPositionChanged.topic**

编辑 `src-tauri/src/app_state.rs`，在现有 `#[cfg(test)] mod tests` 块末尾加：

```rust
    #[test]
    fn notch_position_changed_topic() {
        assert_eq!(
            AppEvent::NotchPositionChanged.topic(),
            "codeisland:notch:position-changed"
        );
    }
```

- [ ] **Step 6.2: 运行测试验证失败**

```bash
cd src-tauri && cargo test notch_position_changed_topic 2>&1 | tail -20
```

Expected: FAIL / 编译错误——`AppEvent::NotchPositionChanged` 未定义。

- [ ] **Step 6.3: 扩展 AppEvent enum + topic**

编辑 `src-tauri/src/app_state.rs`，找到现有 `pub enum AppEvent` 定义，在最后一个变体后加 `NotchPositionChanged,`（注意保持 derive 完整）。

然后在 `impl AppEvent { pub fn topic(&self) -> &'static str { match self { ... } } }` 的 match 里加：

```rust
Self::NotchPositionChanged => "codeisland:notch:position-changed",
```

- [ ] **Step 6.4: 运行测试验证通过**

```bash
cd src-tauri && cargo test notch_position_changed_topic 2>&1 | tail -10
```

Expected: PASS

- [ ] **Step 6.5: 写失败测试 · WindowPosition + get_window_position 结构**

在 `src-tauri/src/commands/window_control.rs` 的 `#[cfg(test)] mod tests` 末尾加：

```rust
    #[test]
    fn window_position_fields_are_i32() {
        let pos = super::WindowPosition { x: 100i32, y: 200i32 };
        assert_eq!(pos.x, 100);
        assert_eq!(pos.y, 200);
    }
```

- [ ] **Step 6.6: 运行测试验证失败**

```bash
cd src-tauri && cargo test window_position_fields_are_i32 2>&1 | tail -20
```

Expected: FAIL / 编译错误——`WindowPosition` 未定义。

- [ ] **Step 6.7: 添加 WindowPosition + get_window_position**

在 `src-tauri/src/commands/window_control.rs` 文件顶部 use 语句后（`KNOWN_WINDOW_LABELS` 定义之前）加：

```rust
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
}
```

在 `open_view_window` 之后加命令函数：

```rust
#[tauri::command]
#[specta::specta]
pub async fn get_window_position(
    app: tauri::AppHandle,
    label: String,
) -> Result<WindowPosition, String> {
    validate_window_label(&label)?;
    let window = app
        .get_webview_window(&label)
        .ok_or_else(|| format!("[window] 窗口未创建: {}", label))?;
    let pos = window
        .outer_position()
        .map_err(|e| format!("[window] 读取位置失败: {}", e))?;
    Ok(WindowPosition { x: pos.x, y: pos.y })
}
```

- [ ] **Step 6.8: 运行测试验证通过**

```bash
cd src-tauri && cargo test window_position 2>&1 | tail -10
```

Expected: PASS

- [ ] **Step 6.9: 注册命令到 main.rs**

编辑 `src-tauri/src/main.rs` 的 `invoke_handler!` 列表，在 `commands::window_control::open_view_window,` 后加：

```rust
commands::window_control::get_window_position,
```

- [ ] **Step 6.10: 跑全量 cargo test 触发 specta regen**

```bash
cd src-tauri && cargo test 2>&1 | tail -30
```

Expected: 全部通过；`src/types/generated.ts` 自动重新生成，内含 `getWindowPosition` 命令 + `WindowPosition` 类型。

- [ ] **Step 6.11: 暴露前端 invoke 封装**

编辑 `src/lib/tauri.ts`，在 `invokeOpenViewWindow` 之后加：

```typescript
import type { WindowPosition } from "@/types/generated";

export async function invokeGetWindowPosition(
  label: string,
): Promise<WindowPosition> {
  const result = await commands.getWindowPosition(label);
  if (result.status === "error") {
    throw new Error(result.error);
  }
  return result.data;
}
```

- [ ] **Step 6.12: 切换 NotchLiveEditView 使用 specta 类型**

编辑 `src/views/NotchLiveEditView.vue`，找到本地 `interface NotchPosition` 声明，删除该 interface，并把所有使用 `NotchPosition` 的地方改为 specta 生成的 `WindowPosition`（注意 `WindowPosition` 没有 `screen_id` 字段，前端 screen_id 独立用 ref 保存）。示例：

读取当前：
```bash
grep -n "NotchPosition" src/views/NotchLiveEditView.vue
```

把：
```typescript
interface NotchPosition {
  x: number;
  y: number;
  screen_id: string;
}
```

删除；把文件内 `NotchPosition` 引用替换为 `WindowPosition`：

```typescript
import type { WindowPosition } from "@/types/generated";
// screen_id 独立保存
const screenId = ref<string>("");
```

（具体 diff 依赖现有 NotchLiveEditView 实现，执行者按实际代码调整字段使用处。注释中 Stage 2 待办的 `NotchPosition` 类型补全标记可删。）

- [ ] **Step 6.13: Type check 验证**

```bash
npm run build
```

Expected: `vue-tsc --noEmit` 通过。

- [ ] **Step 6.14: 提交**

```bash
git add src-tauri/src/app_state.rs src-tauri/src/commands/window_control.rs src-tauri/src/main.rs src/lib/tauri.ts src/views/NotchLiveEditView.vue src/types/generated.ts
git commit -m "[feat|all|app-event][公共]AppEvent::NotchPositionChanged + get_window_position 命令，NotchLiveEdit 切换使用 specta 生成的 WindowPosition 类型"
```

---

## Task 7: autostart 真实 HKCU\Run 读写

**Files:**
- Modify: `src-tauri/Cargo.toml`（`windows` crate +`Win32_System_Registry` feature）
- Modify: `src-tauri/src/autostart/windows.rs`（替换 placeholder）

**目标：** 用 `windows` crate 真实读写 HKCU\Software\Microsoft\Windows\CurrentVersion\Run 键。无新 crate。

- [ ] **Step 7.1: Cargo.toml 加 Win32_System_Registry feature**

编辑 `src-tauri/Cargo.toml`，找到：

```toml
windows = { version = "0.58", features = [
    "Win32_Foundation",
    "Win32_System_Threading",
    "Win32_System_Diagnostics_ToolHelp",
    "Win32_UI_Input_KeyboardAndMouse",
    "Win32_UI_WindowsAndMessaging",
    "Win32_System_DataExchange",
    "Win32_System_Console",
] }
```

在数组末尾加一行（注意逗号）：

```toml
windows = { version = "0.58", features = [
    "Win32_Foundation",
    "Win32_System_Threading",
    "Win32_System_Diagnostics_ToolHelp",
    "Win32_UI_Input_KeyboardAndMouse",
    "Win32_UI_WindowsAndMessaging",
    "Win32_System_DataExchange",
    "Win32_System_Console",
    "Win32_System_Registry",
] }
```

- [ ] **Step 7.2: 写失败测试 · autostart roundtrip**

编辑 `src-tauri/src/autostart/windows.rs`，替换 `#[cfg(test)] mod tests { ... }` 中的 placeholder 测试为（保留原有的 get_autostart_placeholder_returns_false 和 set_autostart_placeholder_echoes_input 删除，替换为真实 roundtrip）：

```rust
#[cfg(all(test, target_os = "windows"))]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_set_get_delete() {
        let pid = std::process::id();
        let test_value_name = format!("CodeIsland-test-{}", pid);

        // 初始：不存在
        let initial = get_autostart_inner(&test_value_name).expect("get before set");
        assert!(!initial, "test value should not exist before set");

        // 写入
        set_autostart_inner(&test_value_name, true).expect("set true");
        let after_set = get_autostart_inner(&test_value_name).expect("get after set");
        assert!(after_set, "test value should exist after set true");

        // 删除
        set_autostart_inner(&test_value_name, false).expect("set false");
        let after_del = get_autostart_inner(&test_value_name).expect("get after del");
        assert!(!after_del, "test value should not exist after set false");

        // 幂等删除
        set_autostart_inner(&test_value_name, false).expect("set false again (idempotent)");
    }
}
```

`#[cfg(all(test, target_os = "windows"))]` 确保非 Windows（本地 Mac 开发）不跑此测试。

- [ ] **Step 7.3: 运行测试验证失败（Windows 上会跑；Mac 上编译失败）**

```bash
cd src-tauri && cargo test roundtrip_set_get_delete 2>&1 | tail -30
```

Expected（Windows）：FAIL，`get_autostart_inner` / `set_autostart_inner` 未定义。
Expected（Mac/Linux）：这条测试被 cfg 屏蔽，其他测试应通过。

- [ ] **Step 7.4: 替换 autostart/windows.rs 主体**

完整重写 `src-tauri/src/autostart/windows.rs`：

```rust
//! autostart — 通过 HKCU\Software\Microsoft\Windows\CurrentVersion\Run 实现开机启动
//!
//! 使用 `windows` crate 的 `Win32_System_Registry` feature（无需新增 crate）。
//!
//! 公开 API 硬编码 VALUE_NAME = "CodeIsland"；内部 inner 版本接受 value_name 参数，
//! 便于单测使用临时键名。

use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Foundation::ERROR_FILE_NOT_FOUND;
use windows::Win32::System::Registry::{
    RegCloseKey, RegCreateKeyExW, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW,
    RegSetValueExW, HKEY, HKEY_CURRENT_USER, KEY_READ, KEY_WRITE, REG_OPTION_NON_VOLATILE,
    REG_SZ,
};

const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const VALUE_NAME: &str = "CodeIsland";

pub fn get_autostart() -> Result<bool, String> {
    get_autostart_inner(VALUE_NAME)
}

pub fn set_autostart(enabled: bool) -> Result<bool, String> {
    set_autostart_inner(VALUE_NAME, enabled)
}

/// 读取 HKCU Run 子键中某 value_name 是否存在。不存在时返回 Ok(false)。
pub(crate) fn get_autostart_inner(value_name: &str) -> Result<bool, String> {
    unsafe {
        let subkey = HSTRING::from(RUN_KEY);
        let mut hkey = HKEY::default();
        let open_status = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR::from_raw(subkey.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        if open_status.is_err() {
            return Ok(false);
        }

        let value_hstr = HSTRING::from(value_name);
        let query_status = RegQueryValueExW(
            hkey,
            PCWSTR::from_raw(value_hstr.as_ptr()),
            None,
            None,
            None,
            None,
        );
        let _ = RegCloseKey(hkey);
        Ok(query_status.is_ok())
    }
}

/// 写入或删除 HKCU Run 子键的某 value_name。enabled=true 写 exe 绝对路径；false 删除。
pub(crate) fn set_autostart_inner(value_name: &str, enabled: bool) -> Result<bool, String> {
    let exe = std::env::current_exe()
        .map_err(|e| format!("[autostart] current_exe: {}", e))?;
    let exe_str = exe
        .to_str()
        .ok_or_else(|| "[autostart] exe 路径非 UTF-8".to_string())?;

    unsafe {
        let subkey = HSTRING::from(RUN_KEY);
        let mut hkey = HKEY::default();
        let create_status = RegCreateKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR::from_raw(subkey.as_ptr()),
            0,
            PCWSTR::null(),
            REG_OPTION_NON_VOLATILE,
            KEY_READ | KEY_WRITE,
            None,
            &mut hkey,
            None,
        );
        if create_status.is_err() {
            return Err(format!("[autostart] RegCreateKeyExW: {:?}", create_status));
        }

        let value_hstr = HSTRING::from(value_name);
        let result = if enabled {
            // REG_SZ 写 UTF-16 字符串 + null terminator
            let data_hstr = HSTRING::from(exe_str);
            let data_u16 = data_hstr.as_wide();
            // UTF-16 字节数 = (len + 1) * 2（含 null terminator）
            let byte_count = (data_u16.len() + 1) * 2;
            let byte_ptr = data_u16.as_ptr() as *const u8;
            // SAFETY: HSTRING::as_wide 返回的切片生命周期由 data_hstr 持有，此 unsafe 块内成立
            let data_bytes: &[u8] = std::slice::from_raw_parts(byte_ptr, byte_count);

            RegSetValueExW(
                hkey,
                PCWSTR::from_raw(value_hstr.as_ptr()),
                0,
                REG_SZ,
                Some(data_bytes),
            )
            .map(|_| true)
            .map_err(|e| format!("[autostart] RegSetValueExW: {:?}", e))
        } else {
            let status = RegDeleteValueW(hkey, PCWSTR::from_raw(value_hstr.as_ptr()));
            if status.is_ok() || status.0 as u32 == ERROR_FILE_NOT_FOUND.0 {
                Ok(false)
            } else {
                Err(format!("[autostart] RegDeleteValueW: {:?}", status))
            }
        };

        let _ = RegCloseKey(hkey);
        result
    }
}

#[cfg(all(test, target_os = "windows"))]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_set_get_delete() {
        let pid = std::process::id();
        let test_value_name = format!("CodeIsland-test-{}", pid);

        // 初始：不存在
        let initial = get_autostart_inner(&test_value_name).expect("get before set");
        assert!(!initial, "test value should not exist before set");

        // 写入
        set_autostart_inner(&test_value_name, true).expect("set true");
        let after_set = get_autostart_inner(&test_value_name).expect("get after set");
        assert!(after_set, "test value should exist after set true");

        // 删除
        set_autostart_inner(&test_value_name, false).expect("set false");
        let after_del = get_autostart_inner(&test_value_name).expect("get after del");
        assert!(!after_del, "test value should not exist after set false");

        // 幂等删除
        set_autostart_inner(&test_value_name, false).expect("set false again (idempotent)");
    }
}
```

- [ ] **Step 7.5: 本地（Mac）编译验证**

```bash
cd src-tauri && cargo build 2>&1 | tail -30
```

Expected：由于 `windows` crate 在 Mac 上是 stub（或编译警告），`autostart/windows.rs` 有 `#[cfg(target_os = "windows")]` 保护的话能跳过。确认 `src-tauri/src/autostart/mod.rs` 现有结构：

```bash
cat src-tauri/src/autostart/mod.rs
```

若 `mod.rs` 目前无条件 `pub use windows::...;`，Mac 上构建会因为 `windows::Win32::System::Registry::*` 在非 Windows 目标上不存在而失败。解决：

编辑 `src-tauri/src/autostart/mod.rs`，把 `mod windows;` + `pub use windows::...` 包裹为 `#[cfg(target_os = "windows")]`；加 stub：

```rust
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{get_autostart, set_autostart};

#[cfg(not(target_os = "windows"))]
pub fn get_autostart() -> Result<bool, String> {
    Ok(false)
}
#[cfg(not(target_os = "windows"))]
pub fn set_autostart(_enabled: bool) -> Result<bool, String> {
    Err("[autostart] 仅 Windows 支持".to_string())
}
```

（注意：若项目已按 Stage 0 清理为 Windows-only，且 Cargo.toml 有 `[target.'cfg(target_os = "windows")'.dependencies]` 隔离，则无需此 stub。执行者按当前 `autostart/mod.rs` 实际情况判断。）

- [ ] **Step 7.6: Windows CI 远程验证**

push 触发 Windows CI：

```bash
git push git@github.com:WangJie0822/CodeIsland-win.git feat/stage0-windows-alignment
gh workflow run build-windows.yml --ref feat/stage0-windows-alignment
```

或直接等下一次 push 触发。CI 会在 Windows 上跑 `roundtrip_set_get_delete`。

Expected: CI `cargo test` 步骤 PASS。

- [ ] **Step 7.7: 提交**

```bash
git add src-tauri/Cargo.toml src-tauri/src/autostart/windows.rs src-tauri/src/autostart/mod.rs
git commit -m "[feat|core|autostart][公共]用 windows crate Win32_System_Registry 实现 HKCU\\Run 真实读写，带 roundtrip 集成测试"
```

---

## Task 8: F5 修复 · island 窗口初始 COLLAPSED 尺寸居中

**Files:**
- Modify: `src-tauri/src/main.rs`（抽 `center_island_window` 纯函数 + 改用 220×32）
- Modify: `src/views/NotchView.vue`（移除 onMount 首次 invokeSetWindowSize）

**目标：** 消除 F5 race（window 创建时 400×48 居中，watcher 改成 220×32 后位置偏移）。Tauri 的 `center: true` 未在 `island` 窗口使用（因为 Stage 1-2 用手写居中算法以保证 y=0）。保留手写但改尺寸为 220。

- [ ] **Step 8.1: 写失败测试 · center_island_window**

在 `src-tauri/src/main.rs`（或新建 `src-tauri/src/window_layout.rs` 存放纯函数，这里假设直接放 main.rs 或 window_control.rs）加单测。推荐放 `src-tauri/src/commands/window_control.rs`（已有 `#[cfg(test)] mod tests`）：

```rust
    #[test]
    fn center_island_window_1920x1080_scale_1() {
        let (x, y) = super::center_island_window(1920.0, 1080.0, 1.0, 220.0, 32.0);
        assert_eq!(x, 850.0);
        assert_eq!(y, 0.0);
    }

    #[test]
    fn center_island_window_2560x1440_scale_1_25() {
        // 逻辑像素宽度 = 2560 / 1.25 = 2048
        let (x, y) = super::center_island_window(2560.0, 1440.0, 1.25, 220.0, 32.0);
        // (2048 - 220) / 2 = 914
        assert_eq!(x, 914.0);
        assert_eq!(y, 0.0);
    }

    #[test]
    fn center_island_window_uses_physical_width_divided_by_scale() {
        let (x, _) = super::center_island_window(3840.0, 2160.0, 2.0, 220.0, 32.0);
        // 3840 / 2 = 1920; (1920 - 220) / 2 = 850
        assert_eq!(x, 850.0);
    }
```

- [ ] **Step 8.2: 运行测试验证失败**

```bash
cd src-tauri && cargo test center_island_window 2>&1 | tail -20
```

Expected: FAIL / 编译错误。

- [ ] **Step 8.3: 添加 center_island_window 纯函数**

在 `src-tauri/src/commands/window_control.rs`（`validate_window_label` 之后），新增：

```rust
/// 计算 island 窗口顶部居中坐标（逻辑像素）。
///
/// `screen_w` / `screen_h` 是物理像素宽高；`scale` 是 monitor.scale_factor；
/// `win_w` / `win_h` 是逻辑尺寸。返回 (x, y) 逻辑坐标，y=0 固定顶部。
pub fn center_island_window(
    screen_w: f64,
    screen_h: f64,
    scale: f64,
    win_w: f64,
    win_h: f64,
) -> (f64, f64) {
    let _ = screen_h;
    let _ = win_h;
    let logical_w = screen_w / scale;
    let x = (logical_w - win_w) / 2.0;
    (x, 0.0)
}
```

- [ ] **Step 8.4: 运行测试验证通过**

```bash
cd src-tauri && cargo test center_island_window 2>&1 | tail -10
```

Expected: PASS (3 tests)

- [ ] **Step 8.5: 替换 main.rs 居中代码使用新函数**

编辑 `src-tauri/src/main.rs`，找到：

```rust
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

替换为：

```rust
if let Some(window) = app.get_webview_window("island") {
    if let Ok(Some(monitor)) = window.current_monitor() {
        let screen_size = monitor.size();
        let scale = monitor.scale_factor();
        let (x, y) = commands::window_control::center_island_window(
            screen_size.width as f64,
            screen_size.height as f64,
            scale,
            220.0,
            32.0,
        );
        let _ = window.set_position(tauri::Position::Logical(
            tauri::LogicalPosition::new(x, y),
        ));
    }
}
```

- [ ] **Step 8.6: 移除 NotchView.vue 首次 invokeSetWindowSize**

读取 `src/views/NotchView.vue` 搜索 `invokeSetWindowSize`：

```bash
grep -n "invokeSetWindowSize" src/views/NotchView.vue
```

找到 `onMounted` 中首次调用 `void invokeSetWindowSize(220, 32)` 的位置（若存在），整行删除。保留 `animateExpanded` / watcher 中的尺寸同步调用。

- [ ] **Step 8.7: 跑全量测试验证**

```bash
cd src-tauri && cargo test 2>&1 | tail -15
cd .. && npm run test
```

Expected: 全部 PASS。

- [ ] **Step 8.8: 提交**

```bash
git add src-tauri/src/main.rs src-tauri/src/commands/window_control.rs src/views/NotchView.vue
git commit -m "[fix|core|main][公共]F5 主刘海窗口初始 COLLAPSED 尺寸居中，抽 center_island_window 纯函数，移除 onMount 首次 invokeSetWindowSize"
```

---

## Task 9: F8 修复 · 订阅协程 Lagged 容错

**Files:**
- Modify: `src-tauri/src/app_state.rs`（添加 `EventBusAction` + `handle_event_bus_recv` 纯函数）
- Modify: `src-tauri/src/main.rs`（改用 loop + match）

**目标：** 现有 `while let Ok(event) = event_rx.recv().await` 遇 `RecvError::Lagged` 退出协程，事件总线从此失联。改为 `loop match` 明确分派 Ok / Lagged / Closed。

- [ ] **Step 9.1: 写失败测试 · handle_event_bus_recv**

编辑 `src-tauri/src/app_state.rs`，在 `#[cfg(test)] mod tests` 末尾加：

```rust
    use tokio::sync::broadcast::error::RecvError;

    #[test]
    fn handle_event_bus_recv_ok_returns_emit() {
        let result = super::handle_event_bus_recv(Ok(AppEvent::SessionsUpdated));
        assert!(matches!(result, super::EventBusAction::Emit("codeisland:sessions:updated")));
    }

    #[test]
    fn handle_event_bus_recv_lagged_returns_warn() {
        let result = super::handle_event_bus_recv(Err(RecvError::Lagged(3)));
        assert!(matches!(result, super::EventBusAction::Warn(3)));
    }

    #[test]
    fn handle_event_bus_recv_closed_returns_break() {
        let result = super::handle_event_bus_recv(Err(RecvError::Closed));
        assert!(matches!(result, super::EventBusAction::Break));
    }
```

- [ ] **Step 9.2: 运行测试验证失败**

```bash
cd src-tauri && cargo test handle_event_bus_recv 2>&1 | tail -20
```

Expected: FAIL / 编译错误。

- [ ] **Step 9.3: 添加 EventBusAction + handle_event_bus_recv**

在 `src-tauri/src/app_state.rs` 现有 `impl AppEvent` 块之后加：

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum EventBusAction {
    Emit(&'static str),
    Warn(u64),
    Break,
}

/// 把 broadcast recv 的结果分派为具体动作。
///
/// 纯函数方便测试；主循环在 main.rs 中按返回值执行副作用。
pub fn handle_event_bus_recv(
    result: Result<AppEvent, tokio::sync::broadcast::error::RecvError>,
) -> EventBusAction {
    use tokio::sync::broadcast::error::RecvError;
    match result {
        Ok(event) => EventBusAction::Emit(event.topic()),
        Err(RecvError::Lagged(n)) => EventBusAction::Warn(n),
        Err(RecvError::Closed) => EventBusAction::Break,
    }
}
```

- [ ] **Step 9.4: 运行测试验证通过**

```bash
cd src-tauri && cargo test handle_event_bus_recv 2>&1 | tail -10
```

Expected: PASS (3 tests)

- [ ] **Step 9.5: 改写 main.rs 订阅协程**

编辑 `src-tauri/src/main.rs`，找到现有：

```rust
let app_handle = app.handle().clone();
let mut event_rx = event_tx.subscribe();
tauri::async_runtime::spawn(async move {
    while let Ok(event) = event_rx.recv().await {
        let _ = app_handle.emit(event.topic(), ());
    }
});
```

替换为：

```rust
let app_handle = app.handle().clone();
let mut event_rx = event_tx.subscribe();
tauri::async_runtime::spawn(async move {
    loop {
        let action = app_state::handle_event_bus_recv(event_rx.recv().await);
        match action {
            app_state::EventBusAction::Emit(topic) => {
                let _ = app_handle.emit(topic, ());
            }
            app_state::EventBusAction::Warn(n) => {
                log::warn!("[event-bus] 订阅滞后，丢失 {} 条事件", n);
            }
            app_state::EventBusAction::Break => break,
        }
    }
});
```

- [ ] **Step 9.6: 跑全量 cargo test 验证**

```bash
cd src-tauri && cargo test 2>&1 | tail -15
```

Expected: 全部 PASS。

- [ ] **Step 9.7: 提交**

```bash
git add src-tauri/src/app_state.rs src-tauri/src/main.rs
git commit -m "[fix|core|event-bus][公共]F8 订阅协程 loop match Lagged 容错，抽 handle_event_bus_recv 纯函数避免协程退出"
```

---

## Task 10: F13 修复 · Windows CI 加前端测试 + CLAUDE.md 同步

**Files:**
- Modify: `.github/workflows/build-windows.yml`
- Modify: `CLAUDE.md`

**目标：** CI 在 `cargo test` 之后、`npx tauri build` 之前插入 `npm run test` + `npm run build` 步骤，让前端单测和 type check 进入门禁。CLAUDE.md 的 CI 段落同步。

**验证方式：** push 后看 CI 结果。

- [ ] **Step 10.1: 读当前 workflow**

```bash
cat .github/workflows/build-windows.yml
```

定位 `cargo test` 步骤（约中段）和 `npx tauri build` 步骤（末尾）。

- [ ] **Step 10.2: 在 workflow 中插入 npm 步骤**

在 `- name: Run Rust tests` / `working-directory: src-tauri` / `run: cargo test` 之后（以及 `upload-artifact: generated-ts` 之后），在 `npx tauri build` 之前，插入：

```yaml
      - name: Run frontend tests
        run: npm run test

      - name: Type check + vite build (frontend)
        run: npm run build
```

（注意 YAML 缩进与现有步骤对齐。）

- [ ] **Step 10.3: 合并当前未提交的 CLAUDE.md 修改（specta 注意事项 + 之前的 CI 描述修正）**

CLAUDE.md 当前有未提交改动（见 `git diff CLAUDE.md`），内容是 Stage 2 追加的 specta 四类约束说明 + CI 描述更新。把 CI 段落进一步改为反映 Task 10 的新步骤。

编辑 `CLAUDE.md` 的 `## CI` 段落，找到：

```markdown
GitHub Actions `build-windows.yml` 仅在 Windows 上运行：`npm ci` → `npx tauri icon` → `cargo test`（含 specta 生成 generated.ts） → 上传 `generated-ts` artifact → `npx tauri build`（内部触发 `vue-tsc --noEmit && vite build` 和 Rust release build），上传 `CodeIsland-win-exe` / `msi` / `nsis` 产物。
```

替换为：

```markdown
GitHub Actions `build-windows.yml` 仅在 Windows 上运行：`npm ci` → `npx tauri icon` → `cargo test`（含 specta 生成 `src/types/generated.ts`） → 上传 `generated-ts` artifact → `npm run test`（Vitest） → `npm run build`（`vue-tsc --noEmit` + `vite build`） → `npx tauri build`（Rust release build + 打包产物），上传 `CodeIsland-win-exe` / `msi` / `nsis` 产物。
```

- [ ] **Step 10.4: 本地验证 CLAUDE.md 无其他未同步内容**

```bash
git diff CLAUDE.md
```

Expected: diff 只包含 Task 10 修改 + Stage 2 累积的 specta 注意事项（都应该保留）。

- [ ] **Step 10.5: 本地跑一遍 npm run test + npm run build 确认 CI 步骤能绿**

```bash
npm run test && npm run build
```

Expected: 两步都成功。

- [ ] **Step 10.6: 提交**

```bash
git add .github/workflows/build-windows.yml CLAUDE.md
git commit -m "[ci|build|workflow][公共]F13 Windows CI 增加 npm run test + npm run build 步骤，CLAUDE.md 同步 CI 描述与 specta 注意事项"
```

---

## Task 11: Stage 3 验收

**Files:** 无代码改动（验收和文档）

- [ ] **Step 11.1: push 到远程触发 CI**

```bash
git push git@github.com:WangJie0822/CodeIsland-win.git feat/stage0-windows-alignment
```

- [ ] **Step 11.2: 触发 + 监控 CI**

```bash
gh workflow run build-windows.yml --ref feat/stage0-windows-alignment
sleep 10
gh run list --workflow=build-windows.yml --limit=1
```

等 CI 跑完后：

```bash
gh run list --workflow=build-windows.yml --limit=1
```

Expected: 最新一条 `completed success`（不要信 `gh run watch --exit-status` 的 exit code，参考 Vault 笔记 §4.1）。

- [ ] **Step 11.3: 本地（Mac）拉回最新 generated.ts artifact 避免漂移**

```bash
RUN_ID=$(gh run list --workflow=build-windows.yml --limit=1 --json databaseId --jq '.[0].databaseId')
gh run download "$RUN_ID" -n generated-ts -D /tmp/gts
diff /tmp/gts/generated.ts src/types/generated.ts && echo "已同步" || (cp /tmp/gts/generated.ts src/types/generated.ts && git status src/types/generated.ts)
```

Expected: 若本地已是 CI 同步版，提示"已同步"；否则按提示 commit `src/types/generated.ts`（若有变化，用 `[build|UI|generated][公共]同步 CI 产物 generated.ts` 提交）。

- [ ] **Step 11.4: 在 Windows 机器上人工 smoke（规格见 spec §10.4）**

下载 CI 产物：

```bash
gh run download "$RUN_ID" -n CodeIsland-win-exe -D /tmp/ci-exe
```

在 Windows 机器上执行该 exe，依次验证：

1. 主刘海显示在屏顶居中 220×32 ✓
2. tray 右键 → 菜单 8 条（主刘海 + 分隔 + 5 View + 分隔 + 退出）
3. tray 点「偏好设置」→ Settings 窗口居中打开
4. Settings 关闭（标题栏 X 或 Alt+F4）→ 窗口隐藏而非销毁；tray 再点 → 同一窗口 show + focus
5. 依次验 Buddy / Usage / Presets / Live Edit
6. 主刘海 hover 展开 → 右上「⋯」按钮 → 点击 popover → 5 条菜单 → 点一条打开窗口
7. Settings 切换「开机启动」Toggle → 重启 Windows → Code Island 自动启动（验证 HKCU\Run\CodeIsland 存在且值为 exe 路径）
8. 退出后重新打开 Settings → Toggle 状态与注册表一致

- [ ] **Step 11.5: 更新 Vault follow-ups 笔记标记 F5 / F8 / F13 / F14 为 ✅**

编辑 `~/.claude/Vault/项目笔记/CodeIsland-win Stage 0 follow-ups.md`，在 F5 / F8 / F13 / F14 各条末尾加：

```
- **状态**：✅ 已完成于 Stage 3 commit <sha>
```

（执行者填入对应 commit sha。）

- [ ] **Step 11.6: 在 Vault 项目笔记新增 Stage 3 收官章节**

在 `~/.claude/Vault/项目笔记/CodeIsland-win Windows 版从零构建.md` 末尾追加：

```markdown
## Stage 3 多窗口接入 + autostart 真实化（2026-04-18）

目标：Stage 1 产出的 5 个 View 接入可达入口（tray + 刘海「⋯」），autostart 真实落注册表，修 F5/F8/F13/F14。

### Task 清单（10 + 1 验收）

| # | 主题 | Commit |
|---|------|--------|
| 1 | tauri.conf.json 声明 6 窗口 | <sha> |
| 2 | vue-router + label 分发 | <sha> |
| 3 | open_view_window 命令 + close_requested 隐藏 | <sha> |
| 4 | tray 菜单扩展 5 View 入口 | <sha> |
| 5 | 刘海「⋯」按钮 + NotchMenuPopover | <sha> |
| 6 | AppEvent::NotchPositionChanged + get_window_position | <sha> |
| 7 | autostart HKCU Run 真实读写 | <sha> |
| 8 | F5 window 初始 COLLAPSED 尺寸 | <sha> |
| 9 | F8 订阅协程 loop match Lagged | <sha> |
| 10 | F13 CI npm run test + CLAUDE.md | <sha> |

Windows CI run <run-id> 全绿。人工 smoke <date> 通过。
```

（执行者填入 sha / run-id / date。）

- [ ] **Step 11.7: Stage 3 收官 commit（可选，按需）**

若有零散未提交的产物（generated.ts 同步、Vault 笔记）：

```bash
git add -A
git status --short  # 确认无 secrets / 无关文件
git commit -m "[docs|all|stage3][公共]Stage 3 收官：follow-ups 标记 + Vault 笔记更新"
git push git@github.com:WangJie0822/CodeIsland-win.git feat/stage0-windows-alignment
```

---

## Self-Review（plan 作者自检）

完成 plan 起草后：

**1. Spec coverage 检查**

| Spec 章节 | 对应 Task |
|---|---|
| §1 目标与范围 | Task 1-10 覆盖 |
| §2 窗口拓扑 | Task 1 |
| §3 前端路由分发 | Task 2 |
| §4 入口机制（tray + 刘海按钮） | Task 3, 4, 5 |
| §5 AppEvent::NotchPositionChanged + get_window_position | Task 6 |
| §6 autostart 真实实现 | Task 7 |
| §7.1 F5 | Task 8 |
| §7.2 F8 | Task 9 |
| §7.3 F13 | Task 10 |
| §8 数据流 | （验收场景通过 Task 5-6 涵盖） |
| §9 错误处理（前缀） | Task 3 / 6 / 7 统一前缀 |
| §10 测试 | Task 2-9 内嵌 TDD |
| §11 提交计划 | 对应 Task 1-10 commit |

**2. Placeholder 检查**：plan 内无 TBD / TODO / "similar to"。

**3. Type 一致性**：
- `KNOWN_WINDOW_LABELS`（Task 3）与 `KNOWN_LABELS` Set（Task 2 `src/main.ts`）同步 6 个 label
- `WindowPosition` 字段 `x: i32, y: i32` 全文一致（Task 6 / NotchLiveEditView）
- `EventBusAction` 变体名 `Emit / Warn / Break` 一致（Task 9）
- `handle_event_bus_recv` 函数名在 app_state.rs 定义 + main.rs 调用一致
- `validate_window_label` / `center_island_window` 纯函数命名全文一致
- autostart `get_autostart_inner` / `set_autostart_inner` 拆分一致
- commit message 前缀 `[类型|模块|功能][公共]` 全文一致

---

## Execution 入口

Plan 完成后，写入路径：`docs/superpowers/plans/2026-04-18-stage3-windows-ui-alignment.md`

两种执行方式：

**1. Subagent-Driven（推荐）** — 主会话为每个 Task dispatch 一个 fresh subagent + 两阶段 review，严格复用 Stage 1 的 agent-driven 模式。
- Required sub-skill: `superpowers:subagent-driven-development`

**2. Inline Execution** — 在当前会话串行执行全部 Task，每个 batch 设置 checkpoint 供 review。
- Required sub-skill: `superpowers:executing-plans`

本 plan 工作量约 475 行代码 + 300 行测试，10 + 1 Task，预计执行 2-3 小时。
