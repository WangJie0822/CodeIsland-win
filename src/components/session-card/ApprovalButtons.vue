<script setup lang="ts">
import { useSessionsStore } from "@/stores/sessions";

const props = defineProps<{ sessionId: string }>();
const sessions = useSessionsStore();

async function onAllow(e: Event) {
  e.stopPropagation();
  await sessions.approve(props.sessionId);
}

async function onDeny(e: Event) {
  e.stopPropagation();
  await sessions.deny(props.sessionId);
}
</script>

<template>
  <div class="approval-row">
    <button class="btn btn-allow" @click="onAllow">Allow</button>
    <button class="btn btn-deny" @click="onDeny">Deny</button>
  </div>
</template>
