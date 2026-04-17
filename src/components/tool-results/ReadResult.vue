<script setup lang="ts">
import { computed, ref } from "vue";

const props = defineProps<{
  toolInput: Record<string, unknown>;
  toolOutput?: unknown;
  toolStatus?: "pending" | "running" | "success" | "error";
}>();

const expanded = ref(false);
const statusClass = computed(() => props.toolStatus ?? "pending");

/** 缩写文件路径：保留末尾两段 */
function abbreviatePath(p: string): string {
  if (!p) return "";
  const parts = p.replace(/\\/g, "/").split("/").filter(Boolean);
  if (parts.length <= 2) return p;
  return "…/" + parts.slice(-2).join("/");
}

const filePath = computed(() => String(props.toolInput.file_path ?? props.toolInput.path ?? ""));
const shortPath = computed(() => abbreviatePath(filePath.value));

const offset = computed(() => Number(props.toolInput.offset ?? 1));
const limit = computed(() => props.toolInput.limit != null ? Number(props.toolInput.limit) : null);

const lineRange = computed(() => {
  const start = offset.value;
  const end = limit.value != null ? start + limit.value - 1 : null;
  return end != null ? `L${start}-${end}` : `L${start}+`;
});

const outputLines = computed((): string[] => {
  if (props.toolOutput == null) return [];
  const s = typeof props.toolOutput === "string"
    ? props.toolOutput
    : JSON.stringify(props.toolOutput, null, 2);
  return s.split("\n");
});

const totalLines = computed(() => outputLines.value.length);
const previewLines = computed(() => outputLines.value.slice(0, 20));
</script>

<template>
  <div class="tool-result read-result" :class="statusClass">
    <template v-if="toolStatus === 'pending'">
      <div class="skeleton-line" style="width: 65%" />
      <div class="skeleton-line" style="width: 30%" />
    </template>
    <template v-else>
      <div class="result-header">
        <span class="badge">Read</span>
        <span class="path-text" :title="filePath">{{ shortPath }}</span>
        <span class="range-text">{{ lineRange }}</span>
        <span v-if="toolStatus === 'success'" class="total-text">{{ totalLines }} 行</span>
        <span v-if="toolStatus === 'error'" class="error-badge">ERROR</span>
        <button class="expand-btn" @click="expanded = !expanded">
          {{ expanded ? "收起" : "展开" }}
        </button>
      </div>
      <div v-if="toolStatus === 'error'" class="error-bar">
        {{ typeof toolOutput === "string" ? toolOutput : JSON.stringify(toolOutput) }}
      </div>
      <div v-else-if="expanded && previewLines.length > 0" class="code-block">
        <pre class="mono-pre">{{ previewLines.join("\n") }}</pre>
        <div v-if="totalLines > 20" class="more-hint">…还有 {{ totalLines - 20 }} 行</div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.tool-result {
  max-height: 160px;
  overflow-y: auto;
  padding: 6px 8px;
  font-size: 11px;
  color: var(--text-secondary);
  border-left: 2px solid var(--text-tertiary);
}
.tool-result.pending { border-left-color: var(--text-tertiary); }
.tool-result.running { border-left-color: var(--accent-blue); }
.tool-result.success { border-left-color: var(--accent-green); }
.tool-result.error   { border-left-color: var(--accent-red); }

.result-header {
  display: flex;
  align-items: center;
  gap: 5px;
  flex-wrap: wrap;
}

.badge {
  font-family: var(--font-mono);
  font-size: 10px;
  background: rgba(255,255,255,0.08);
  padding: 1px 5px;
  border-radius: 3px;
  color: var(--text-primary);
  flex-shrink: 0;
}

.path-text {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 180px;
}

.range-text {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--accent-blue);
  flex-shrink: 0;
}

.total-text {
  font-size: 10px;
  color: var(--text-tertiary);
  flex-shrink: 0;
}

.error-badge { font-size: 10px; color: var(--accent-red); font-weight: 600; }

.expand-btn {
  margin-left: auto;
  background: none;
  border: none;
  color: var(--text-tertiary);
  font-size: 10px;
  cursor: pointer;
  padding: 1px 4px;
}
.expand-btn:hover { color: var(--text-secondary); }

.error-bar {
  color: var(--accent-red);
  font-family: var(--font-mono);
  font-size: 10px;
  white-space: pre-wrap;
  word-break: break-all;
  margin-top: 4px;
}

.code-block { margin-top: 4px; }

.mono-pre {
  font-family: var(--font-mono);
  font-size: 10px;
  white-space: pre-wrap;
  word-break: break-all;
  color: var(--text-secondary);
}

.more-hint {
  font-size: 10px;
  color: var(--text-tertiary);
  margin-top: 2px;
}

.skeleton-line {
  height: 10px;
  background: rgba(255,255,255,0.06);
  border-radius: 3px;
  margin-bottom: 6px;
  animation: pulse 1.2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 0.4; }
  50% { opacity: 0.8; }
}
</style>
