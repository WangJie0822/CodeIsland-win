<script setup lang="ts">
import { computed, ref } from "vue";

const props = defineProps<{
  toolInput: Record<string, unknown>;
  toolOutput?: unknown;
  toolStatus?: "pending" | "running" | "success" | "error";
}>();

const expanded = ref(false);
const statusClass = computed(() => props.toolStatus ?? "pending");

function abbreviatePath(p: string): string {
  if (!p) return "";
  const parts = p.replace(/\\/g, "/").split("/").filter(Boolean);
  if (parts.length <= 2) return p;
  return "…/" + parts.slice(-2).join("/");
}

const filePath = computed(() => String(props.toolInput.file_path ?? props.toolInput.path ?? ""));
const shortPath = computed(() => abbreviatePath(filePath.value));

/** 统计改动段数：MultiEdit 有 edits 数组，Edit 算 1 段 */
const segmentCount = computed(() => {
  const edits = props.toolInput.edits;
  if (Array.isArray(edits)) return edits.length;
  return props.toolInput.old_string != null ? 1 : 0;
});

/** 生成简单 diff 预览（前 20 行） */
const diffLines = computed((): string[] => {
  if (props.toolOutput == null && props.toolInput.old_string == null) return [];
  const out = typeof props.toolOutput === "string"
    ? props.toolOutput
    : JSON.stringify(props.toolOutput, null, 2);
  return out.split("\n").slice(0, 20);
});

const hasMoreDiff = computed(() => {
  if (props.toolOutput == null) return false;
  const s = typeof props.toolOutput === "string"
    ? props.toolOutput
    : JSON.stringify(props.toolOutput);
  return s.split("\n").length > 20;
});
</script>

<template>
  <div class="tool-result edit-result" :class="statusClass">
    <template v-if="toolStatus === 'pending'">
      <div class="skeleton-line" style="width: 60%" />
      <div class="skeleton-line" style="width: 45%" />
    </template>
    <template v-else>
      <div class="result-header">
        <span class="badge">Edit</span>
        <span class="path-text" :title="filePath">{{ shortPath }}</span>
        <span v-if="segmentCount > 0" class="seg-text">{{ segmentCount }} 段改动</span>
        <span v-if="toolStatus === 'error'" class="error-badge">ERROR</span>
        <button class="expand-btn" @click="expanded = !expanded">
          {{ expanded ? "收起" : "展开 diff" }}
        </button>
      </div>
      <div v-if="toolStatus === 'error'" class="error-bar">
        {{ typeof toolOutput === "string" ? toolOutput : JSON.stringify(toolOutput) }}
      </div>
      <div v-else-if="expanded && diffLines.length > 0" class="diff-block">
        <pre class="mono-pre">{{ diffLines.join("\n") }}</pre>
        <div v-if="hasMoreDiff" class="more-hint">…diff 已截断，展开前 20 行</div>
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

.seg-text {
  font-size: 10px;
  color: var(--accent-yellow);
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

.diff-block { margin-top: 4px; }

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
