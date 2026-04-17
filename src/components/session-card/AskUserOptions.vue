<script setup lang="ts">
/**
 * AskUserOptions — AskUserQuestion 工具的多选项审批区
 * 每个选项渲染为独立按钮，点击后发送到终端。
 */
import { computed } from 'vue';
import { useSessionsStore } from '@/stores/sessions';

const props = defineProps<{
  sessionId: string;
  toolInput: unknown;
}>();

const sessions = useSessionsStore();

/** 从 tool_input.options 提取选项列表 */
const options = computed<string[]>(() => {
  if (!props.toolInput) return [];
  const raw = (props.toolInput as { options?: unknown }).options;
  return Array.isArray(raw) ? (raw as string[]) : [];
});

/** question 提示文本（若存在） */
const question = computed<string>(() => {
  if (!props.toolInput) return '';
  return String((props.toolInput as { question?: unknown }).question ?? '');
});

async function onChoose(option: string, e: Event) {
  e.stopPropagation();
  await sessions.sendToTerminal(props.sessionId, option);
}
</script>

<template>
  <div v-if="options.length" class="auo" role="group" aria-label="选择选项">
    <p v-if="question" class="auo__question">{{ question }}</p>
    <div class="auo__options">
      <button
        v-for="option in options"
        :key="option"
        class="auo__btn"
        @click="onChoose(option, $event)"
      >
        {{ option }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.auo {
  padding: 4px 8px 6px 48px;
}

.auo__question {
  font-family: var(--font-sans);
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: 6px;
  line-height: 1.4;
}

.auo__options {
  display: flex;
  flex-wrap: wrap;
  gap: 5px;
}

.auo__btn {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border: 1px solid rgba(33, 150, 243, 0.4);
  border-radius: var(--radius-sm);
  background: rgba(33, 150, 243, 0.1);
  color: var(--accent-blue);
  font-family: var(--font-sans);
  font-size: 12px;
  cursor: pointer;
  transition: background var(--dur-fast), border-color var(--dur-fast);
  user-select: none;
}

.auo__btn:hover {
  background: rgba(33, 150, 243, 0.22);
  border-color: var(--accent-blue);
}

.auo__btn:active {
  transform: scale(0.97);
}
</style>
