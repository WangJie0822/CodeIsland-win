<script setup lang="ts">
import { computed, ref } from "vue";

const props = defineProps<{
  toolInput: Record<string, unknown>;
  toolOutput?: unknown;
  toolStatus?: "pending" | "running" | "success" | "error";
  toolName?: string;
}>();

const expanded = ref(false);

const statusClass = computed(() => props.toolStatus ?? "pending");

const preview = computed(() => {
  const out = props.toolOutput;
  if (out == null) return null;
  const str = typeof out === "string" ? out : JSON.stringify(out, null, 2);
  return str.length > 500 ? str.slice(0, 500) + "…" : str;
});

const inputPreview = computed(() => {
  const str = JSON.stringify(props.toolInput, null, 2);
  return str.length > 300 ? str.slice(0, 300) + "…" : str;
});
</script>

<template>
  <div class="tool-result generic-result" :class="statusClass">
    <!-- 骨架：pending -->
    <template v-if="toolStatus === 'pending'">
      <div class="skeleton-line" style="width: 60%" />
      <div class="skeleton-line" style="width: 40%" />
    </template>
    <!-- 正常内容 -->
    <template v-else>
      <div class="result-header">
        <span class="tool-name-badge">{{ toolName ?? "Tool" }}</span>
        <span v-if="toolStatus === 'error'" class="error-badge">ERROR</span>
        <button class="expand-btn" @click="expanded = !expanded">
          {{ expanded ? "收起" : "展开" }}
        </button>
      </div>
      <div v-if="toolStatus === 'error'" class="error-bar">
        {{ typeof toolOutput === "string" ? toolOutput : JSON.stringify(toolOutput) }}
      </div>
      <template v-else>
        <pre class="mono-pre">{{ preview }}</pre>
        <div v-if="expanded" class="expanded-content">
          <div class="label">Input:</div>
          <pre class="mono-pre">{{ inputPreview }}</pre>
        </div>
      </template>
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
  gap: 6px;
  margin-bottom: 4px;
}

.tool-name-badge {
  font-family: var(--font-mono);
  font-size: 10px;
  background: rgba(255,255,255,0.08);
  padding: 1px 5px;
  border-radius: 3px;
  color: var(--text-primary);
}

.error-badge {
  font-size: 10px;
  color: var(--accent-red);
  font-weight: 600;
}

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

.mono-pre {
  font-family: var(--font-mono);
  font-size: 10px;
  white-space: pre-wrap;
  word-break: break-all;
  color: var(--text-secondary);
}

.error-bar {
  color: var(--accent-red);
  font-family: var(--font-mono);
  font-size: 10px;
  white-space: pre-wrap;
  word-break: break-all;
}

.label {
  font-size: 10px;
  color: var(--text-tertiary);
  margin-top: 4px;
  margin-bottom: 2px;
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
