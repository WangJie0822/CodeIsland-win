<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  toolInput: Record<string, unknown>;
  toolOutput?: unknown;
  toolStatus?: "pending" | "running" | "success" | "error";
}>();

const statusClass = computed(() => props.toolStatus ?? "pending");

const query = computed(() => {
  const q = props.toolInput.query ?? props.toolInput.q ?? props.toolInput.search_query;
  return typeof q === "string" ? q : "";
});

const resultCount = computed(() => {
  if (props.toolOutput == null) return null;
  if (Array.isArray(props.toolOutput)) return props.toolOutput.length;
  if (typeof props.toolOutput === "object") {
    const obj = props.toolOutput as Record<string, unknown>;
    if (Array.isArray(obj.results)) return obj.results.length;
    if (typeof obj.total === "number") return obj.total;
  }
  return null;
});
</script>

<template>
  <div class="tool-result websearch-result" :class="statusClass">
    <template v-if="toolStatus === 'pending'">
      <div class="skeleton-line" style="width: 75%" />
      <div class="skeleton-line" style="width: 30%" />
    </template>
    <template v-else>
      <div class="result-header">
        <span class="badge">WebSearch</span>
        <span class="query-text">{{ query }}</span>
        <span v-if="resultCount != null && toolStatus === 'success'" class="count-text">
          {{ resultCount }} 个结果
        </span>
        <span v-if="toolStatus === 'error'" class="error-badge">ERROR</span>
      </div>
      <div v-if="toolStatus === 'error'" class="error-bar">
        {{ typeof toolOutput === "string" ? toolOutput : JSON.stringify(toolOutput) }}
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

.query-text {
  font-size: 10px;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
  font-style: italic;
}

.count-text {
  font-size: 10px;
  color: var(--text-tertiary);
  flex-shrink: 0;
}

.error-badge { font-size: 10px; color: var(--accent-red); font-weight: 600; }

.error-bar {
  color: var(--accent-red);
  font-family: var(--font-mono);
  font-size: 10px;
  white-space: pre-wrap;
  word-break: break-all;
  margin-top: 4px;
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
