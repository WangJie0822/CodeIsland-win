<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";

interface MenuItemDef {
  label: string;
  viewLabel: string;
}

const items: MenuItemDef[] = [
  { label: "偏好设置", viewLabel: "settings" },
  { label: "伙伴", viewLabel: "buddy" },
  { label: "用量报告", viewLabel: "usage" },
  { label: "启动预设", viewLabel: "presets" },
  { label: "调整刘海位置", viewLabel: "notch-live-edit" },
];

const emit = defineEmits<{
  "open-view": [label: string];
  close: [];
}>();

const rootRef = ref<HTMLDivElement | null>(null);

function onItemClick(viewLabel: string): void {
  emit("open-view", viewLabel);
}

function onDocClick(e: MouseEvent): void {
  if (!rootRef.value) return;
  if (!rootRef.value.contains(e.target as Node)) {
    emit("close");
  }
}

onMounted(() => {
  setTimeout(() => {
    document.addEventListener("click", onDocClick, true);
  }, 0);
});

onUnmounted(() => {
  document.removeEventListener("click", onDocClick, true);
});
</script>

<template>
  <Teleport to="body">
    <div ref="rootRef" class="notch-popover">
      <button
        v-for="item in items"
        :key="item.viewLabel"
        class="notch-popover-item"
        :data-popover-item="item.viewLabel"
        type="button"
        @click="onItemClick(item.viewLabel)"
      >
        {{ item.label }}
      </button>
    </div>
  </Teleport>
</template>

<style scoped>
.notch-popover {
  position: fixed;
  top: 40px;
  right: 12px;
  min-width: 180px;
  background: var(--bg-notch, rgba(20, 20, 24, 0.95));
  border: 1px solid rgba(255, 255, 255, 0.12);
  border-radius: 8px;
  padding: 4px 0;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
  z-index: 1000;
  display: flex;
  flex-direction: column;
  font-family: var(--font-sans);
  font-size: 13px;
}

.notch-popover-item {
  background: transparent;
  border: none;
  color: var(--text-primary, #fff);
  padding: 8px 14px;
  text-align: left;
  cursor: pointer;
  transition: background 100ms;
}

.notch-popover-item:hover {
  background: rgba(255, 255, 255, 0.08);
}
</style>
