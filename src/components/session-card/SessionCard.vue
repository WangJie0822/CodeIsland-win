<script setup lang="ts">
/**
 * SessionCard — 会话卡片容器
 * 组合 Header + Meta + Body + 审批区，负责容器布局与点击行为。
 */
import { computed } from 'vue';
import type { SessionSummary } from '@/types/generated';
import { useSessionsStore } from '@/stores/sessions';
import SessionCardHeader from './SessionCardHeader.vue';
import SessionCardMeta from './SessionCardMeta.vue';
import SessionCardBody from './SessionCardBody.vue';
import ApprovalButtons from './ApprovalButtons.vue';
import AskUserOptions from './AskUserOptions.vue';

const props = defineProps<{ session: SessionSummary }>();
const sessions = useSessionsStore();

/** 是否为 AskUserQuestion 工具调用（需渲染多选项） */
const isAskUser = computed(
  () =>
    props.session.tool_name === 'AskUserQuestion' ||
    props.session.tool_name === 'AskUser',
);

/** 是否在等待权限审批（且非 AskUser 类型，AskUser 走独立选项区） */
const isWaitingApproval = computed(
  () => props.session.phase === 'waitingForApproval' && !isAskUser.value,
);

/** 是否有工具调用数据可展示（Body 区） */
const hasToolBody = computed(
  () => props.session.tool_name != null && props.session.phase !== 'idle',
);

async function onCardClick() {
  await sessions.sendToTerminal(props.session.session_id, '');
}

function onClose() {
  // Stage 2 接入 close_session 命令
  console.warn('[SessionCard] close 暂未实现，待 Stage 2 追加 close_session 命令');
}
</script>

<template>
  <div
    class="session-card"
    :class="{
      'session-card--attention': session.needs_attention,
      [`session-card--${session.phase}`]: true,
    }"
    @click="onCardClick"
  >
    <!-- 头部：头像 + 项目名 + launcher + 时长 + 关闭 -->
    <SessionCardHeader :session="session" @close="onClose" />

    <!-- 摘要行：last_message / 状态描述 -->
    <SessionCardMeta :session="session" />

    <!-- 工具调用结果（Body）：phase 为非 idle 且有 tool_name 时展示 -->
    <SessionCardBody v-if="hasToolBody" :session="session" />

    <!-- 审批按钮（waitingForApproval 且非 AskUser） -->
    <ApprovalButtons
      v-if="isWaitingApproval"
      :session-id="session.session_id"
    />

    <!-- AskUser 多选项区 -->
    <AskUserOptions
      v-if="isAskUser"
      :session-id="session.session_id"
      :tool-input="session.tool_input"
    />
  </div>
</template>

<style scoped>
.session-card {
  background: var(--bg-secondary);
  border-radius: var(--radius-sm);
  border: 1px solid rgba(255, 255, 255, 0.06);
  cursor: pointer;
  transition: border-color var(--dur-fast), box-shadow var(--dur-fast);
  overflow: hidden;
}

.session-card:hover {
  border-color: rgba(255, 255, 255, 0.12);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

/* 需要注意（等待审批）时高亮边框 */
.session-card--attention {
  border-color: var(--accent-yellow);
  box-shadow: 0 0 0 1px rgba(255, 152, 0, 0.3);
}

/* waitingForApproval 加强提示 */
.session-card--waitingForApproval {
  border-color: var(--accent-yellow);
}

/* waitingForInput 蓝色提示 */
.session-card--waitingForInput {
  border-color: var(--accent-blue);
}
</style>
