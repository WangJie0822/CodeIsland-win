<script setup lang="ts">
import { computed } from "vue";
import { useNotchStore } from "@/stores/notch";

const notch = useNotchStore();

const rootStyle = computed(() => ({
  width: `${notch.geometry.width}px`,
  height: `${notch.geometry.height}px`,
}));

const clipId = "notch-clip";
const shadowId = "notch-shadow";
</script>

<template>
  <div class="notch-root" :style="rootStyle">
    <svg class="notch-svg" :viewBox="notch.viewBox" :width="notch.geometry.width" :height="notch.geometry.height">
      <defs>
        <clipPath :id="clipId">
          <path :d="notch.path" />
        </clipPath>
        <filter :id="shadowId" x="-20%" y="-20%" width="140%" height="140%">
          <feDropShadow dx="0" dy="4" stdDeviation="12" flood-opacity="0.4" />
        </filter>
      </defs>
      <path :d="notch.path" fill="rgba(0,0,0,0.85)" :filter="`url(#${shadowId})`" />
    </svg>
    <div class="notch-content" :style="{ clipPath: `url(#${clipId})` }">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.notch-root {
  position: relative;
  pointer-events: none;
}
.notch-svg {
  position: absolute;
  top: 0;
  left: 0;
  pointer-events: none;
  overflow: visible;
}
.notch-content {
  position: absolute;
  inset: 0;
  pointer-events: auto;
}
</style>
