<script setup lang="ts">
import { computed, ref } from "vue";

const props = defineProps<{
  toolInput: Record<string, unknown>;
  toolOutput?: unknown;
  toolStatus?: "pending" | "running" | "success" | "error";
}>();

const expanded = ref(false);
const statusClass = computed(() => props.toolStatus ?? "pending");

const command = computed(() => {
  const cmd = props.toolInput.command;
  return typeof cmd === "string" ? cmd : "";
});

/** 截断命令到单行 80 字符 */
const shortCommand = computed(() => {
  const cmd = command.value.split("\n")[0];
  return cmd.length > 80 ? cmd.slice(0, 80) + "…" : cmd;
});

interface BashOutput {
  stdout?: string;
  stderr?: string;
  exit_code?: number;
}

function parseBashOutput(): BashOutput {
  const out = props.toolOutput;
  if (out == null) return {};
  if (typeof out === "object" && out !== null) return out as BashOutput;
  if (typeof out === "string") {
    try { return JSON.parse(out); } catch { return { stdout: out }; }
  }
  return {};
}

const parsed = computed(() => parseBashOutput());
const exitCode = computed(() => parsed.value.exit_code ?? null);
const stdout = computed(() => {
  const s = parsed.value.stdout ?? "";
  return s.split("\n").slice(0, 10);
});
const hasMoreStdout = computed(() => {
  const s = parsed.value.stdout ?? "";
  return s.split("\n").length > 10;
});
const stderr = computed(() => parsed.value.stderr ?? "");
</script>

<template>
  <div class="tool-result bash-result" :class="statusClass">
    <template v-if="toolStatus === 'pending'">
      <div class="skeleton-line" style="width: 80%" />
      <div class="skeleton-line" style="width: 50%" />
    </template>
    <template v-else>
      <div class="result-header">
        <span class="badge">Bash</span>
        <code class="cmd-text">{{ shortCommand }}</code>
        <span
          v-if="exitCode != null"
          class="exit-code"
          :class="exitCode === 0 ? 'ok' : 'fail'"
        >exit {{ exitCode }}</span>
        <button class="expand-btn" @click="expanded = !expanded">
          {{ expanded ? "收起" : "展开" }}
        </button>
      </div>
      <div v-if="toolStatus === 'error'" class="error-bar">
        {{ stderr || (typeof toolOutput === "string" ? toolOutput : JSON.stringify(toolOutput)) }}
      </div>
      <template v-else-if="stdout.length > 0">
        <pre class="mono-pre">{{ stdout.join("\n") }}</pre>
        <div v-if="hasMoreStdout && !expanded" class="more-hint">…stdout 已截断，前 10 行</div>
        <pre v-if="expanded && parsed.stdout" class="mono-pre">{{ parsed.stdout }}</pre>
        <pre v-if="expanded && stderr" class="mono-pre stderr">{{ stderr }}</pre>
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

.cmd-text {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.exit-code {
  font-family: var(--font-mono);
  font-size: 10px;
  flex-shrink: 0;
}
.exit-code.ok   { color: var(--accent-green); }
.exit-code.fail { color: var(--accent-red); }

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

.mono-pre {
  font-family: var(--font-mono);
  font-size: 10px;
  white-space: pre-wrap;
  word-break: break-all;
  color: var(--text-secondary);
  margin-top: 4px;
}

.mono-pre.stderr {
  color: var(--accent-red);
  opacity: 0.85;
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
