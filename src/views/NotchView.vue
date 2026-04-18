<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useSessionsStore } from "@/stores/sessions";
import { useNotchStore } from "@/stores/notch";
import {
  onSessionsUpdated,
  invokeSetIgnoreCursorEvents,
  invokeSetWindowSize,
} from "@/lib/tauri";
import StatusDot from "@/components/common/StatusDot.vue";
import SessionCard from "@/components/session-card/SessionCard.vue";
import NotchShape from "@/components/notch/NotchShape.vue";
import NotchMenuPopover from "@/components/notch/NotchMenuPopover.vue";
import { invokeOpenViewWindow } from "@/lib/tauri";

const sessions = useSessionsStore();
const notch = useNotchStore();
let collapseTimer: number | null = null;
let unlisten: (() => void) | null = null;
let hitTestEl: SVGPathElement | null = null;

const sessionCount = computed(() => sessions.list.length);
const countLabel = computed(
  () => `${sessionCount.value} session${sessionCount.value !== 1 ? "s" : ""}`,
);

const popoverOpen = ref(false);

function togglePopover(): void {
  popoverOpen.value = !popoverOpen.value;
}

async function onOpenView(label: string): Promise<void> {
  popoverOpen.value = false;
  try {
    await invokeOpenViewWindow(label);
  } catch (e) {
    console.warn("[notch] 打开窗口失败:", e);
  }
}

function onPopoverClose(): void {
  popoverOpen.value = false;
}

function hitTest(clientX: number, clientY: number): boolean {
  if (!hitTestEl) return true;
  const rect = hitTestEl.getBoundingClientRect();
  const localX = clientX - rect.left;
  const localY = clientY - rect.top;
  const point = new DOMPoint(localX, localY);
  const svg = hitTestEl.ownerSVGElement;
  if (!svg) return true;
  const ctm = hitTestEl.getScreenCTM();
  if (!ctm) return true;
  const transformed = point.matrixTransform(ctm.inverse());
  return hitTestEl.isPointInFill(transformed);
}

function onGlobalPointerMove(e: PointerEvent) {
  const inside = hitTest(e.clientX, e.clientY);
  void invokeSetIgnoreCursorEvents(!inside).catch(() => {});
}

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

watch(
  () => ({ w: notch.geometry.width, h: notch.geometry.height }),
  async ({ w, h }) => {
    await invokeSetWindowSize(w, h);
  },
);

onMounted(async () => {
  notch.setExpanded(false);
  await invokeSetWindowSize(notch.geometry.width, notch.geometry.height);
  await invokeSetIgnoreCursorEvents(true);

  hitTestEl = document.querySelector<SVGPathElement>(".notch-svg path");
  window.addEventListener("pointermove", onGlobalPointerMove);

  await sessions.refresh();
  unlisten = await onSessionsUpdated(() => {
    void sessions.refresh();
  });
});

onUnmounted(() => {
  if (unlisten) unlisten();
  if (collapseTimer !== null) window.clearTimeout(collapseTimer);
  window.removeEventListener("pointermove", onGlobalPointerMove);
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
          <button
            class="notch-menu-btn"
            data-testid="notch-menu-button"
            type="button"
            aria-label="更多菜单"
            @click.stop="togglePopover"
          >
            ⋯
          </button>
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
    <NotchMenuPopover
      v-if="popoverOpen"
      @open-view="onOpenView"
      @close="onPopoverClose"
    />
  </div>
</template>

<style scoped>
.notch-wrapper {
  position: fixed;
  top: 0;
  left: 50%;
  transform: translateX(-50%);
}

.notch-menu-btn {
  width: 22px;
  height: 22px;
  background: transparent;
  border: none;
  color: var(--text-tertiary, rgba(255, 255, 255, 0.6));
  cursor: pointer;
  font-size: 16px;
  line-height: 1;
  padding: 0;
  border-radius: 4px;
  margin-right: 4px;
}
.notch-menu-btn:hover {
  background: rgba(255, 255, 255, 0.12);
  color: var(--text-primary, #fff);
}
</style>
