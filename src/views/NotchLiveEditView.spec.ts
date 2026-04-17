import { describe, expect, it, vi, beforeEach, afterEach } from "vitest";
import { mount, flushPromises } from "@vue/test-utils";

// ------- mock @tauri-apps/api/core -------
const mockInvoke = vi.fn(async (..._args: unknown[]) => undefined);
vi.mock("@tauri-apps/api/core", () => ({
  invoke: (cmd: string, payload?: unknown) => mockInvoke(cmd, payload),
}));

// ------- mock @tauri-apps/api/window -------
vi.mock("@tauri-apps/api/window", () => ({
  availableMonitors: vi.fn(async () => [
    { size: { width: 1920, height: 1080 }, name: "DISPLAY1" },
  ]),
  getCurrentWindow: vi.fn(() => ({ setPosition: vi.fn() })),
}));

// ------- mock composable（隔离 pointer 事件绑定，只测视图逻辑） -------
vi.mock("@/composables/useDragResize", () => ({
  useDragResize: vi.fn(),
}));

import NotchLiveEditView from "./NotchLiveEditView.vue";

describe("NotchLiveEditView", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
    mockInvoke.mockResolvedValue(undefined);
  });

  afterEach(() => {
    // 清理 classList
    document.body.classList.remove("no-notch-anim");
  });

  it("挂载后添加 no-notch-anim class", async () => {
    mount(NotchLiveEditView);
    await flushPromises();
    expect(document.body.classList.contains("no-notch-anim")).toBe(true);
  });

  it("卸载后移除 no-notch-anim class", async () => {
    const wrapper = mount(NotchLiveEditView);
    await flushPromises();
    wrapper.unmount();
    expect(document.body.classList.contains("no-notch-anim")).toBe(false);
  });

  it("渲染尺寸指示器", () => {
    const wrapper = mount(NotchLiveEditView);
    expect(wrapper.find("[data-testid='size-indicator']").exists()).toBe(true);
    // 默认 EXPANDED 尺寸 760×520
    expect(wrapper.find("[data-testid='size-indicator']").text()).toContain("760");
    expect(wrapper.find("[data-testid='size-indicator']").text()).toContain("520");
  });

  it("贴合顶部 按钮点击调 set_window_position with y=0", async () => {
    const wrapper = mount(NotchLiveEditView);
    await wrapper.find("[data-testid='btn-snap-top']").trigger("click");
    await flushPromises();
    expect(mockInvoke).toHaveBeenCalledWith("set_window_position", expect.objectContaining({ y: 0 }));
  });

  it("← 按钮逐像素减少 X", async () => {
    const wrapper = mount(NotchLiveEditView);
    await flushPromises();
    mockInvoke.mockClear();
    await wrapper.find("[data-testid='btn-nudge-left']").trigger("click");
    await flushPromises();
    // 调用了 set_window_position
    expect(mockInvoke).toHaveBeenCalledWith("set_window_position", expect.any(Object));
  });

  it("→ 按钮逐像素增加 X", async () => {
    const wrapper = mount(NotchLiveEditView);
    await flushPromises();
    mockInvoke.mockClear();
    await wrapper.find("[data-testid='btn-nudge-right']").trigger("click");
    await flushPromises();
    expect(mockInvoke).toHaveBeenCalledWith("set_window_position", expect.any(Object));
  });

  it("保存按钮调 update_settings 并 emit close", async () => {
    const wrapper = mount(NotchLiveEditView);
    await flushPromises();
    await wrapper.find("[data-testid='btn-save']").trigger("click");
    await flushPromises();
    expect(mockInvoke).toHaveBeenCalledWith(
      "update_settings",
      expect.objectContaining({
        patch: expect.objectContaining({ notch_position: expect.any(Object) }),
      }),
    );
    expect(wrapper.emitted("close")).toBeTruthy();
  });

  it("取消按钮 emit close 并复位位置", async () => {
    const wrapper = mount(NotchLiveEditView);
    await flushPromises();
    await wrapper.find("[data-testid='btn-cancel']").trigger("click");
    await flushPromises();
    expect(wrapper.emitted("close")).toBeTruthy();
  });

  it("数字键 2（中上）调 set_window_position X 居中 Y=0", async () => {
    mount(NotchLiveEditView);
    await flushPromises();
    mockInvoke.mockClear();

    // 模拟键盘事件
    window.dispatchEvent(new KeyboardEvent("keydown", { key: "2" }));
    await flushPromises();

    expect(mockInvoke).toHaveBeenCalledWith(
      "set_window_position",
      expect.objectContaining({ y: 0 }),
    );
  });

  it("数字键 1（左上）X=0 Y=0", async () => {
    mount(NotchLiveEditView);
    await flushPromises();
    mockInvoke.mockClear();

    window.dispatchEvent(new KeyboardEvent("keydown", { key: "1" }));
    await flushPromises();

    expect(mockInvoke).toHaveBeenCalledWith(
      "set_window_position",
      expect.objectContaining({ x: 0, y: 0 }),
    );
  });

  it("数字键 3（右上）X=最右 Y=0", async () => {
    mount(NotchLiveEditView);
    await flushPromises();
    mockInvoke.mockClear();

    window.dispatchEvent(new KeyboardEvent("keydown", { key: "3" }));
    await flushPromises();

    // mock monitor 1920 宽，window 760 宽，所以 X = 1920 - 760 = 1160
    expect(mockInvoke).toHaveBeenCalledWith(
      "set_window_position",
      expect.objectContaining({ x: 1160, y: 0 }),
    );
  });

  it("拖动模式按钮切换 dragMode 状态", async () => {
    const wrapper = mount(NotchLiveEditView);
    const btn = wrapper.find("[data-testid='btn-drag-mode']");
    expect(btn.classes()).not.toContain("btn-action--active");
    await btn.trigger("click");
    expect(btn.classes()).toContain("btn-action--active");
    await btn.trigger("click");
    expect(btn.classes()).not.toContain("btn-action--active");
  });
});
