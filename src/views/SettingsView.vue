<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useSettingsStore } from "@/stores/settings";
import SystemSettings from "@/components/settings/SystemSettings.vue";
import SoundSettings from "@/components/settings/SoundSettings.vue";
import NotchCustomizationSettings from "@/components/settings/NotchCustomizationSettings.vue";
import HooksSettings from "@/components/settings/HooksSettings.vue";

const settingsStore = useSettingsStore();

type TabKey = "system" | "sound" | "notch" | "hooks";
const activeTab = ref<TabKey>("system");

const TABS: { key: TabKey; label: string }[] = [
  { key: "system", label: "系统" },
  { key: "sound", label: "声音" },
  { key: "notch", label: "刘海" },
  { key: "hooks", label: "Hooks" },
];

onMounted(async () => {
  await settingsStore.load();
});
</script>

<template>
  <div class="settings-view">
    <header class="settings-header" data-tauri-drag-region>
      <span class="settings-title">偏好设置</span>
    </header>

    <!-- 标签栏 -->
    <nav class="tab-bar" role="tablist">
      <button
        v-for="tab in TABS"
        :key="tab.key"
        class="tab-btn"
        :class="{ active: activeTab === tab.key }"
        role="tab"
        :aria-selected="activeTab === tab.key"
        @click="activeTab = tab.key"
      >
        {{ tab.label }}
      </button>
    </nav>

    <!-- 加载/错误状态 -->
    <div v-if="settingsStore.loading" class="state-msg">加载中…</div>
    <div v-else-if="settingsStore.error" class="state-msg error">
      加载失败：{{ settingsStore.error }}
    </div>

    <!-- 内容区 -->
    <div v-else class="settings-body">
      <SystemSettings v-if="activeTab === 'system'" />
      <SoundSettings v-else-if="activeTab === 'sound'" />
      <NotchCustomizationSettings v-else-if="activeTab === 'notch'" />
      <HooksSettings v-else-if="activeTab === 'hooks'" />
    </div>
  </div>
</template>

<style scoped>
.settings-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-notch);
  color: var(--text-primary);
  font-family: var(--font-sans);
  border-radius: var(--radius);
  overflow: hidden;
}

.settings-header {
  display: flex;
  align-items: center;
  padding: 14px 16px 10px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.06);
}

.settings-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.tab-bar {
  display: flex;
  gap: 0;
  padding: 8px 12px 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.06);
}

.tab-btn {
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  color: var(--text-secondary);
  font-size: 12px;
  font-family: var(--font-sans);
  cursor: pointer;
  padding: 6px 12px;
  transition: color var(--dur-fast), border-color var(--dur-fast);
  white-space: nowrap;
}

.tab-btn:hover {
  color: var(--text-primary);
}

.tab-btn.active {
  color: var(--text-primary);
  border-bottom-color: var(--accent-blue);
}

.settings-body {
  flex: 1;
  overflow-y: auto;
  padding: 0 16px 16px;
  scrollbar-width: thin;
  scrollbar-color: rgba(255, 255, 255, 0.1) transparent;
}

.settings-body::-webkit-scrollbar {
  width: 4px;
}

.settings-body::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 2px;
}

.state-msg {
  padding: 24px 16px;
  font-size: 12px;
  color: var(--text-secondary);
  text-align: center;
}

.state-msg.error {
  color: var(--accent-red);
}
</style>
