<script setup lang="ts">
import { computed, onMounted, onUnmounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useSessionsStore } from "@/stores/sessions";
import { useNotchStore } from "@/stores/notch";
import { onSessionsUpdated } from "@/lib/tauri";
import StatusDot from "@/components/common/StatusDot.vue";
import SessionCard from "@/components/session-card/SessionCard.vue";
import NotchShape from "@/components/notch/NotchShape.vue";

const sessions = useSessionsStore();
const notch = useNotchStore();
let collapseTimer: number | null = null;
let unlisten: (() => void) | null = null;

const sessionCount = computed(() => sessions.list.length);
const countLabel = computed(
  () => `${sessionCount.value} session${sessionCount.value !== 1 ? "s" : ""}`,
);

function onEnter() {
  if (collapseTimer !== null) {
    window.clearTimeout(collapseTimer);
    collapseTimer = null;
  }
  void notch.animateExpanded(true);
}

function onLeave() {
  collapseTimer = window.setTimeout(() => {
    void notch.animateExpanded(false);
  }, 300);
}

async function onCloseClick(e: Event) {
  e.stopPropagation();
  await getCurrentWindow().hide();
}

onMounted(async () => {
  notch.setExpanded(false);
  await sessions.refresh();
  unlisten = await onSessionsUpdated(() => {
    void sessions.refresh();
  });
});

onUnmounted(() => {
  if (unlisten) unlisten();
  if (collapseTimer !== null) window.clearTimeout(collapseTimer);
});
</script>

<template>
  <div class="notch-wrapper" @mouseenter="onEnter" @mouseleave="onLeave">
    <NotchShape>
      <div v-if="!notch.expanded" class="island-collapsed" data-tauri-drag-region>
        <StatusDot :status="sessions.overallStatus" />
        <span class="session-count">{{ countLabel }}</span>
        <span class="status-label">{{ sessions.overallStatus }}</span>
      </div>
      <div v-else class="island-expanded">
        <div class="expanded-header" data-tauri-drag-region>
          <span class="header-title">Code Island</span>
          <button class="btn-close" title="隐藏窗口" @click="onCloseClick">×</button>
        </div>
        <div class="session-list">
          <div v-if="sessions.list.length === 0" class="empty-state">
            No active sessions
          </div>
          <SessionCard
            v-for="session in sessions.list"
            :key="session.session_id"
            :session="session"
          />
        </div>
      </div>
    </NotchShape>
  </div>
</template>

<style scoped>
.notch-wrapper {
  position: fixed;
  top: 0;
  left: 50%;
  transform: translateX(-50%);
}
</style>
