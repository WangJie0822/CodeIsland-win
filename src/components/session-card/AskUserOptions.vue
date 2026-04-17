<script setup lang="ts">
import { computed } from "vue";
import { useSessionsStore } from "@/stores/sessions";

const props = defineProps<{
  sessionId: string;
  toolInput: Record<string, unknown> | null;
}>();

const sessions = useSessionsStore();

const options = computed<string[]>(() => {
  if (!props.toolInput) return [];
  const raw = (props.toolInput as { options?: unknown }).options;
  return Array.isArray(raw) ? (raw as string[]) : [];
});

async function onChoose(option: string, e: Event) {
  e.stopPropagation();
  await sessions.sendToTerminal(props.sessionId, option);
}
</script>

<template>
  <div v-if="options.length" class="askuser-options">
    <button
      v-for="option in options"
      :key="option"
      class="btn-option"
      @click="onChoose(option, $event)"
    >
      {{ option }}
    </button>
  </div>
</template>
