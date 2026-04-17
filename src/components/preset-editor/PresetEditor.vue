<script setup lang="ts">
/**
 * PresetEditor — 新建/编辑 LaunchPreset 的模态表单。
 *
 * Props:
 *   - preset: 编辑时传入已有 preset；新建时传入 null。
 * Emits:
 *   - save(preset): 用户点击保存后触发。
 *   - cancel: 用户点击取消后触发。
 *
 * NOTE（Stage 2 待办）：
 *   - cwd 字段目前为 text input；待主会话实现 `pick_folder` 命令后替换为文件夹选择器。
 */
import { reactive, computed } from "vue";
import type { LaunchPreset } from "@/stores/presets";

const MODEL_OPTIONS = [
  { value: "inherit", label: "使用默认（inherit）" },
  { value: "claude-opus-4-5", label: "Claude Opus" },
  { value: "claude-sonnet-4-5", label: "Claude Sonnet" },
  { value: "claude-haiku-3-5", label: "Claude Haiku" },
];

const props = defineProps<{
  preset: LaunchPreset | null;
}>();

const emit = defineEmits<{
  save: [preset: LaunchPreset];
  cancel: [];
}>();

// 表单状态（以 preset 初始化，或用默认值）
const form = reactive<{
  name: string;
  icon: string;
  cwd: string;
  initial_prompt: string;
  model: string;
  agent: string;
  mcp_servers_raw: string; // 逗号分隔字符串
  sort_order: number;
}>({
  name: props.preset?.name ?? "",
  icon: props.preset?.icon ?? "🚀",
  cwd: props.preset?.cwd ?? "",
  initial_prompt: props.preset?.initial_prompt ?? "",
  model: props.preset?.model ?? "inherit",
  agent: props.preset?.agent ?? "",
  mcp_servers_raw: (props.preset?.mcp_servers ?? []).join(", "),
  sort_order: props.preset?.sort_order ?? 0,
});

const isValid = computed(
  () => form.name.trim().length > 0 && form.cwd.trim().length > 0,
);

function onSave() {
  if (!isValid.value) return;

  const mcp_servers = form.mcp_servers_raw
    .split(",")
    .map((s) => s.trim())
    .filter((s) => s.length > 0);

  const preset: LaunchPreset = {
    id: props.preset?.id ?? "",
    name: form.name.trim(),
    icon: form.icon.trim() || "🚀",
    cwd: form.cwd.trim(),
    initial_prompt: form.initial_prompt.trim() || null,
    model: form.model === "inherit" ? null : form.model || null,
    agent: form.agent.trim() || null,
    mcp_servers,
    sort_order: form.sort_order,
  };

  emit("save", preset);
}
</script>

<template>
  <div class="preset-editor-overlay" data-testid="preset-editor-overlay" @click.self="$emit('cancel')">
    <div class="preset-editor" data-testid="preset-editor">
      <h2 class="editor-title">{{ preset ? "编辑预设" : "新建预设" }}</h2>

      <div class="form-row">
        <label class="form-label">图标</label>
        <input
          v-model="form.icon"
          class="form-input form-input--icon"
          data-testid="input-icon"
          placeholder="🚀"
          maxlength="4"
        />
      </div>

      <div class="form-row">
        <label class="form-label">名称 <span class="required">*</span></label>
        <input
          v-model="form.name"
          class="form-input"
          data-testid="input-name"
          placeholder="我的项目"
        />
      </div>

      <div class="form-row">
        <label class="form-label">工作目录 <span class="required">*</span></label>
        <div class="cwd-row">
          <!-- Stage 2 待办：替换为文件夹选择器（invoke pick_folder） -->
          <input
            v-model="form.cwd"
            class="form-input"
            data-testid="input-cwd"
            placeholder="C:\projects\my-project"
          />
          <span class="cwd-hint">（Stage 2 将支持文件夹选择）</span>
        </div>
      </div>

      <div class="form-row">
        <label class="form-label">初始提示词</label>
        <textarea
          v-model="form.initial_prompt"
          class="form-textarea"
          data-testid="input-initial-prompt"
          placeholder="启动时自动发送的提示词（可选）"
          rows="3"
        />
      </div>

      <div class="form-row">
        <label class="form-label">模型</label>
        <select
          v-model="form.model"
          class="form-select"
          data-testid="select-model"
        >
          <option v-for="opt in MODEL_OPTIONS" :key="opt.value" :value="opt.value">
            {{ opt.label }}
          </option>
        </select>
      </div>

      <div class="form-row">
        <label class="form-label">Agent（可选）</label>
        <input
          v-model="form.agent"
          class="form-input"
          data-testid="input-agent"
          placeholder="agent 名称"
        />
      </div>

      <div class="form-row">
        <label class="form-label">MCP Servers</label>
        <input
          v-model="form.mcp_servers_raw"
          class="form-input"
          data-testid="input-mcp-servers"
          placeholder="server-a, server-b（逗号分隔）"
        />
      </div>

      <div class="form-row">
        <label class="form-label">排序序号</label>
        <input
          v-model.number="form.sort_order"
          type="number"
          class="form-input form-input--short"
          data-testid="input-sort-order"
        />
      </div>

      <div class="editor-actions">
        <button
          class="btn btn--cancel"
          data-testid="btn-cancel"
          @click="$emit('cancel')"
        >
          取消
        </button>
        <button
          class="btn btn--save"
          data-testid="btn-save"
          :disabled="!isValid"
          @click="onSave"
        >
          保存
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.preset-editor-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.preset-editor {
  background: var(--bg-secondary);
  border-radius: var(--radius-sm);
  padding: 20px;
  width: 480px;
  max-height: 90vh;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.editor-title {
  font-size: 15px;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.form-row {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.form-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.required {
  color: var(--accent-red);
}

.form-input,
.form-select,
.form-textarea {
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 13px;
  padding: 6px 8px;
  outline: none;
  font-family: var(--font-sans);
}

.form-input:focus,
.form-select:focus,
.form-textarea:focus {
  border-color: var(--accent-blue);
}

.form-input--icon {
  width: 60px;
  text-align: center;
}

.form-input--short {
  width: 80px;
}

.form-textarea {
  resize: vertical;
}

.form-select option {
  background: #1e1e1e;
}

.cwd-row {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.cwd-hint {
  font-size: 11px;
  color: var(--text-tertiary);
}

.editor-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: 8px;
}

.btn {
  padding: 6px 16px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  cursor: pointer;
  font-family: var(--font-sans);
}

.btn--cancel {
  background: rgba(255, 255, 255, 0.1);
  color: var(--text-primary);
}

.btn--cancel:hover {
  background: rgba(255, 255, 255, 0.15);
}

.btn--save {
  background: var(--accent-green);
  color: #fff;
}

.btn--save:hover:not(:disabled) {
  filter: brightness(1.1);
}

.btn--save:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}
</style>
