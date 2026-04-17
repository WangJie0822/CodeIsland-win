import { describe, expect, it, vi, beforeEach } from "vitest";
import { createPinia, setActivePinia } from "pinia";

const mockSettings = {
  target_screen: null,
  notification_sound: "default",
  language: "en",
  pixel_cat_enabled: false,
  group_by_project: false,
  smart_suppression: false,
  auto_collapse_on_leave: true,
  auto_collapse_ms: 3000,
  launch_at_login: false,
  hooks_enabled: true,
  notch_position: null,
};

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(async (cmd: string, _args?: unknown) => {
    if (cmd === "get_settings") return { ...mockSettings };
    if (cmd === "update_settings") return { ...mockSettings, hooks_enabled: false };
    throw new Error(`未知命令: ${cmd}`);
  }),
}));

import { useSettingsStore } from "./settings";

describe("useSettingsStore", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("初始 value 为 null", () => {
    const store = useSettingsStore();
    expect(store.value).toBeNull();
  });

  it("load() 从 invoke 加载设置", async () => {
    const store = useSettingsStore();
    await store.load();
    expect(store.value).not.toBeNull();
    expect(store.value?.notification_sound).toBe("default");
    expect(store.value?.hooks_enabled).toBe(true);
  });

  it("update() 调用 update_settings 并更新 value", async () => {
    const store = useSettingsStore();
    await store.load();
    await store.update({ hooks_enabled: false });
    expect(store.value?.hooks_enabled).toBe(false);
  });

  it("load 失败时 error 有值", async () => {
    const { invoke } = await import("@tauri-apps/api/core");
    vi.mocked(invoke).mockRejectedValueOnce(new Error("[settings] 读取失败"));
    const store = useSettingsStore();
    await store.load();
    expect(store.error).not.toBeNull();
    expect(store.value).toBeNull();
  });
});
