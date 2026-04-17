<script setup lang="ts">
import { computed } from "vue";
import type { SessionSummary } from "@/types/generated";
import { useSessionsStore } from "@/stores/sessions";
import StatusDot from "@/components/common/StatusDot.vue";
import ApprovalButtons from "./ApprovalButtons.vue";
import AskUserOptions from "./AskUserOptions.vue";

const props = defineProps<{ session: SessionSummary }>();
const sessions = useSessionsStore();

const dotStatus = computed<"idle" | "processing" | "attention">(() => {
  if (props.session.needs_attention) return "attention";
  if (props.session.phase === "processing") return "processing";
  return "idle";
});

const statusText = computed(() => {
  const s = props.session;
  switch (s.phase) {
    case "processing":
      return `Processing${s.tool_name ? ` (${s.tool_name})` : ""}`;
    case "waitingForApproval":
      return `Awaiting approval: ${s.tool_name ?? "tool"}`;
    case "waitingForInput":
      return s.last_message ?? "Waiting for input";
    case "compacting":
      return "Compacting context...";
    case "idle":
      return s.last_message ?? "Idle";
    default:
      return s.phase;
  }
});

const toolInfoText = computed(() => {
  const s = props.session;
  if (!s.tool_name || !s.tool_input || s.phase !== "waitingForApproval") return null;
  const parts = [`Tool: ${s.tool_name}`];
  for (const [key, value] of Object.entries(s.tool_input)) {
    const str = typeof value === "string" ? value : JSON.stringify(value);
    parts.push(`${key}: ${str.length > 100 ? str.slice(0, 100) + "..." : str}`);
  }
  return parts.join("\n");
});

const isAskUser = computed(
  () =>
    props.session.tool_name === "AskUserQuestion" ||
    props.session.tool_name === "AskUser",
);

async function onCardClick() {
  await sessions.sendToTerminal(props.session.session_id, "");
}
</script>

<template>
  <div class="session-card" @click="onCardClick">
    <div class="session-header">
      <StatusDot :status="dotStatus" />
      <span class="session-name">{{ session.project_name }}</span>
      <span class="session-cwd">{{ session.cwd }}</span>
    </div>
    <div :class="['session-status', { attention: session.needs_attention }]">
      {{ statusText }}
    </div>
    <div v-if="toolInfoText" class="tool-info">{{ toolInfoText }}</div>
    <ApprovalButtons
      v-if="session.phase === 'waitingForApproval'"
      :session-id="session.session_id"
    />
    <AskUserOptions
      v-if="isAskUser"
      :session-id="session.session_id"
      :tool-input="session.tool_input"
    />
  </div>
</template>
