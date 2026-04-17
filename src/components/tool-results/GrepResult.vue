<script setup lang="ts">
import { computed, ref } from "vue";

const props = defineProps<{
  toolInput: Record<string, unknown>;
  toolOutput?: unknown;
  toolStatus?: "pending" | "running" | "success" | "error";
}>();

const expanded = ref(false);
const statusClass = computed(() => props.toolStatus ?? "pending");

const pattern = computed(() => String(props.toolInput.pattern ?? ""));

/** 解析输出：按行提取文件路径（去重） */
const matchedFiles = computed((): string[] => {
  if (props.toolOutput == null) return [];
  const out = typeof props.toolOutput === "string"
    ? props.toolOutput
    : JSON.stringify(props.toolOutput, null, 2);
  const lines = out.split("\n").filter(Boolean);
  const seen = new Set<string>();
  const files: string[] = [];
  for (const line of lines) {
    // 格式：path:line_no:content 或仅 path
    const colonIdx = line.indexOf(":");
    const file = colonIdx > 0 ? line.slice(0, colonIdx) : line;
    if (!seen.has(file)) {
      seen.add(file);
      files.push(file);
    }
  }
  return files;
});

const previewFiles = computed(() => matchedFiles.value.slice(0, 5));
const totalFiles = computed(() => matchedFiles.value.length);
</script>

<template>
  <div class="tool-result grep-result" :class="statusClass">
    <template v-if="toolStatus === 'pending'">
      <div class="skeleton-line" style="width: 55%" />
      <div class="skeleton-line" style="width: 40%" />
    </template>
    <template v-else>
      <div class="result-header">
        <span class="badge">Grep</span>
        <code class="pattern-text">{{ pattern }}</code>
        <span v-if="toolStatus === 'success'" class="count-text">{{ totalFiles }} 个文件</span>
        <span v-if="toolStatus === 'error'" class="error-badge">ERROR</span>
        <button v-if="totalFiles > 0" class="expand-btn" @click="expanded = !expanded">
          {{ expanded ? "收起" : "展开" }}
        </button>
      </div>
      <div v-if="toolStatus === 'error'" class="error-bar">
        {{ typeof toolOutput === "string" ? toolOutput : JSON.stringify(toolOutput) }}
      </div>
      <ul v-else-if="previewFiles.length > 0 || expanded" class="file-list">
        <li v-for="f in previewFiles" :key="f" class="file-item">{{ f }}</li>
        <li v-if="totalFiles > 5 && !expanded" class="more-hint">…还有 {{ totalFiles - 5 }} 个文件</li>
        <template v-if="expanded">
          <li v-for="f in matchedFiles.slice(5)" :key="f" class="file-item">{{ f }}</li>
        </template>
      </ul>
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
  flex-wrap: nowrap;
  overflow: hidden;
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

.pattern-text {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--accent-yellow);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.count-text {
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
  flex-shrink: 0;
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

.file-list {
  list-style: none;
  margin-top: 4px;
}

.file-item {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  padding: 1px 0;
}

.more-hint {
  font-size: 10px;
  color: var(--text-tertiary);
  padding: 1px 0;
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
