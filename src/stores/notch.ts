import { defineStore } from "pinia";
import { computed, ref } from "vue";
import {
  COLLAPSED,
  EXPANDED,
  computeNotchPath,
  interpolateGeometry,
  type NotchGeometry,
} from "@/utils/notch-shape";

export const useNotchStore = defineStore("notch", () => {
  const expanded = ref(false);
  const geometry = ref<NotchGeometry>({ ...COLLAPSED });

  const path = computed(() => computeNotchPath(geometry.value));

  const viewBox = computed(
    () => `0 0 ${geometry.value.width} ${geometry.value.height}`,
  );

  function setGeometry(next: NotchGeometry) {
    geometry.value = { ...next };
  }

  function setExpanded(next: boolean) {
    expanded.value = next;
    geometry.value = next ? { ...EXPANDED } : { ...COLLAPSED };
  }

  function animateTo(target: NotchGeometry, durationMs = 280): Promise<void> {
    return new Promise((resolve) => {
      const start = { ...geometry.value };
      const startTs = performance.now();
      function tick(now: number) {
        const t = Math.min(1, (now - startTs) / durationMs);
        const eased = easeOutExpo(t);
        geometry.value = interpolateGeometry(start, target, eased);
        if (t < 1) requestAnimationFrame(tick);
        else resolve();
      }
      requestAnimationFrame(tick);
    });
  }

  function animateExpanded(next: boolean, durationMs = 280) {
    expanded.value = next;
    return animateTo(next ? { ...EXPANDED } : { ...COLLAPSED }, durationMs);
  }

  return {
    expanded,
    geometry,
    path,
    viewBox,
    setGeometry,
    setExpanded,
    animateTo,
    animateExpanded,
  };
});

function easeOutExpo(t: number): number {
  return t === 1 ? 1 : 1 - Math.pow(2, -10 * t);
}
