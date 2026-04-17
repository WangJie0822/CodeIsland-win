<script setup lang="ts">
import { computed } from "vue";
import { useSettingsStore } from "@/stores/settings";

const settingsStore = useSettingsStore();

const notificationSound = computed({
  get: () => settingsStore.value?.notification_sound ?? "default",
  set: (v: string) => void settingsStore.update({ notification_sound: v }),
});

const SOUND_OPTIONS = [
  { value: "default", label: "默认提示音" },
  { value: "chime", label: "清脆提示" },
  { value: "soft", label: "柔和提示" },
  { value: "none", label: "静音" },
];
</script>

<template>
  <section class="settings-section">
    <h3 class="section-title">声音设置</h3>

    <!-- 通知音效 -->
    <div class="setting-row">
      <label class="setting-label">通知音效</label>
      <div class="setting-control">
        <select
          class="picker"
          :value="notificationSound"
          @change="notificationSound = ($event.target as HTMLSelectElement).value"
        >
          <option
            v-for="opt in SOUND_OPTIONS"
            :key="opt.value"
            :value="opt.value"
          >
            {{ opt.label }}
          </option>
        </select>
      </div>
    </div>

    <!-- 音量（预留 UI，Stage 2 接入音量命令） -->
    <div class="setting-row">
      <label class="setting-label">音量</label>
      <div class="setting-control volume-control">
        <input
          type="range"
          class="slider"
          min="0"
          max="100"
          step="1"
          value="80"
          aria-label="音量"
        />
        <span class="hint">80%</span>
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

.volume-control {
  min-width: 140px;
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

.slider {
  flex: 1;
  accent-color: var(--accent-blue);
  cursor: pointer;
}

.hint {
  font-size: 11px;
  color: var(--text-secondary);
  min-width: 32px;
  text-align: right;
}
</style>
