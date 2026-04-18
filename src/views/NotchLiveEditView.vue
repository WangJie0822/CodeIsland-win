<script setup lang="ts">
/**
 * NotchLiveEditView — 刘海 Live Edit 模式
 *
 * 提供拖动定位 + 尺寸微调功能。
 * 本视图仅提供 UI；路由挂接由 Stage 2 主会话负责。
 */
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { availableMonitors, type Monitor } from "@tauri-apps/api/window";
import { useDragResize } from "@/composables/useDragResize";
import { EXPANDED } from "@/utils/notch-shape";
import type { WindowPosition } from "@/types/generated";

// ------- 状态 -------
const dragHandleRef = ref<HTMLElement | null>(null);
const dragMode = ref(false);

const posX = ref(0);
const posY = ref(0);
const winW = ref(EXPANDED.width);
const winH = ref(EXPANDED.height);

// 记录进入 live-edit 时的原始位置，用于"复位"
const originX = ref(0);
const originY = ref(0);

const sizeLabel = computed(() => `${Math.round(winW.value)} × ${Math.round(winH.value)}`);

// ------- 拖动 composable -------
useDragResize(dragHandleRef, {
  enabled: dragMode,
  onDrag({ x, y }) {
    posX.value += x;
    posY.value += y;
    void setPosition(posX.value, posY.value);
  },
});

// ------- Tauri 调用 -------
async function setPosition(x: number, y: number) {
  await invoke("set_window_position", { windowLabel: "island", x, y }).catch(
    (e) => console.error("[live-edit] set_window_position failed:", e),
  );
}

// ------- 快捷键预设（数字键 1-9） -------
// 1=左上 2=中上 3=右上 4=左中 5=中 6=右中 7=左下 8=中下 9=右下
async function applyPreset(num: number) {
  let monitors: Monitor[];
  try {
    monitors = await availableMonitors();
  } catch {
    monitors = [];
  }

  const mon = monitors[0];
  // 屏幕可用区域（无 monitor 时用 screen API 回退）
  const sw = mon ? mon.size.width : window.screen.availWidth;
  const sh = mon ? mon.size.height : window.screen.availHeight;
  const w = winW.value;
  const h = winH.value;

  const col = ((num - 1) % 3) as 0 | 1 | 2; // 0=left 1=center 2=right
  const row = Math.floor((num - 1) / 3) as 0 | 1 | 2; // 0=top 1=mid 2=bottom

  const xMap = [0, Math.round((sw - w) / 2), Math.round(sw - w)];
  const yMap = [0, Math.round((sh - h) / 2), Math.round(sh - h)];

  posX.value = xMap[col];
  posY.value = yMap[row];
  await setPosition(posX.value, posY.value);
}

function onKeyDown(e: KeyboardEvent) {
  const n = parseInt(e.key, 10);
  if (n >= 1 && n <= 9) {
    e.preventDefault();
    void applyPreset(n);
  }
}

// ------- 按钮操作 -------
async function snapToTop() {
  posY.value = 0;
  await setPosition(posX.value, posY.value);
}

function toggleDragMode() {
  dragMode.value = !dragMode.value;
}

async function resetPosition() {
  // 水平居中，Y=0
  let sw = window.screen.availWidth;
  try {
    const monitors = await availableMonitors();
    if (monitors[0]) sw = monitors[0].size.width;
  } catch {
    /* ignore */
  }
  posX.value = Math.round((sw - winW.value) / 2);
  posY.value = 0;
  await setPosition(posX.value, posY.value);
}

// X 轴微调
async function nudgeX(delta: number) {
  posX.value += delta;
  await setPosition(posX.value, posY.value);
}

// ------- 保存 / 取消 -------
async function save() {
  const patch: { notch_position: WindowPosition } = {
    notch_position: { x: posX.value, y: posY.value },
  };
  await invoke("update_settings", { patch }).catch((e) =>
    console.error("[live-edit] update_settings failed:", e),
  );
  emit("close");
}

function cancel() {
  void setPosition(originX.value, originY.value);
  emit("close");
}

// ------- lifecycle -------
onMounted(() => {
  // 禁用刘海动画
  document.body.classList.add("no-notch-anim");

  // 记录当前窗口位置作为原始值（jsdom 下 screenX/Y 为 0，实际运行时有效）
  originX.value = window.screenX ?? 0;
  originY.value = window.screenY ?? 0;
  posX.value = originX.value;
  posY.value = originY.value;

  window.addEventListener("keydown", onKeyDown);
});

onUnmounted(() => {
  document.body.classList.remove("no-notch-anim");
  window.removeEventListener("keydown", onKeyDown);
});

