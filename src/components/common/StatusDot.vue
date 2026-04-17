<script setup lang="ts">
defineProps<{ status: "idle" | "processing" | "attention" | "success" | "error" }>();
</script>

<template>
  <span class="status-dot" :class="status" />
</template>
