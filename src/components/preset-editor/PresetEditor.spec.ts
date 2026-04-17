import { describe, it, expect, vi, beforeEach } from "vitest";
import { mount, flushPromises } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import PresetEditor from "./PresetEditor.vue";
import type { LaunchPreset } from "@/stores/presets";

// PresetEditor 内部不直接 invoke，由父组件通过 @save 事件处理，所以不需要 mock tauri
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

function makePreset(partial: Partial<LaunchPreset> = {}): LaunchPreset {
  return {
    id: "p1",
    name: "Test Preset",
    icon: "🚀",
    cwd: "C:/projects/test",
    initial_prompt: "hello",
    model: "claude-sonnet-4-5",
    agent: null,
    mcp_servers: ["server-a"],
    sort_order: 0,
    ...partial,
  };
}

describe("PresetEditor", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
  });

  it("渲染模态覆盖层", () => {
    const wrapper = mount(PresetEditor, { props: { preset: null } });
    expect(wrapper.find("[data-testid='preset-editor-overlay']").exists()).toBe(true);
    expect(wrapper.find("[data-testid='preset-editor']").exists()).toBe(true);
  });

  it("新建模式时显示「新建预设」标题", () => {
    const wrapper = mount(PresetEditor, { props: { preset: null } });
    expect(wrapper.text()).toContain("新建预设");
  });

  it("编辑模式时显示「编辑预设」标题", () => {
    const wrapper = mount(PresetEditor, { props: { preset: makePreset() } });
    expect(wrapper.text()).toContain("编辑预设");
  });

  it("编辑模式时表单预填入 preset 数据", () => {
    const preset = makePreset({ name: "My App", cwd: "C:/work/myapp" });
    const wrapper = mount(PresetEditor, { props: { preset } });

    const nameInput = wrapper.find("[data-testid='input-name']");
    const cwdInput = wrapper.find("[data-testid='input-cwd']");
    expect((nameInput.element as HTMLInputElement).value).toBe("My App");
    expect((cwdInput.element as HTMLInputElement).value).toBe("C:/work/myapp");
  });

  it("名称为空时保存按钮禁用", async () => {
    const wrapper = mount(PresetEditor, { props: { preset: null } });
    const saveBtn = wrapper.find("[data-testid='btn-save']");
    // 未填 name 和 cwd 时 disabled
    expect((saveBtn.element as HTMLButtonElement).disabled).toBe(true);
  });

  it("名称和 cwd 都有值时保存按钮可用", async () => {
    const wrapper = mount(PresetEditor, { props: { preset: null } });

    await wrapper.find("[data-testid='input-name']").setValue("My Project");
    await wrapper.find("[data-testid='input-cwd']").setValue("C:/projects/mine");

    const saveBtn = wrapper.find("[data-testid='btn-save']");
    expect((saveBtn.element as HTMLButtonElement).disabled).toBe(false);
  });

  it("点击保存按钮 emit save 事件并携带 preset 数据", async () => {
    const wrapper = mount(PresetEditor, { props: { preset: null } });

    await wrapper.find("[data-testid='input-name']").setValue("New Project");
    await wrapper.find("[data-testid='input-cwd']").setValue("C:/projects/new");
    await wrapper.find("[data-testid='input-initial-prompt']").setValue("帮我写测试");
    await wrapper.find("[data-testid='input-mcp-servers']").setValue("server-a, server-b");

    await wrapper.find("[data-testid='btn-save']").trigger("click");

    const emitted = wrapper.emitted("save");
    expect(emitted).toBeTruthy();
    expect(emitted![0]).toHaveLength(1);
    const saved = emitted![0][0] as LaunchPreset;
    expect(saved.name).toBe("New Project");
    expect(saved.cwd).toBe("C:/projects/new");
    expect(saved.initial_prompt).toBe("帮我写测试");
    expect(saved.mcp_servers).toEqual(["server-a", "server-b"]);
  });

  it("点击取消按钮 emit cancel 事件", async () => {
    const wrapper = mount(PresetEditor, { props: { preset: null } });
    await wrapper.find("[data-testid='btn-cancel']").trigger("click");
    expect(wrapper.emitted("cancel")).toBeTruthy();
  });

  it("model 选择 inherit 时 emit 的 preset.model 为 null", async () => {
    const wrapper = mount(PresetEditor, { props: { preset: null } });
    await wrapper.find("[data-testid='input-name']").setValue("P");
    await wrapper.find("[data-testid='input-cwd']").setValue("C:/p");
    // inherit 是默认值，不改动
    await wrapper.find("[data-testid='btn-save']").trigger("click");

    const saved = wrapper.emitted("save")![0][0] as LaunchPreset;
    // "inherit" → null
    expect(saved.model).toBeNull();
  });

  it("mcp_servers 逗号分隔字符串正确解析", async () => {
    const wrapper = mount(PresetEditor, { props: { preset: null } });
    await wrapper.find("[data-testid='input-name']").setValue("P");
    await wrapper.find("[data-testid='input-cwd']").setValue("C:/p");
    await wrapper.find("[data-testid='input-mcp-servers']").setValue("  server-a , server-b  , ");

    await wrapper.find("[data-testid='btn-save']").trigger("click");
    const saved = wrapper.emitted("save")![0][0] as LaunchPreset;
    expect(saved.mcp_servers).toEqual(["server-a", "server-b"]);
  });

  it("编辑现有 preset 时保存按钮触发 save 含原 id", async () => {
    const preset = makePreset({ id: "existing-id", name: "Old Name" });
    const wrapper = mount(PresetEditor, { props: { preset } });

    await wrapper.find("[data-testid='input-name']").setValue("New Name");
    await wrapper.find("[data-testid='btn-save']").trigger("click");

    const saved = wrapper.emitted("save")![0][0] as LaunchPreset;
    expect(saved.id).toBe("existing-id");
    expect(saved.name).toBe("New Name");
  });
});
