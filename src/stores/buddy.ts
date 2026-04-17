import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// ── 本地类型替身（Stage 2 由 generated.ts 补 Buddy domain 类型后替换）──────────
export interface BuddyStats {
  debug: number;
  patience: number;
  chaos: number;
  wisdom: number;
  sneak: number;
}

export type Rarity = 'Common' | 'Rare' | 'Epic' | 'Legendary';

export interface Buddy {
  id: string;
  name: string;
  display_name: string;
  rarity: Rarity;
  ascii_art: string[];
  description: string;
  stats: BuddyStats;
  unlocked_at: number | null;
}

export interface BuddyState {
  current: string;
  roster: Buddy[];
  completed_sessions: number;
}
// ─────────────────────────────────────────────────────────────────────────────

export const useBuddyStore = defineStore('buddy', () => {
  const current = ref<Buddy | null>(null);
  const roster = ref<Buddy[]>([]);
  const completedSessions = ref(0);

  /** 从后端加载 BuddyState，更新 store */
  async function load(): Promise<void> {
    try {
      const state: BuddyState = await invoke('get_buddy');
      _applyState(state);
    } catch (e) {
      console.error('[buddy] load 失败:', e);
    }
  }

  /** 切换当前角色 */
  async function switchBuddy(id: string): Promise<void> {
    try {
      const state: BuddyState = await invoke('switch_buddy', { id });
      _applyState(state);
    } catch (e) {
      console.error('[buddy] switchBuddy 失败:', e);
    }
  }

  /** 内部：将 BuddyState 写入 store refs */
  function _applyState(state: BuddyState): void {
    roster.value = state.roster;
    completedSessions.value = state.completed_sessions;
    current.value = state.roster.find((b) => b.id === state.current) ?? null;
  }

  return { current, roster, completedSessions, load, switchBuddy };
});
