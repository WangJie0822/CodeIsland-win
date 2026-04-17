/**
 * presets store：LaunchPreset CRUD + 启动。
 *
 * NOTE（Stage 2 待办）：
 * - `LaunchPreset` 类型目前由本文件内 local interface 定义，
 *   待 specta 重新生成 generated.ts 后统一切换到 `@/types/generated`。
 * - `onPresetsUpdated` 事件监听在 Stage 2 注册后接入。
 */
import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

/**
 * 启动预设类型定义（Stage 2 将改为从 @/types/generated 导入）。
 */
export interface LaunchPreset {
  id: string;
  name: string;
  icon: string;
  cwd: string;
  initial_prompt: string | null;
  model: string | null;
  agent: string | null;
  mcp_servers: string[];
  sort_order: number;
}

// —— invoke 封装 ——

async function invokeListPresets(): Promise<LaunchPreset[]> {
  return invoke("list_presets");
}

async function invokeSavePreset(preset: LaunchPreset): Promise<LaunchPreset> {
  return invoke("save_preset", { preset });
}

async function invokeDeletePreset(id: string): Promise<void> {
  return invoke("delete_preset", { id });
}

async function invokeLaunchPreset(id: string): Promise<void> {
  return invoke("launch_preset", { id });
}

// —— store ——

export const usePresetsStore = defineStore("presets", () => {
  const list = ref<LaunchPreset[]>([]);

  /** 从后端加载 preset 列表（已按 sort_order 排序）。 */
  async function load(): Promise<void> {
    try {
      list.value = await invokeListPresets();
    } catch (e) {
      console.error("[presets] load 失败:", e);
    }
  }

  /** 保存（新增或更新）一个 preset，成功后刷新列表。 */
  async function save(preset: LaunchPreset): Promise<LaunchPreset> {
    const saved = await invokeSavePreset(preset);
    await load();
    return saved;
  }

  /** 删除指定 id 的 preset，成功后刷新列表。 */
  async function remove(id: string): Promise<void> {
    await invokeDeletePreset(id);
    await load();
  }

  /** 启动指定 id 的 preset 对应的 Claude CLI 会话。 */
  async function launch(id: string): Promise<void> {
    await invokeLaunchPreset(id);
  }

  return { list, load, save, remove, launch };
});
