<script setup lang="ts">
/**
 * LaunchPresetsView — 启动预设列表视图。
 *
 * 展示所有已保存的 preset 卡片，支持：
 * - 新建 preset（打开 PresetEditor 模态）
 * - 编辑 preset
 * - 删除 preset
 * - 一键启动 preset（invoke launch_preset）
 */
import { ref, onMounted } from "vue";
import { usePresetsStore } from "@/stores/presets";
import type { LaunchPreset } from "@/stores/presets";
import PresetEditor from "@/components/preset-editor/PresetEditor.vue";

const store = usePresetsStore();

/** 编辑器状态：null = 关闭，LaunchPreset(id='') = 新建，否则 = 编辑 */
const editingPreset = ref<LaunchPreset | null>(null);
const isEditorOpen = ref(false);

/** 正在启动中的 preset id */
const launchingId = ref<string | null>(null);

/** 错误提示（简单字符串） */
const errorMsg = ref<string | null>(null);

onMounted(() => {
  store.load();
});

function openNew() {
  editingPreset.value = null;
  isEditorOpen.value = true;
}

function openEdit(preset: LaunchPreset) {
  editingPreset.value = { ...preset };
  isEditorOpen.value = true;
}

function closeEditor() {
  isEditorOpen.value = false;
  editingPreset.value = null;
}

async function onSave(preset: LaunchPreset) {
  try {
    await store.save(preset);
    closeEditor();
  } catch (e) {
    errorMsg.value = `保存失败: ${e}`;
  }
}

async function onDelete(id: string) {
  try {
    await store.remove(id);
  } catch (e) {
    errorMsg.value = `删除失败: ${e}`;
  }
}

async function onLaunch(id: string) {
  launchingId.value = id;
  errorMsg.value = null;
  try {
    await store.launch(id);
  } catch (e) {
    errorMsg.value = `启动失败: ${e}`;
  } finally {
    launchingId.value = null;
  }
}
</script>

<template>
  <div class="presets-view" data-testid="presets-view">
    <!-- 顶部工具栏 -->
    <div class="presets-toolbar">
      <span class="presets-title">启动预设</span>
      <button
        class="btn btn--new"
        data-testid="btn-new-preset"
        @click="openNew"
      >
        + 新建预设
      </button>
    </div>

    <!-- 错误提示 -->
    <div v-if="errorMsg" class="error-banner" data-testid="error-banner">
      {{ errorMsg }}
      <button class="error-close" @click="errorMsg = null">✕</button>
    </div>

    <!-- 空状态 -->
    <div
      v-if="store.list.length === 0"
      class="empty-state"
      data-testid="empty-state"
    >
      暂无预设，点击「新建预设」开始
    </div>

    <!-- Preset 卡片列表 -->
    <div class="presets-list">
      <div
        v-for="preset in store.list"
        :key="preset.id"
        class="preset-card"
        :data-testid="`preset-card-${preset.id}`"
      >
        <!-- 图标 + 名称 + 路径 -->
        <div class="preset-card-main">
          <span class="preset-icon" aria-hidden="true">{{ preset.icon }}</span>
          <div class="preset-info">
            <span class="preset-name">{{ preset.name }}</span>
            <span class="preset-cwd">{{ preset.cwd }}</span>
            <span v-if="preset.model && preset.model !== 'inherit'" class="preset-meta">
              {{ preset.model }}
            </span>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="preset-card-actions">
          <button
            class="btn btn--launch"
            :data-testid="`btn-launch-${preset.id}`"
            :disabled="launchingId === preset.id"
            @click="onLaunch(preset.id)"
          >
            {{ launchingId === preset.id ? "启动中…" : "启动" }}
          </button>
          <button
            class="btn btn--edit"
            :data-testid="`btn-edit-${preset.id}`"
            @click="openEdit(preset)"
          >
            编辑
          </button>
          <button
            class="btn btn--delete"
            :data-testid="`btn-delete-${preset.id}`"
            @click="onDelete(preset.id)"
          >
            删除
          </button>
        </div>
      </div>
    </div>

    <!-- PresetEditor 模态 -->
    <PresetEditor
      v-if="isEditorOpen"
      :preset="editingPreset"
      @save="onSave"
      @cancel="closeEditor"
    />
  </div>
</template>

<style scoped>
.presets-view {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 16px;
  min-height: 200px;
}

.presets-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.presets-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.empty-state {
  color: var(--text-secondary);
  font-size: 13px;
  text-align: center;
  padding: 24px 0;
}

.presets-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.preset-card {
  background: var(--bg-secondary);
  border-radius: var(--radius-sm);
  padding: 12px 14px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.preset-card-main {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  flex: 1;
  min-width: 0;
}

.preset-icon {
  font-size: 18px;
  line-height: 1;
  flex-shrink: 0;
}

.preset-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.preset-name {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.preset-cwd {
  font-size: 11px;
  color: var(--text-secondary);
  font-family: var(--font-mono);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.preset-meta {
  font-size: 11px;
  color: var(--text-tertiary);
}

.preset-card-actions {
  display: flex;
  gap: 6px;
  flex-shrink: 0;
}

.btn {
  padding: 4px 12px;
  border: none;
  border-radius: 6px;
  font-size: 12px;
  cursor: pointer;
  font-family: var(--font-sans);
  white-space: nowrap;
}

.btn--new {
  background: var(--accent-green);
  color: #fff;
  padding: 5px 14px;
  font-size: 13px;
}

.btn--new:hover {
  filter: brightness(1.1);
}

.btn--launch {
  background: var(--accent-green);
  color: #fff;
}

.btn--launch:hover:not(:disabled) {
  filter: brightness(1.1);
}

.btn--launch:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn--edit {
  background: rgba(255, 255, 255, 0.1);
  color: var(--text-primary);
}

.btn--edit:hover {
  background: rgba(255, 255, 255, 0.15);
}

.btn--delete {
  background: var(--accent-red);
  color: #fff;
}

.btn--delete:hover {
  filter: brightness(1.1);
}

.error-banner {
  background: rgba(244, 67, 54, 0.15);
  border: 1px solid var(--accent-red);
  border-radius: 6px;
  padding: 8px 12px;
  font-size: 12px;
  color: var(--accent-red);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.error-close {
  background: none;
  border: none;
  color: var(--accent-red);
  cursor: pointer;
  font-size: 12px;
  padding: 0 2px;
}
</style>
