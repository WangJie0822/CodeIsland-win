<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useSettingsStore } from "@/stores/settings";

const settingsStore = useSettingsStore();

const pixelCatEnabled = computed({
  get: () => settingsStore.value?.pixel_cat_enabled ?? false,
  set: (v: boolean) => void settingsStore.update({ pixel_cat_enabled: v }),
});

const groupByProject = computed({
  get: () => settingsStore.value?.group_by_project ?? false,
  set: (v: boolean) => void settingsStore.update({ group_by_project: v }),
});

const smartSuppression = computed({
  get: () => settingsStore.value?.smart_suppression ?? false,
  set: (v: boolean) => void settingsStore.update({ smart_suppression: v }),
});

const autoCollapseOnLeave = computed({
  get: () => settingsStore.value?.auto_collapse_on_leave ?? true,
  set: (v: boolean) => void settingsStore.update({ auto_collapse_on_leave: v }),
});

// 使用本地 ref 避免每次输入都触发 update，失焦时才提交
const autoCollapseMsLocal = ref<number>(settingsStore.value?.auto_collapse_ms ?? 3000);

watch(
  () => settingsStore.value?.auto_collapse_ms,
  (v) => {
    if (v !== undefined) autoCollapseMsLocal.value = v;
  },
);

function commitAutoCollapseMs() {
  const clamped = Math.max(500, Math.min(30000, autoCollapseMsLocal.value));
  autoCollapseMsLocal.value = clamped;
  void settingsStore.update({ auto_collapse_ms: clamped });
}
</script>

<template>
  <section class="settings-section">
    <h3 class="section-title">刘海自定义</h3>

    <!-- Pixel Cat -->
    <div class="setting-row">
      <div class="setting-label-group">
        <span class="setting-label">像素猫</span>
        <span class="setting-desc">在刘海显示像素风格小猫动画</span>
      </div>
      <button
        class="toggle"
        :class="{ active: pixelCatEnabled }"
        role="switch"
        :aria-checked="pixelCatEnabled"
        @click="pixelCatEnabled = !pixelCatEnabled"
      >
        <span class="toggle-thumb" />
      </button>
    </div>

    <!-- Group by Project -->
    <div class="setting-row">
      <div class="setting-label-group">
        <span class="setting-label">按项目分组</span>
        <span class="setting-desc">将同一项目的会话合并显示</span>
      </div>
      <button
        class="toggle"
        :class="{ active: groupByProject }"
        role="switch"
        :aria-checked="groupByProject"
        @click="groupByProject = !groupByProject"
      >
        <span class="toggle-thumb" />
      </button>
    </div>

    <!-- Smart Suppression -->
    <div class="setting-row">
      <div class="setting-label-group">
        <span class="setting-label">智能抑制</span>
        <span class="setting-desc">全屏应用运行时自动隐藏刘海</span>
      </div>
      <button
        class="toggle"
        :class="{ active: smartSuppression }"
        role="switch"
        :aria-checked="smartSuppression"
        @click="smartSuppression = !smartSuppression"
      >
        <span class="toggle-thumb" />
      </button>
    </div>

    <!-- Auto-Collapse on Leave -->
    <div class="setting-row">
      <div class="setting-label-group">
        <span class="setting-label">离开时自动收起</span>
        <span class="setting-desc">鼠标离开刘海后延迟收起</span>
      </div>
      <button
        class="toggle"
        :class="{ active: autoCollapseOnLeave }"
        role="switch"
        :aria-checked="autoCollapseOnLeave"
        @click="autoCollapseOnLeave = !autoCollapseOnLeave"
      >
        <span class="toggle-thumb" />
      </button>
    </div>

    <!-- Auto-Collapse 延迟 ms 输入（仅当开启时显示） -->
    <div v-if="autoCollapseOnLeave" class="setting-row indent">
      <label class="setting-label">收起延迟（ms）</label>
      <div class="setting-control">
        <input
          v-model.number="autoCollapseMsLocal"
          type="number"
          class="ms-input"
          min="500"
          max="30000"
          step="100"
          @blur="commitAutoCollapseMs"
          @keydown.enter="commitAutoCollapseMs"
        />
        <span class="hint">ms</span>
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

.setting-row.indent {
  padding-left: 16px;
  background: rgba(255, 255, 255, 0.02);
  border-radius: var(--radius-sm);
}

.setting-label-group {
  display: flex;
  flex-direction: column;
  gap: 2px;
  flex: 1;
}

.setting-label {
  font-size: 13px;
  color: var(--text-primary);
}

.setting-desc {
  font-size: 11px;
  color: var(--text-secondary);
}

.setting-control {
  display: flex;
  align-items: center;
  gap: 6px;
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
  flex-shrink: 0;
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

.ms-input {
  width: 80px;
  background: var(--bg-secondary);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  font-size: 12px;
  padding: 4px 8px;
  outline: none;
  text-align: right;
}

.ms-input:focus {
  border-color: var(--accent-blue);
}

.hint {
  font-size: 11px;
  color: var(--text-secondary);
}
</style>
