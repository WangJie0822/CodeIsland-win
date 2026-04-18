import { describe, it, expect, afterEach } from "vitest";
import { mount, flushPromises } from "@vue/test-utils";
import NotchMenuPopover from "./NotchMenuPopover.vue";

describe("NotchMenuPopover", () => {
  afterEach(() => {
    // 清理 DOM，防止 Teleport 内容跨测试污染
    document.body.innerHTML = "";
  });

  it("渲染 5 个菜单项：settings / buddy / usage / presets / notch-live-edit", () => {
    mount(NotchMenuPopover, { attachTo: document.body });
    // Teleport 将内容渲染到 document.body，需直接查询
    const items = document.body.querySelectorAll("[data-popover-item]");
    expect(items.length).toBe(5);
    const labels = Array.from(items).map((i) => i.getAttribute("data-popover-item"));
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
    // Teleport 渲染到 body，从 document 查询按钮
    const btn = document.body.querySelector("[data-popover-item='settings']") as HTMLElement;
    btn.click();
    await flushPromises();
    expect(w.emitted("open-view")).toBeTruthy();
    expect(w.emitted("open-view")?.[0]).toEqual(["settings"]);
  });

  it("点击外部 emit 'close' 事件", async () => {
    const w = mount(NotchMenuPopover, { attachTo: document.body });
    await flushPromises();
    await new Promise((r) => setTimeout(r, 10));
    document.body.click();
    await flushPromises();
    expect(w.emitted("close")).toBeTruthy();
  });

  it("菜单项显示中文标签", () => {
    mount(NotchMenuPopover, { attachTo: document.body });
    // Teleport 渲染到 body
    const texts = Array.from(
      document.body.querySelectorAll("[data-popover-item]")
    ).map((i) => i.textContent?.trim() ?? "");
    expect(texts).toEqual(["偏好设置", "伙伴", "用量报告", "启动预设", "调整刘海位置"]);
  });
});
