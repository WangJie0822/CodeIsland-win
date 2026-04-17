<script setup lang="ts">
/**
 * SessionCardBody — 工具调用结果区
 * Stage 1 使用 GenericResult 占位；B agent (tool-results) 合并后按 tool_name 路由替换。
 *
 * Stage 2 待办：将 GenericResult 替换为 resolveToolView(toolName) 动态路由组件。
 */
import type { SessionSummary } from '@/types/generated';

const props = defineProps<{
  session: SessionSummary;
}>();

/** tool_output —— Stage 2 SessionSummary 字段追加后可用，目前 optional chaining fallback */
const toolOutput = (() => (props.session as any).tool_output ?? null)();

/** tool_status —— pending / running / success / error，Stage 2 追加 */
const toolStatus: string = (props.session as any).tool_status ?? 'pending';

/** 是否有工具调用数据可展示 */
const hasToolData = props.session.tool_name != null;

/** 工具调用 JSON 预览（最多 500 字符） */
function jsonPreview(value: unknown): string {
  if (value == null) return '';
  const str = typeof value === 'string' ? value : JSON.stringify(value, null, 2);
  return str.length > 500 ? str.slice(0, 500) + '…' : str;
}

const outputPreview = toolOutput != null
  ? jsonPreview(toolOutput)
  : props.session.tool_input != null
    ? jsonPreview(props.session.tool_input)
    : '';
</script>

<template>
  <div v-if="hasToolData" class="scb">
    <!-- tool_name + status 徽章 -->
    <div class="scb__header">
      <span class="scb__tool-name">{{ session.tool_name }}</span>
      <span
        class="scb__status-badge"
        :class="`scb__status-badge--${toolStatus}`"
      >{{ toolStatus }}</span>
    </div>

    <!-- 输出预览（JSON 兜底） -->
    <pre v-if="outputPreview" class="scb__preview">{{ outputPreview }}</pre>
  </div>
</template>

<style scoped>
.scb {
  margin: 4px 8px;
  padding: 6px 8px;
  background: var(--bg-secondary);
  border-radius: var(--radius-sm);
  border: 1px solid rgba(255, 255, 255, 0.06);
  max-height: 160px;
  overflow-y: auto;
}

.scb__header {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 4px;
}

.scb__tool-name {
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--text-primary);
  font-weight: 600;
}

.scb__status-badge {
  font-family: var(--font-mono);
  font-size: 9px;
  padding: 1px 5px;
  border-radius: 4px;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.scb__status-badge--pending {
  background: rgba(255, 152, 0, 0.15);
  color: var(--accent-yellow);
}

.scb__status-badge--running {
  background: rgba(33, 150, 243, 0.15);
  color: var(--accent-blue);
}

.scb__status-badge--success {
  background: rgba(76, 175, 80, 0.15);
  color: var(--accent-green);
}

.scb__status-badge--error {
  background: rgba(244, 67, 54, 0.15);
  color: var(--accent-red);
}

.scb__preview {
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--text-secondary);
  white-space: pre-wrap;
  word-break: break-all;
  line-height: 1.5;
  margin: 0;
}
</style>
