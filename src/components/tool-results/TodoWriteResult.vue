<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  toolInput: Record<string, unknown>;
  toolOutput?: unknown;
  toolStatus?: "pending" | "running" | "success" | "error";
}>();

const statusClass = computed(() => props.toolStatus ?? "pending");

interface TodoItem {
  id?: string;
  content?: string;
  status?: "completed" | "in_progress" | "pending" | string;
  priority?: string;
}

const todos = computed((): TodoItem[] => {
  const raw = props.toolInput.todos ?? props.toolInput.items ?? props.toolInput.todo_list;
  if (Array.isArray(raw)) return raw as TodoItem[];
  return [];
});

const completed = computed(() => todos.value.filter(t => t.status === "completed").length);
const inProgress = computed(() => todos.value.filter(t => t.status === "in_progress").length);
const pending = computed(() => todos.value.filter(t => t.status === "pending" || !t.status).length);

function statusLabel(status?: string): string {
  switch (status) {
    case "completed":  return "✓";
    case "in_progress": return "→";
    default:           return "·";
  }
}

function statusColor(status?: string): string {
  switch (status) {
    case "completed":   return "var(--accent-green)";
    case "in_progress": return "var(--accent-blue)";
    default:            return "var(--text-tertiary)";
  }
}
</script>

<template>
  <div class="tool-result todo-result" :class="statusClass">
    <template v-if="toolStatus === 'pending'">
      <div class="skeleton-line" style="width: 65%" />
      <div class="skeleton-line" style="width: 45%" />
    </template>
    <template v-else>
      <div class="result-header">
        <span class="badge">TodoWrite</span>
        <span class="stat done">{{ completed }} 完成</span>
        <span class="stat wip">{{ inProgress }} 进行</span>
        <span class="stat todo">{{ pending }} 待处理</span>
        <span v-if="toolStatus === 'error'" class="error-badge">ERROR</span>
      </div>
      <div v-if="toolStatus === 'error'" class="error-bar">
        {{ typeof toolOutput === "string" ? toolOutput : JSON.stringify(toolOutput) }}
      </div>
      <ul v-else class="todo-list">
        <li
          v-for="(item, idx) in todos"
          :key="item.id ?? idx"
          class="todo-item"
          :style="{ color: statusColor(item.status) }"
        >
          <span class="todo-marker">{{ statusLabel(item.status) }}</span>
          <span class="todo-content">{{ item.content ?? JSON.stringify(item) }}</span>
        </li>
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
  gap: 6px;
  margin-bottom: 4px;
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

.stat {
  font-size: 10px;
  flex-shrink: 0;
}
.stat.done { color: var(--accent-green); }
.stat.wip  { color: var(--accent-blue); }
.stat.todo { color: var(--text-tertiary); }

.error-badge { font-size: 10px; color: var(--accent-red); font-weight: 600; }

.error-bar {
  color: var(--accent-red);
  font-family: var(--font-mono);
  font-size: 10px;
  white-space: pre-wrap;
  word-break: break-all;
  margin-top: 4px;
}

.todo-list { list-style: none; }

.todo-item {
  display: flex;
  align-items: flex-start;
  gap: 5px;
  padding: 1px 0;
  font-size: 10px;
}

.todo-marker {
  flex-shrink: 0;
  width: 10px;
  font-family: var(--font-mono);
}

.todo-content {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
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
