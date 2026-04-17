<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  toolInput: Record<string, unknown>;
  toolOutput?: unknown;
  toolStatus?: "pending" | "running" | "success" | "error";
}>();

const statusClass = computed(() => props.toolStatus ?? "pending");

const agentName = computed(() =>
  String(props.toolInput.agent ?? props.toolInput.name ?? props.toolInput.subagent ?? "subagent"),
);

const description = computed(() => {
  const desc = props.toolInput.description ?? props.toolInput.prompt ?? props.toolInput.task;
  if (!desc) return null;
  const s = String(desc);
  return s.length > 120 ? s.slice(0, 120) + "…" : s;
});

const stateLabel = computed(() => {
  switch (props.toolStatus) {
    case "pending": return "等待中";
    case "running": return "运行中";
    case "success": return "完成";
    case "error":   return "失败";
    default:        return "等待中";
  }
});

const stateColor = computed(() => {
  switch (props.toolStatus) {
    case "running": return "var(--accent-blue)";
    case "success": return "var(--accent-green)";
    case "error":   return "var(--accent-red)";
    default:        return "var(--text-tertiary)";
  }
});
</script>

<template>
  <div class="tool-result task-result" :class="statusClass">
    <template v-if="toolStatus === 'pending'">
      <div class="skeleton-line" style="width: 50%" />
      <div class="skeleton-line" style="width: 70%" />
    </template>
    <template v-else>
      <div class="result-header">
        <span class="badge">Task</span>
        <span class="agent-name">{{ agentName }}</span>
        <span class="state-pill" :style="{ color: stateColor }">{{ stateLabel }}</span>
      </div>
      <div v-if="description" class="desc-text">{{ description }}</div>
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
  gap: 6px;
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

.agent-name {
  font-size: 11px;
  color: var(--text-primary);
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.state-pill {
  font-size: 10px;
  flex-shrink: 0;
  font-weight: 500;
}

.desc-text {
  font-size: 10px;
  color: var(--text-secondary);
  margin-top: 4px;
  line-height: 1.4;
}

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
