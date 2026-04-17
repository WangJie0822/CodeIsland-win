import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

// @stage2 替换：从 '@/types/generated' 导入 Settings 类型（当 specta 导出后）
export interface NotchPosition {
  x: number;
  y: number;
  screen_id: number | null;
}

export interface Settings {
  target_screen: number | null;
  notification_sound: string;
  language: string;
  pixel_cat_enabled: boolean;
  group_by_project: boolean;
  smart_suppression: boolean;
  auto_collapse_on_leave: boolean;
  auto_collapse_ms: number;
  launch_at_login: boolean;
  hooks_enabled: boolean;
  notch_position: NotchPosition | null;
}

export const DEFAULT_SETTINGS: Settings = {
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

export const useSettingsStore = defineStore("settings", () => {
  const value = ref<Settings | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function load(): Promise<void> {
    loading.value = true;
    error.value = null;
    try {
      value.value = await invoke<Settings>("get_settings");
    } catch (e) {
      error.value = String(e);
      console.error("[settings] load 失败:", e);
    } finally {
      loading.value = false;
    }
  }

  async function update(patch: Partial<Settings>): Promise<void> {
    loading.value = true;
    error.value = null;
    try {
      value.value = await invoke<Settings>("update_settings", { patch });
    } catch (e) {
      error.value = String(e);
      console.error("[settings] update 失败:", e);
    } finally {
      loading.value = false;
    }
  }

  return { value, loading, error, load, update };
});