// ------- emits -------
const emit = defineEmits<{ (e: "close"): void }>();
</script>

<template>
  <div class="live-edit-wrapper" data-testid="live-edit-wrapper">
    <!-- 黄色虚线边框 + 尺寸指示器 -->
    <div class="live-edit-frame" :class="{ 'drag-active': dragMode }">
      <span class="size-indicator" data-testid="size-indicator">{{ sizeLabel }}</span>

      <!-- 拖动手柄（drag mode 开启时可拖动） -->
      <div
        ref="dragHandleRef"
        class="drag-handle"
        :class="{ 'drag-handle--active': dragMode }"
        data-testid="drag-handle"
      >
        <span v-if="dragMode" class="drag-hint">拖动中...</span>
      </div>

      <!-- X 轴微调 -->
      <div class="nudge-row" data-testid="nudge-row">
        <button class="btn-nudge" data-testid="btn-nudge-left" @click="nudgeX(-1)">←</button>
        <span class="nudge-label">X: {{ posX }}</span>
        <button class="btn-nudge" data-testid="btn-nudge-right" @click="nudgeX(1)">→</button>
      </div>

      <!-- 3 个功能按钮 -->
      <div class="action-row" data-testid="action-row">
        <button class="btn-action" data-testid="btn-snap-top" @click="snapToTop">贴合顶部</button>
        <button
          class="btn-action"
          :class="{ 'btn-action--active': dragMode }"
          data-testid="btn-drag-mode"
          @click="toggleDragMode"
        >
          拖动模式
        </button>
        <button class="btn-action" data-testid="btn-reset" @click="resetPosition">复位</button>
      </div>

      <!-- 快捷键提示 -->
      <div class="shortcut-hint" data-testid="shortcut-hint">
        <span>数字 1-9 快速对齐预设位置</span>
      </div>

      <!-- 保存 / 取消 -->
      <div class="save-row" data-testid="save-row">
        <button class="btn-save" data-testid="btn-save" @click="save">保存</button>
        <button class="btn-cancel" data-testid="btn-cancel" @click="cancel">取消</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.live-edit-wrapper {
  position: fixed;
  inset: 0;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  pointer-events: none;
}

.live-edit-frame {
  pointer-events: all;
  position: relative;
  border: 2px dashed var(--accent-yellow);
  border-radius: var(--radius-sm);
  background: var(--bg-secondary);
  color: var(--text-primary);
  padding: 8px 12px 10px;
  min-width: 200px;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.live-edit-frame.drag-active {
  cursor: grab;
}

.size-indicator {
  position: absolute;
  top: -1px;
  right: 6px;
  font-size: 11px;
  color: var(--accent-yellow);
  background: var(--bg-secondary);
  padding: 0 4px;
  pointer-events: none;
}

.drag-handle {
  width: 100%;
  height: 24px;
  border-radius: 4px;
  background: transparent;
  display: flex;
  align-items: center;
  justify-content: center;
}

.drag-handle--active {
  cursor: grabbing;
  background: rgba(255, 152, 0, 0.08);
}

.drag-hint {
  font-size: 11px;
  color: var(--accent-yellow);
}

.nudge-row {
  display: flex;
  align-items: center;
  gap: 6px;
}

.nudge-label {
  font-size: 12px;
  color: var(--text-secondary);
  min-width: 60px;
  text-align: center;
}

.btn-nudge {
  background: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--text-tertiary);
  border-radius: 4px;
  padding: 2px 8px;
  cursor: pointer;
  font-size: 14px;
  line-height: 1.4;
}

.btn-nudge:hover {
  border-color: var(--accent-yellow);
}

.action-row {
  display: flex;
  gap: 6px;
}

.btn-action {
  flex: 1;
  background: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--text-tertiary);
  border-radius: 4px;
  padding: 4px 6px;
  cursor: pointer;
  font-size: 12px;
}

.btn-action:hover,
.btn-action--active {
  border-color: var(--accent-yellow);
  color: var(--accent-yellow);
}

.shortcut-hint {
  font-size: 11px;
  color: var(--text-tertiary);
  text-align: center;
}

.save-row {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  margin-top: 2px;
}

.btn-save {
  background: var(--accent-yellow);
  color: #000;
  border: none;
  border-radius: 4px;
  padding: 4px 14px;
  cursor: pointer;
  font-size: 12px;
  font-weight: 600;
}

.btn-save:hover {
  opacity: 0.85;
}

.btn-cancel {
  background: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--text-tertiary);
  border-radius: 4px;
  padding: 4px 10px;
  cursor: pointer;
  font-size: 12px;
}

.btn-cancel:hover {
  color: var(--text-primary);
  border-color: var(--text-secondary);
}
</style>
