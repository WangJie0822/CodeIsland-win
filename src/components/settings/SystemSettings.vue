<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useSettingsStore } from "@/stores/settings";

// @stage2 替换：从 '@/types/generated' 导入 ScreenInfo
interface ScreenInfo {
  id: number;
  name: string;
  width: number;
  height: number;
  is_primary: boolean;
}

const settingsStore = useSettingsStore();
const screens = ref<ScreenInfo[]>([]);
const screensLoading = ref(false);

const targetScreen = computed({
  get: () => settingsStore.value?.target_screen ?? null,
  set: (v: number | null) => void settingsStore.update({ target_screen: v }),
});

const language = computed({
  get: () => settingsStore.value?.language ?? "en",
  set: (v: string) => void settingsStore.update({ language: v }),
});

const launchAtLogin = computed({
  get: () => settingsStore.value?.launch_at_login ?? false,
  set: (v: boolean) => void settingsStore.update({ launch_at_login: v }),
});

const hooksEnabled = computed({
  get: () => settingsStore.value?.hooks_enabled ?? true,
  set: (v: boolean) => void settingsStore.update({ hooks_enabled: v }),
});

async function loadScreens() {
  screensLoading.value = true;
  try {
    screens.value = await invoke<ScreenInfo[]>("get_screens");
  } catch (e) {
    console.error("[SystemSettings] 获取屏幕失败:", e);
  } finally {
    screensLoading.value = false;
  }
}

onMounted(loadScreens);

const LANGUAGES = [
  { value: "en", label: "English" },
  { value: "zh", label: "中文" },
];
</script>

<template>
  <section class="settings-section">
    <h3 class="section-title">系统设置</h3>

    <!-- Screen 选择 -->
    <div class="setting-row">
      <label class="setting-label">显示屏幕</label>
      <div class="setting-control">
        <select
          class="picker"
          :value="targetScreen ?? ''"
          @change="targetScreen = ($event.target as HTMLSelectElement).value === '' ? null : Number(($event.target as HTMLSelectElement).value)"
        >
          <option value="">自动（主显示器）</option>
          <option
            v-for="s in screens"
            :key="s.id"
            :value="s.id"
          >
            {{ s.name }} ({{ s.width }}×{{ s.height }}){{ s.is_primary ? " [主]" : "" }}
          </option>
        </select>
        <span v-if="screensLoading" class="hint">加载中…</span>
      </div>
    </div>

    <!-- Language 选择 -->
    <div class="setting-row">
      <label class="setting-label">语言</label>
      <div class="setting-control">
        <select
          class="picker"
          :value="language"
          @change="language = ($event.target as HTMLSelectElement).value"
        >
          <option v-for="lang in LANGUAGES" :key="lang.value" :value="lang.value">
            {{ lang.label }}
          </option>
        </select>
      </div>
    </div>

    <!-- Launch at Login -->
    <div class="setting-row">
      <label class="setting-label">登录时启动</label>
      <div class="setting-control">
        <button
          class="toggle"
          :class="{ active: launchAtLogin }"
          role="switch"
          :aria-checked="launchAtLogin"
          @click="launchAtLogin = !launchAtLogin"
        >
          <span class="toggle-thumb" />
        </button>
      </div>
    </div>

    <!-- Hooks -->
    <div class="setting-row">
      <label class="setting-label">Claude Hooks</label>
      <div class="setting-control">
        <button
          class="toggle"
          :class="{ active: hooksEnabled }"
          role="switch"
          :aria-checked="hooksEnabled"
          @click="hooksEnabled = !hooksEnabled"
        >
          <span class="toggle-thumb" />
        </button>
        <span class="hint">{{ hooksEnabled ? "已启用" : "已禁用" }}</span>
      </div>
    </div>
  </section>
</template>

<style scoped>
.settings-section {
  padding: 12px 0;
}

.section-title {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.6px;
  margin-bottom: 12px;
}

.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.setting-row:last-child {
  border-bottom: none;
}

.setting-label {
  font-size: 13px;
  color: var(--text-primary);
  flex: 1;
}

.setting-control {
  display: flex;
  align-items: center;
  gap: 8px;
}

.picker {
  background: var(--bg-secondary);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  font-size: 12px;
  padding: 4px 8px;
  outline: none;
  cursor: pointer;
}

.picker:focus {
  border-color: var(--accent-blue);
}

.toggle {
  position: relative;
  width: 36px;
  height: 20px;
  background: var(--text-tertiary);
  border: none;
  border-radius: 10px;
  cursor: pointer;
  transition: background var(--dur-fast);
  padding: 0;
}

.toggle.active {
  background: var(--accent-green);
}

.toggle-thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 16px;
  height: 16px;
  background: #fff;
  border-radius: 50%;
  transition: transform var(--dur-fast);
}

.toggle.active .toggle-thumb {
  transform: translateX(16px);
}

.hint {
  font-size: 11px;
  color: var(--text-secondary);
}
</style>
