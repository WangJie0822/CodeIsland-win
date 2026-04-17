import { type Ref, watch, onUnmounted } from "vue";

export interface DragPos {
  x: number;
  y: number;
}

export interface UseDragResizeOptions {
  onDragStart?: (pos: DragPos) => void;
  onDrag: (pos: DragPos) => void;
  onDragEnd?: (pos: DragPos) => void;
  enabled: Ref<boolean>;
}

/**
 * 拖动定位 composable。
 * 监听 target 上的 pointer 事件，通过回调上报坐标增量。
 * enabled 为 false 时不响应任何事件。
 */
export function useDragResize(
  target: Ref<HTMLElement | null>,
  opts: UseDragResizeOptions,
): void {
  let isDragging = false;
  let startX = 0;
  let startY = 0;

  function onPointerDown(e: PointerEvent) {
    if (!opts.enabled.value) return;
    isDragging = true;
    startX = e.clientX;
    startY = e.clientY;
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    opts.onDragStart?.({ x: e.clientX, y: e.clientY });
  }

  function onPointerMove(e: PointerEvent) {
    if (!isDragging || !opts.enabled.value) return;
    const dx = e.clientX - startX;
    const dy = e.clientY - startY;
    startX = e.clientX;
    startY = e.clientY;
    opts.onDrag({ x: dx, y: dy });
  }

  function onPointerUp(e: PointerEvent) {
    if (!isDragging) return;
    isDragging = false;
    (e.currentTarget as HTMLElement).releasePointerCapture(e.pointerId);
    opts.onDragEnd?.({ x: e.clientX, y: e.clientY });
  }

  function attach(el: HTMLElement) {
    el.addEventListener("pointerdown", onPointerDown);
    el.addEventListener("pointermove", onPointerMove);
    el.addEventListener("pointerup", onPointerUp);
  }

  function detach(el: HTMLElement) {
    el.removeEventListener("pointerdown", onPointerDown);
    el.removeEventListener("pointermove", onPointerMove);
    el.removeEventListener("pointerup", onPointerUp);
  }

  const stopWatch = watch(
    target,
    (newEl, oldEl) => {
      if (oldEl) detach(oldEl);
      if (newEl) attach(newEl);
    },
    { immediate: true },
  );

  onUnmounted(() => {
    stopWatch();
    if (target.value) detach(target.value);
  });
}
