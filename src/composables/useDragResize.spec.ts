import { describe, expect, it, vi, beforeEach } from "vitest";
import { ref, nextTick } from "vue";
import { useDragResize } from "./useDragResize";

// 最小化 HTMLElement 模拟，支持 addEventListener/removeEventListener
// 和 setPointerCapture/releasePointerCapture
function makeEl() {
  const listeners: Record<string, EventListener[]> = {};
  const el = {
    addEventListener(type: string, fn: EventListener) {
      if (!listeners[type]) listeners[type] = [];
      listeners[type].push(fn);
    },
    removeEventListener(type: string, fn: EventListener) {
      if (listeners[type]) {
        listeners[type] = listeners[type].filter((f) => f !== fn);
      }
    },
    setPointerCapture: vi.fn(),
    releasePointerCapture: vi.fn(),
    dispatchEvent(e: Event) {
      // currentTarget 是只读的，需要定义属性
      Object.defineProperty(e, "currentTarget", { value: el, configurable: true });
      const fns = listeners[e.type] ?? [];
      for (const fn of fns) fn(e);
    },
  };
  return el as unknown as HTMLElement;
}

function makePointerEvent(
  type: string,
  clientX: number,
  clientY: number,
  pointerId = 1,
): PointerEvent {
  return { type, clientX, clientY, pointerId } as unknown as PointerEvent;
}

describe("useDragResize", () => {
  let el: HTMLElement;

  beforeEach(() => {
    el = makeEl();
  });

  it("enabled=false 时不触发 onDrag", () => {
    const target = ref<HTMLElement | null>(el);
    const enabled = ref(false);
    const onDrag = vi.fn();

    useDragResize(target, { onDrag, enabled });

    el.dispatchEvent(makePointerEvent("pointerdown", 100, 200));
    el.dispatchEvent(makePointerEvent("pointermove", 110, 210));

    expect(onDrag).not.toHaveBeenCalled();
  });

  it("enabled=true 时 pointerdown → pointermove 触发 onDrag 并传递 delta", () => {
    const target = ref<HTMLElement | null>(el);
    const enabled = ref(true);
    const onDrag = vi.fn();

    useDragResize(target, { onDrag, enabled });

    el.dispatchEvent(makePointerEvent("pointerdown", 100, 200));
    el.dispatchEvent(makePointerEvent("pointermove", 115, 225));

    expect(onDrag).toHaveBeenCalledOnce();
    expect(onDrag).toHaveBeenCalledWith({ x: 15, y: 25 });
  });

  it("多次 move 累计 delta 分批回调", () => {
    const target = ref<HTMLElement | null>(el);
    const enabled = ref(true);
    const onDrag = vi.fn();

    useDragResize(target, { onDrag, enabled });

    el.dispatchEvent(makePointerEvent("pointerdown", 0, 0));
    el.dispatchEvent(makePointerEvent("pointermove", 10, 0));
    el.dispatchEvent(makePointerEvent("pointermove", 25, 0));

    expect(onDrag).toHaveBeenCalledTimes(2);
    expect(onDrag).toHaveBeenNthCalledWith(1, { x: 10, y: 0 });
    expect(onDrag).toHaveBeenNthCalledWith(2, { x: 15, y: 0 });
  });

  it("pointerup 之后 move 不再触发 onDrag", () => {
    const target = ref<HTMLElement | null>(el);
    const enabled = ref(true);
    const onDrag = vi.fn();

    useDragResize(target, { onDrag, enabled });

    el.dispatchEvent(makePointerEvent("pointerdown", 0, 0));
    el.dispatchEvent(makePointerEvent("pointermove", 5, 5));
    el.dispatchEvent(makePointerEvent("pointerup", 5, 5));
    el.dispatchEvent(makePointerEvent("pointermove", 20, 20));

    expect(onDrag).toHaveBeenCalledTimes(1);
  });

  it("onDragStart 和 onDragEnd 在正确时机调用", () => {
    const target = ref<HTMLElement | null>(el);
    const enabled = ref(true);
    const onDrag = vi.fn();
    const onDragStart = vi.fn();
    const onDragEnd = vi.fn();

    useDragResize(target, { onDrag, onDragStart, onDragEnd, enabled });

    el.dispatchEvent(makePointerEvent("pointerdown", 50, 60));
    expect(onDragStart).toHaveBeenCalledWith({ x: 50, y: 60 });

    el.dispatchEvent(makePointerEvent("pointermove", 55, 65));
    el.dispatchEvent(makePointerEvent("pointerup", 55, 65));
    expect(onDragEnd).toHaveBeenCalledWith({ x: 55, y: 65 });
  });

  it("target 切换时解绑旧元素、绑定新元素", async () => {
    const el2 = makeEl();
    const target = ref<HTMLElement | null>(el);
    const enabled = ref(true);
    const onDrag = vi.fn();

    useDragResize(target, { onDrag, enabled });

    // 切换 target
    target.value = el2;
    await nextTick();

    // 旧元素的事件不再响应
    el.dispatchEvent(makePointerEvent("pointerdown", 0, 0));
    el.dispatchEvent(makePointerEvent("pointermove", 10, 0));
    expect(onDrag).not.toHaveBeenCalled();

    // 新元素响应
    el2.dispatchEvent(makePointerEvent("pointerdown", 0, 0));
    el2.dispatchEvent(makePointerEvent("pointermove", 10, 0));
    expect(onDrag).toHaveBeenCalledWith({ x: 10, y: 0 });
  });
});
