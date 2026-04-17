<script setup lang="ts">
/**
 * ProcessingSpinner — 像素风格加载动画
 * 4 帧 SVG 雪碧图，使用 CSS animation steps(4, end) 逐帧切换。
 * 设计：8×8 旋转点阵（顺时针旋转 4 帧）
 */
import { computed } from 'vue';

interface Props {
  /** 渲染尺寸（px），默认 16 */
  size?: number;
  /** 旋转一圈时长（ms），默认 600 */
  duration?: number;
  /** 颜色（CSS color），默认使用 --pixel-tint-default */
  color?: string;
}

const props = withDefaults(defineProps<Props>(), {
  size: 16,
  duration: 600,
  color: undefined,
});

const GRID = 8;

/**
 * 4 帧旋转点阵，每帧 8×8 bit。
 * 设计：顺时针旋转的 L 形像素臂。
 */
const FRAMES: number[][][] = [
  // 帧 0 — 12点方向
  [
    [0,0,0,1,1,0,0,0],
    [0,0,0,1,1,0,0,0],
    [0,0,0,1,0,0,0,0],
    [0,0,0,1,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
  ],
  // 帧 1 — 3点方向
  [
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,1,1,1],
    [0,0,0,0,0,0,1,1],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
  ],
  // 帧 2 — 6点方向
  [
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,1,0,0,0],
    [0,0,0,0,1,0,0,0],
    [0,0,0,1,1,0,0,0],
    [0,0,0,1,1,0,0,0],
  ],
  // 帧 3 — 9点方向
  [
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [1,1,0,0,0,0,0,0],
    [1,1,1,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0],
  ],
];

interface Rect { x: number; y: number }

function frameRects(fi: number): Rect[] {
  const frame = FRAMES[fi];
  const out: Rect[] = [];
  for (let r = 0; r < GRID; r++) {
    for (let c = 0; c < GRID; c++) {
      if (frame[r][c]) out.push({ x: c, y: r });
    }
  }
  return out;
}

const allFrameRects = FRAMES.map((_, i) => frameRects(i));

const symbolIds = ['psp-f0', 'psp-f1', 'psp-f2', 'psp-f3'];

const colorStyle = computed(() =>
  props.color ? `color: ${props.color}` : undefined,
);

const animDuration = computed(() => `${props.duration}ms`);
</script>

<template>
  <div
    class="psp"
    :style="[`--psp-size: ${props.size}px`, `--psp-dur: ${animDuration}`, colorStyle]"
    role="status"
    aria-label="加载中"
  >
    <!-- defs -->
    <svg xmlns="http://www.w3.org/2000/svg" class="psp__defs" aria-hidden="true">
      <defs>
        <symbol
          v-for="(_, fi) in FRAMES"
          :id="symbolIds[fi]"
          :key="symbolIds[fi]"
          :viewBox="`0 0 ${GRID} ${GRID}`"
        >
          <rect
            v-for="(r, ri) in allFrameRects[fi]"
            :key="ri"
            :x="r.x"
            :y="r.y"
            width="1"
            height="1"
            fill="currentColor"
          />
        </symbol>
      </defs>
    </svg>

    <svg
      class="psp__canvas"
      :width="props.size"
      :height="props.size"
      :viewBox="`0 0 ${GRID} ${GRID}`"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
    >
      <!-- 4 帧，每帧占 25% 时间窗口，依次 animation-delay -->
      <use
        v-for="fi in [0, 1, 2, 3]"
        :key="fi"
        :href="`#${symbolIds[fi]}`"
        :class="`psp__frame psp__frame--${fi}`"
        :data-frame="fi"
      />
    </svg>
  </div>
</template>

<style scoped>
.psp {
  display: inline-block;
  color: var(--pixel-tint-default, var(--accent-blue));
  image-rendering: pixelated;
}

.psp__defs {
  position: absolute;
  width: 0;
  height: 0;
  overflow: hidden;
}

.psp__canvas {
  display: block;
  image-rendering: pixelated;
  image-rendering: crisp-edges;
  width: var(--psp-size, 16px);
  height: var(--psp-size, 16px);
}

/* 每帧默认隐藏，通过 animation 依次显示 */
.psp__frame {
  visibility: hidden;
  animation: psp-show var(--psp-dur, 600ms) steps(1, end) infinite;
}

/* 4 帧各偏移 25% = dur/4 */
.psp__frame--0 { animation-delay: 0ms; }
.psp__frame--1 { animation-delay: calc(var(--psp-dur, 600ms) * 0.25); }
.psp__frame--2 { animation-delay: calc(var(--psp-dur, 600ms) * 0.5); }
.psp__frame--3 { animation-delay: calc(var(--psp-dur, 600ms) * 0.75); }

@keyframes psp-show {
  0%,   24.9% { visibility: visible; }
  25%,  99.9% { visibility: hidden; }
}
</style>
