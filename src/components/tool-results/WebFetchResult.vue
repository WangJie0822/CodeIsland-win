<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  toolInput: Record<string, unknown>;
  toolOutput?: unknown;
  toolStatus?: "pending" | "running" | "success" | "error";
}>();

const statusClass = computed(() => props.toolStatus ?? "pending");

const url = computed(() => String(props.toolInput.url ?? props.toolInput.uri ?? ""));

/** 缩短 URL：保留 scheme + host + 最后路径段 */
function shortenUrl(raw: string): string {
  if (!raw) return "";
  try {
    const u = new URL(raw);
    const pathParts = u.pathname.split("/").filter(Boolean);
    const last = pathParts.length > 0 ? "/" + pathParts[pathParts.length - 1] : "/";
    const prefix = `${u.protocol}//${u.host}`;
    return pathParts.length > 1 ? `${prefix}/…${last}` : `${prefix}${last}`;
  } catch {
    // 非标准 URL 直接截断
    return raw.length > 60 ? raw.slice(0, 60) + "…" : raw;
  }
}

const shortUrl = computed(() => shortenUrl(url.value));

const byteCount = computed(() => {
  const out = props.toolOutput;
  if (out == null) return null;
  const str = typeof out === "string" ? out : JSON.stringify(out);
  return new TextEncoder().encode(str).length;
});

function formatBytes(n: number): string {
  if (n < 1024) return `${n} B`;
  if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`;
  return `${(n / 1024 / 1024).toFixed(1)} MB`;
}
</script>

<template>
  <div class="tool-result webfetch-result" :class="statusClass">
    <template v-if="toolStatus === 'pending'">
      <div class="skeleton-line" style="width: 80%" />
      <div class="skeleton-line" style="width: 25%" />
    </template>
    <template v-else>
      <div class="result-header">
        <span class="badge">WebFetch</span>
        <span class="url-text" :title="url">{{ shortUrl }}</span>
        <span v-if="byteCount != null && toolStatus === 'success'" class="size-text">
          {{ formatBytes(byteCount) }}
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

.url-text {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--accent-blue);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
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
