<script setup lang="ts">
/**
 * ApprovalButtons — Allow / Deny 审批按钮
 * 支持键盘快捷键：Enter → Allow，Esc → Deny
 */
import { onMounted, onUnmounted } from 'vue';
import { useSessionsStore } from '@/stores/sessions';

const props = defineProps<{ sessionId: string }>();
const sessions = useSessionsStore();

async function onAllow(e?: Event) {
  e?.stopPropagation();
  await sessions.approve(props.sessionId);
}

async function onDeny(e?: Event) {
  e?.stopPropagation();
  await sessions.deny(props.sessionId);
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') {
    e.preventDefault();
    onAllow();
  } else if (e.key === 'Escape') {
    e.preventDefault();
    onDeny();
  }
}

onMounted(() => {
  window.addEventListener('keydown', onKeydown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', onKeydown);
});
</script>

<template>
  <div class="approval-row" role="group" aria-label="审批操作">
    <button
      class="btn btn-allow"
      title="批准 (Enter)"
      @click="onAllow($event)"
    >Allow</button>
    <button
      class="btn btn-deny"
      title="拒绝 (Esc)"
      @click="onDeny($event)"
    >Deny</button>
    <span class="approval-row__hint">Enter / Esc</span>
  </div>
</template>

<style scoped>
.approval-row {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 8px 6px 48px; /* 左对齐到内容区 */
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 4px 12px;
  border: none;
  border-radius: var(--radius-sm);
  font-family: var(--font-sans);
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: opacity var(--dur-fast), transform var(--dur-fast);
  user-select: none;
}

.btn:active {
  transform: scale(0.96);
}

.btn-allow {
  background: var(--accent-green);
  color: #fff;
}

.btn-allow:hover {
  opacity: 0.85;
}

.btn-deny {
  background: var(--accent-red);
  color: #fff;
}

.btn-deny:hover {
  opacity: 0.85;
}

.approval-row__hint {
  font-family: var(--font-mono);
  font-size: 10px;
  color: var(--text-tertiary);
  margin-left: 2px;
}
</style>
