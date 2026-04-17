<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useUsageStore } from "@/stores/usage";
import type { UsageBucket } from "@/stores/usage";

const store = useUsageStore();

onMounted(() => {
  void store.load();
});

// ── 颜色映射 ────────────────────────────────────────────────────────────────
function barColor(percent: number): string {
  if (percent > 0.8) return "var(--accent-red)";
  if (percent > 0.5) return "var(--accent-yellow)";
  return "var(--accent-green)";
}

// ── 剩余时间格式化 ──────────────────────────────────────────────────────────
function formatRemaining(resetsAt: number | null): string {
  if (resetsAt == null) return "未知";
  const diffMs = resetsAt * 1000 - Date.now();
  if (diffMs <= 0) return "已过期";
  const totalMinutes = Math.floor(diffMs / 60_000);
  const hours = Math.floor(totalMinutes / 60);
  const minutes = totalMinutes % 60;
  if (hours > 0) return `${hours}h ${minutes}m`;
  return `${minutes}m`;
}

// ── 进度条宽度（clamp 在 0~100%） ──────────────────────────────────────────
function barWidth(percent: number): string {
  return `${Math.min(1, Math.max(0, percent)) * 100}%`;
}

// ── 超额标志 ────────────────────────────────────────────────────────────────
function isOver(bucket: UsageBucket): boolean {
  return bucket.percent > 1.0;
}

const has5h = computed(() => store.report != null && store.report.window_5h.tokens > 0);
const has7d = computed(() => store.report != null && store.report.window_7d.tokens > 0);
const hasAnyData = computed(() => has5h.value || has7d.value);
</script>

<template>
  <div class="daily-report-card" data-testid="daily-report-card">
    <!-- 标题栏 -->
    <div class="card-header">
      <span class="card-title">Daily Report</span>
      <button
        class="btn-refresh"
        title="刷新用量"
        data-testid="btn-refresh"
        @click="store.refresh()"
      >
        ↻
      </button>
    </div>

    <!-- 无数据态 -->
    <div v-if="!store.report || !hasAnyData" class="empty-state" data-testid="empty-state">
      No usage data yet
    </div>

    <!-- 数据态 -->
    <template v-else>
      <!-- 5h 窗口 -->
      <div class="bucket-row" data-testid="bucket-5h">
        <div class="bucket-label">
          <span class="window-label">5h 窗口</span>
          <span class="token-count">{{ store.report!.window_5h.tokens.toLocaleString() }} tokens</span>
          <span v-if="isOver(store.report!.window_5h)" class="over-badge" data-testid="over-badge-5h">超额</span>
        </div>
        <div class="progress-track">
          <div
            class="progress-fill"
            :class="{ 'is-over': isOver(store.report!.window_5h), 'is-warn': store.report!.window_5h.percent > 0.8 }"
            :style="{ width: barWidth(store.report!.window_5h.percent), background: barColor(store.report!.window_5h.percent) }"
            data-testid="bar-5h"
          />
        </div>
        <div class="bucket-meta">
          <span class="percent-label">{{ (store.report!.window_5h.percent * 100).toFixed(1) }}%</span>
          <span class="resets-label">重置：{{ formatRemaining(store.report!.window_5h.resets_at) }}</span>
        </div>
      </div>

      <!-- 7d 窗口 -->
      <div class="bucket-row" data-testid="bucket-7d">
        <div class="bucket-label">
          <span class="window-label">7d 窗口</span>
          <span class="token-count">{{ store.report!.window_7d.tokens.toLocaleString() }} tokens</span>
          <span v-if="isOver(store.report!.window_7d)" class="over-badge" data-testid="over-badge-7d">超额</span>
        </div>
        <div class="progress-track">
          <div
            class="progress-fill"
            :class="{ 'is-over': isOver(store.report!.window_7d), 'is-warn': store.report!.window_7d.percent > 0.8 }"
            :style="{ width: barWidth(store.report!.window_7d.percent), background: barColor(store.report!.window_7d.percent) }"
            data-testid="bar-7d"
          />
        </div>
        <div class="bucket-meta">
          <span class="percent-label">{{ (store.report!.window_7d.percent * 100).toFixed(1) }}%</span>
          <span class="resets-label">重置：{{ formatRemaining(store.report!.window_7d.resets_at) }}</span>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.daily-report-card {
  background: var(--bg-secondary);
  border-radius: var(--radius-sm);
  padding: 12px 16px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  min-width: 240px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.card-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}

.btn-refresh {
  background: none;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 16px;
  padding: 0 4px;
  line-height: 1;
  transition: color var(--dur-fast) var(--ease-notch);
}

.btn-refresh:hover {
  color: var(--text-primary);
}

.empty-state {
  color: var(--text-tertiary);
  font-size: 12px;
  text-align: center;
  padding: 8px 0;
}

/* ── 桶行 ───────────────────────────────────────────────────────────────── */
.bucket-row {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.bucket-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
}

.window-label {
  font-weight: 600;
  color: var(--text-secondary);
}

.token-count {
  color: var(--text-tertiary);
}

.over-badge {
  font-size: 10px;
  background: var(--accent-red);
  color: #fff;
  border-radius: 3px;
  padding: 1px 4px;
}

/* ── 进度条 ─────────────────────────────────────────────────────────────── */
.progress-track {
  width: 100%;
  height: 6px;
  background: rgba(255, 255, 255, 0.08);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  border-radius: 3px;
  transition: width 300ms var(--ease-notch), background 300ms var(--ease-notch);
}

/* >80% 时闪烁动画 */
@keyframes usage-blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.progress-fill.is-warn {
  animation: usage-blink 1.2s var(--ease-notch) infinite;
}

/* ── 桶元信息 ────────────────────────────────────────────────────────────── */
.bucket-meta {
  display: flex;
  justify-content: space-between;
  font-size: 10px;
  color: var(--text-tertiary);
}

.percent-label {
  font-variant-numeric: tabular-nums;
}

.resets-label {
  font-variant-numeric: tabular-nums;
}
</style>
