<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  toolInput: Record<string, unknown>;
  toolOutput?: unknown;
  toolStatus?: "pending" | "running" | "success" | "error";
}>();

const statusClass = computed(() => props.toolStatus ?? "pending");

function abbreviatePath(p: string): string {
  if (!p) return "";
  const parts = p.replace(/\\/g, "/").split("/").filter(Boolean);
  if (parts.length <= 2) return p;
  return "…/" + parts.slice(-2).join("/");
}

const filePath = computed(() => String(props.toolInput.file_path ?? props.toolInput.path ?? ""));
const shortPath = computed(() => abbreviatePath(filePath.value));

/** 估算字节数：content 字段的 UTF-8 字节数 */
const byteCount = computed(() => {
  const content = props.toolInput.content;
  if (content == null) return null;
  const str = typeof content === "string" ? content : JSON.stringify(content);
  return new TextEncoder().encode(str).length;
});

function formatBytes(n: number): string {
  if (n < 1024) return `${n} B`;
  if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`;
  return `${(n / 1024 / 1024).toFixed(1)} MB`;
}
</script>

<template>
  <div class="tool-result write-result" :class="statusClass">
    <template v-if="toolStatus === 'pending'">
      <div class="skeleton-line" style="width: 70%" />
      <div class="skeleton-line" style="width: 35%" />
    </template>
    <template v-else>
      <div class="result-header">
        <span class="badge">Write</span>
        <span class="new-badge">NEW</span>
        <span class="path-text" :title="filePath">{{ shortPath }}</span>
        <span v-if="byteCount != null" class="size-text">{{ formatBytes(byteCount) }}</span>
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

.new-badge {
  font-size: 9px;
  font-weight: 700;
  color: var(--accent-green);
  background: rgba(76,175,80,0.15);
  padding: 1px 4px;
  border-radius: 3px;
  flex-shrink: 0;
  letter-spacing: 0.5px;
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

.size-text {
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
