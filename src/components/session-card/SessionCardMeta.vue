<script setup lang="ts">
/**
 * SessionCardMeta — 会话摘要行
 * 显示 last_message 内容（AI 摘要 / 状态文本）
 */
import type { SessionSummary } from '@/types/generated';

const props = defineProps<{
  session: SessionSummary;
}>();

/**
 * 根据 phase 计算摘要文本：
 * - processing     → "Processing (tool_name)"
 * - waitingForApproval → "等待审批: tool_name"
 * - waitingForInput → last_message ?? "等待输入"
 * - compacting     → "压缩上下文..."
 * - ended          → last_message ?? "已结束"
 * - idle (default) → last_message ?? ""
 */
function computeSummary(session: SessionSummary): string {
  switch (session.phase) {
    case 'processing':
      return session.tool_name
        ? `Processing (${session.tool_name})`
        : 'Processing…';
    case 'waitingForApproval':
      return `等待审批: ${session.tool_name ?? 'tool'}`;
    case 'waitingForInput':
      return session.last_message ?? '等待输入…';
    case 'compacting':
      return '压缩上下文…';
    case 'ended':
      return session.last_message ?? '已结束';
    default:
      return session.last_message ?? '';
  }
}
</script>

<template>
  <div v-if="computeSummary(session)" class="scm">
    <p class="scm__text">{{ computeSummary(session) }}</p>
  </div>
</template>

<style scoped>
.scm {
  padding: 2px 8px 4px 48px; /* 48px = 8px padding + 32px avatar + 8px gap */
}

.scm__text {
  font-family: var(--font-sans);
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.4;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
