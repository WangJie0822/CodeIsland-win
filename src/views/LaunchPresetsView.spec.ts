import { describe, it, expect, vi, beforeEach } from "vitest";
import { mount, flushPromises } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";

// mock tauri invoke（前端单测不跑真实 IPC）
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

import { invoke } from "@tauri-apps/api/core";
import LaunchPresetsView from "./LaunchPresetsView.vue";
import type { LaunchPreset } from "@/stores/presets";

function makePreset(partial: Partial<LaunchPreset> = {}): LaunchPreset {
  return {
    id: "p1",
    name: "Test Preset",
    icon: "🚀",
    cwd: "C:/projects/test",
    initial_prompt: null,
    model: "claude-sonnet-4-5",
    agent: null,
    mcp_servers: [],
    sort_order: 0,
    ...partial,
  };
}

const mockInvoke = invoke as ReturnType<typeof vi.fn>;

describe("LaunchPresetsView", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
  });

  it("渲染视图容器", () => {
    mockInvoke.mockResolvedValue([]);
    const wrapper = mount(LaunchPresetsView);
    expect(wrapper.find("[data-testid='presets-view']").exists()).toBe(true);
  });

  it("空列表时显示空状态提示", async () => {
    mockInvoke.mockResolvedValue([]);
    const wrapper = mount(LaunchPresetsView);
    await flushPromises();
    expect(wrapper.find("[data-testid='empty-state']").exists()).toBe(true);
  });

  it("渲染 preset 列表卡片", async () => {
    const presets = [makePreset({ id: "p1", name: "项目 A" }), makePreset({ id: "p2", name: "项目 B" })];
    mockInvoke.mockResolvedValue(presets);
    const wrapper = mount(LaunchPresetsView);
    await flushPromises();

    expect(wrapper.find("[data-testid='preset-card-p1']").exists()).toBe(true);
    expect(wrapper.find("[data-testid='preset-card-p2']").exists()).toBe(true);
    expect(wrapper.text()).toContain("项目 A");
    expect(wrapper.text()).toContain("项目 B");
  });

  it("点击「新建预设」按钮打开 PresetEditor 模态", async () => {
    mockInvoke.mockResolvedValue([]);
    const wrapper = mount(LaunchPresetsView);
    await flushPromises();

    await wrapper.find("[data-testid='btn-new-preset']").trigger("click");
    expect(wrapper.find("[data-testid='preset-editor-overlay']").exists()).toBe(true);
  });

  it("点击「启动」按钮调用 launch_preset", async () => {
    const presets = [makePreset({ id: "p1" })];
    // list_presets 返回列表，launch_preset 返回 undefined
    mockInvoke.mockImplementation((cmd: string) => {
      if (cmd === "list_presets") return Promise.resolve(presets);
      if (cmd === "launch_preset") return Promise.resolve(undefined);
      return Promise.resolve(undefined);
    });

    const wrapper = mount(LaunchPresetsView);
    await flushPromises();

    await wrapper.find("[data-testid='btn-launch-p1']").trigger("click");
    await flushPromises();

    // 验证 invoke 被调用过 launch_preset
    expect(mockInvoke).toHaveBeenCalledWith("launch_preset", { id: "p1" });
  });

  it("点击「删除」按钮调用 delete_preset 并刷新列表", async () => {
    const presets = [makePreset({ id: "p1" })];
    mockInvoke.mockImplementation((cmd: string) => {
      if (cmd === "list_presets") return Promise.resolve(presets);
      if (cmd === "delete_preset") return Promise.resolve(undefined);
      return Promise.resolve(undefined);
    });

    const wrapper = mount(LaunchPresetsView);
    await flushPromises();

    await wrapper.find("[data-testid='btn-delete-p1']").trigger("click");
    await flushPromises();

    expect(mockInvoke).toHaveBeenCalledWith("delete_preset", { id: "p1" });
  });

  it("点击「编辑」按钮打开 PresetEditor 并传入 preset", async () => {
    const preset = makePreset({ id: "p1", name: "My Preset" });
    mockInvoke.mockResolvedValue([preset]);

    const wrapper = mount(LaunchPresetsView);
    await flushPromises();

    await wrapper.find("[data-testid='btn-edit-p1']").trigger("click");
    // PresetEditor 模态应打开
    expect(wrapper.find("[data-testid='preset-editor-overlay']").exists()).toBe(true);
  });

  it("preset 卡片显示 cwd", async () => {
    const preset = makePreset({ id: "p1", cwd: "C:\\projects\\myapp" });
    mockInvoke.mockResolvedValue([preset]);

    const wrapper = mount(LaunchPresetsView);
    await flushPromises();

    expect(wrapper.find("[data-testid='preset-card-p1']").text()).toContain("C:\\projects\\myapp");
  });
});
