import { defineStore } from "pinia";
import { ref } from "vue";

// Stage 2 待办：UsageReport/UsageBucket 由 specta 自动生成后，
// 替换为 import type { UsageReport } from "@/types/generated"
// 并在 commands/mod.rs 注册 get_usage_report / refresh_usage_report

/** UsageBucket 本地类型替身（待 Stage 2 从 generated.ts 导入） */
export interface UsageBucket {
  tokens: number;
  quota: number;
  percent: number;
  resets_at: number | null;
}

/** UsageReport 本地类型替身（待 Stage 2 从 generated.ts 导入） */
export interface UsageReport {
  window_5h: UsageBucket;
  window_7d: UsageBucket;
  last_refresh_ts: number;
}

export const useUsageStore = defineStore("usage", () => {
  const report = ref<UsageReport | null>(null);

  async function load(): Promise<void> {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      report.value = await invoke<UsageReport>("get_usage_report");
    } catch (e) {
      console.error("[usage] load 失败:", e);
    }
  }

  async function refresh(): Promise<void> {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      report.value = await invoke<UsageReport>("refresh_usage_report");
    } catch (e) {
      console.error("[usage] refresh 失败:", e);
    }
  }

  return { report, load, refresh };
});
