<script setup lang="ts">
/**
 * NeonPixelCat — 像素风霓虹猫头像
 * 8 帧 SVG 雪碧图，<use> 切换帧，animation-timing-function: steps(N, end) 逐帧动画。
 * 状态映射：
 *   idle             → 静止（帧 0）
 *   processing       → 眨眼 + 摆尾（帧 0-3 循环）
 *   waitingForApproval → 警觉 + 闪烁（帧 4-5 循环）
 *   waitingForInput  → 伸懒腰（帧 6-7 循环）
 */
import { computed } from 'vue';
import type { AnimationState } from './types';

interface Props {
  state: AnimationState;
  /** 渲染尺寸（像素），默认 32 */
  size?: number;
  /** 叠加色调（CSS color），默认使用 --pixel-tint-default */
  tint?: string;
}

const props = withDefaults(defineProps<Props>(), {
  size: 32,
  tint: undefined,
});

/** 16×16 像素猫体素 —— 每帧 16 行，每行 16 列，1=前景，0=背景 */
const FRAMES: number[][][] = [
  // 帧 0 — idle（正面静止，双眼睁开）
  [
    [0,0,0,0,0,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,1,1,1,1,1,1,1,1,1,1,0,0,0],
    [0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
    [0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0],
    [0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,0,0,1,1,1,1,0,0,1,1,1,0],
    [0,1,1,1,0,1,1,1,1,1,1,0,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,0,1,1,1,1,0,1,1,0,1,1,1,1,0,0],
    [0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
    [0,0,0,1,1,1,1,0,0,1,1,1,1,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,1,1,0,0,0,0,0,0,1,1,0,0,0],
    [0,0,1,1,0,0,0,0,0,0,0,0,1,1,0,0],
  ],
  // 帧 1 — processing 眨眼（眼睛半闭）
  [
    [0,0,0,0,0,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,1,1,1,1,1,1,1,1,1,1,0,0,0],
    [0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
    [0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0],
    [0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,0,0,1,1,1,1,0,0,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0], // 眼睛条
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,0,1,1,1,1,0,1,1,0,1,1,1,1,0,0],
    [0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
    [0,0,0,1,1,1,1,0,0,1,1,1,1,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,1,1,0,0,0,0,0,0,1,1,0,0,0],
    [0,0,1,1,0,0,0,0,0,0,0,0,1,1,0,0],
  ],
  // 帧 2 — processing 眨眼全闭
  [
    [0,0,0,0,0,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,1,1,1,1,1,1,1,1,1,1,0,0,0],
    [0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
    [0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0],
    [0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0], // 眼睛全闭上线
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,0,1,1,1,1,0,1,1,0,1,1,1,1,0,0],
    [0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
    [0,0,0,1,1,1,1,0,0,1,1,1,1,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,1,1,0,0,0,0,0,0,1,1,0,0,0],
    [0,0,1,1,0,0,0,0,0,0,0,0,1,1,0,0],
  ],
  // 帧 3 — processing 摆尾（尾巴右移）
  [
    [0,0,0,0,0,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,1,1,1,1,1,1,1,1,1,1,0,0,0],
    [0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
    [0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0],
    [0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,0,0,1,1,1,1,0,0,1,1,1,0],
    [0,1,1,1,0,1,1,1,1,1,1,0,1,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,0,1,1,1,1,0,1,1,0,1,1,1,1,0,0],
    [0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
    [0,0,0,1,1,1,1,0,0,1,1,1,1,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,0,1,1,0,0,0,0,0,0,1,1,0,0], // 尾巴右移
    [0,0,0,0,0,1,1,0,0,0,0,0,0,1,1,0],
  ],
  // 帧 4 — waitingForApproval 警觉（耳朵竖起，眼睛变大）
  [
    [0,0,1,0,0,1,1,1,1,1,1,0,0,1,0,0], // 耳朵竖
    [0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
    [0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
    [0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0],
    [0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,0,0,0,1,1,1,1,0,0,0,1,1,0], // 大眼
    [0,1,1,0,0,1,1,1,1,1,1,0,0,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,0,1,1,1,1,0,1,1,0,1,1,1,1,0,0],
    [0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
    [0,0,0,1,1,1,1,0,0,1,1,1,1,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,1,1,0,0,0,0,0,0,1,1,0,0,0],
    [0,0,1,1,0,0,0,0,0,0,0,0,1,1,0,0],
  ],
  // 帧 5 — waitingForApproval 闪烁（身体轮廓加亮）
  [
    [0,0,1,0,0,1,1,1,1,1,1,0,0,1,0,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0], // 轮廓加亮
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [1,1,1,0,1,1,1,1,1,1,1,1,0,1,1,1],
    [1,1,1,0,1,1,1,1,1,1,1,1,0,1,1,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,1,1,0,0,0,1,1,1,1,0,0,0,1,1,1],
    [1,1,1,0,0,1,1,1,1,1,1,0,0,1,1,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [0,1,1,1,1,1,0,1,1,0,1,1,1,1,1,0],
    [0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
    [0,0,0,1,1,1,1,0,0,1,1,1,1,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,1,1,0,0,0,0,0,0,1,1,0,0,0],
    [0,0,1,1,0,0,0,0,0,0,0,0,1,1,0,0],
  ],
  // 帧 6 — waitingForInput 伸懒腰（前爪伸出）
  [
    [0,0,0,0,0,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,1,1,1,1,1,1,1,1,1,1,0,0,0],
    [0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
    [0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0],
    [0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [0,1,1,1,0,0,1,1,1,1,0,0,1,1,1,0],
    [0,1,1,1,0,1,1,1,1,1,1,0,1,1,1,0],
    [0,1,1,1,1,1,1,0,0,1,1,1,1,1,1,0],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], // 爪子伸出
    [1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,1],
    [1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,1],
    [0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,1,1,0,0,0,0,0,0,1,1,0,0,0],
    [0,0,1,1,0,0,0,0,0,0,0,0,1,1,0,0],
  ],
  // 帧 7 — waitingForInput 伸懒腰（背部弓起）
  [
    [0,0,0,0,0,1,1,1,1,1,1,0,0,0,0,0],
    [0,0,0,1,1,1,1,1,1,1,1,1,1,0,0,0],
    [0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
    [0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0],
    [0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0],
    [0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0], // 背弓起缩肩
    [0,0,0,1,0,0,1,1,1,1,0,0,1,0,0,0],
    [0,0,0,1,0,1,1,1,1,1,1,0,1,0,0,0],
    [0,0,1,1,1,1,1,0,0,1,1,1,1,1,0,0],
    [0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    [1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,1],
    [1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,1],
    [0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0],
    [0,0,0,1,1,0,0,0,0,0,0,1,1,0,0,0],
    [0,0,1,1,0,0,0,0,0,0,0,0,1,1,0,0],
  ],
];

/** 根据状态返回参与动画的帧索引列表 */
const stateFrames: Record<AnimationState, number[]> = {
  idle:              [0],
  processing:        [0, 1, 2, 3],
  waitingForApproval:[4, 5],
  waitingForInput:   [6, 7],
};

const GRID = 16;

const cssClass = computed<string[]>(() => {
  const base = ['npc'];
  base.push(`npc--${props.state}`);
  return base;
});

/** 将帧数据转为若干 <rect> 描述（以 SVG 内坐标系） */
function frameRects(frameIdx: number): { x: number; y: number }[] {
  const frame = FRAMES[frameIdx];
  const rects: { x: number; y: number }[] = [];
  for (let r = 0; r < GRID; r++) {
    for (let c = 0; c < GRID; c++) {
      if (frame[r][c]) rects.push({ x: c, y: r });
    }
  }
  return rects;
}

/** 每帧对应的 SVG symbol id */
const symbolIds = FRAMES.map((_, i) => `npc-frame-${i}`);

const tintStyle = computed(() =>
  props.tint ? `color: ${props.tint}` : undefined,
);
</script>

<template>
  <div
    :class="cssClass"
    :style="[
      `--npc-size: ${props.size}px`,
      tintStyle,
    ]"
    role="img"
    aria-label="像素猫"
  >
    <!-- SVG 雪碧图定义（hidden，仅用于 <use> 引用） -->
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="npc__defs"
      aria-hidden="true"
    >
      <defs>
        <symbol
          v-for="(_, fi) in FRAMES"
          :id="symbolIds[fi]"
          :key="symbolIds[fi]"
          :viewBox="`0 0 ${GRID} ${GRID}`"
        >
          <rect
            v-for="(rect, ri) in frameRects(fi)"
            :key="ri"
            :x="rect.x"
            :y="rect.y"
            width="1"
            height="1"
            fill="currentColor"
          />
        </symbol>
      </defs>
    </svg>

    <!-- 渲染区：按状态挂动画类，动画通过 CSS @keyframes + steps 控制 <use> visibility -->
    <svg
      class="npc__canvas"
      :width="props.size"
      :height="props.size"
      :viewBox="`0 0 ${GRID} ${GRID}`"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
    >
      <!-- 每帧一个 <use>，通过 CSS animation steps 逐帧显示/隐藏 -->
      <use
        v-for="fi in stateFrames[props.state]"
        :key="fi"
        :href="`#${symbolIds[fi]}`"
        :class="`npc__frame npc__frame--${props.state}-f${fi}`"
        :data-frame="fi"
        :data-frame-index="stateFrames[props.state].indexOf(fi)"
        :data-frame-count="stateFrames[props.state].length"
      />
    </svg>
  </div>
</template>

<style scoped>
.npc {
  display: inline-block;
  color: var(--pixel-tint-default, var(--accent-green));
  image-rendering: pixelated;
  image-rendering: crisp-edges;
}

.npc__defs {
  /* 仅用于定义 symbol，不占布局空间 */
  position: absolute;
  width: 0;
  height: 0;
  overflow: hidden;
}

.npc__canvas {
  display: block;
  image-rendering: pixelated;
  image-rendering: crisp-edges;
  width: var(--npc-size, 32px);
  height: var(--npc-size, 32px);
}

/* idle: 只有帧 0，始终可见 */
.npc--idle .npc__frame {
  visibility: visible;
}

/* processing: 4 帧循环，steps(4, end) */
.npc--processing .npc__frame {
  visibility: hidden;
  animation: npc-frame-cycle-4 0.6s steps(1, end) infinite;
  animation-timing-function: steps(1, end);
}
.npc--processing .npc__frame[data-frame-index="0"] { animation-delay: 0s; }
.npc--processing .npc__frame[data-frame-index="1"] { animation-delay: 0.15s; }
.npc--processing .npc__frame[data-frame-index="2"] { animation-delay: 0.30s; }
.npc--processing .npc__frame[data-frame-index="3"] { animation-delay: 0.45s; }

/* waitingForApproval: 2 帧交替闪烁 */
.npc--waitingForApproval .npc__frame {
  visibility: hidden;
  animation: npc-frame-cycle-2 0.4s steps(1, end) infinite;
}
.npc--waitingForApproval .npc__frame[data-frame-index="0"] { animation-delay: 0s; }
.npc--waitingForApproval .npc__frame[data-frame-index="1"] { animation-delay: 0.2s; }

/* waitingForInput: 2 帧慢切换（伸懒腰节奏） */
.npc--waitingForInput .npc__frame {
  visibility: hidden;
  animation: npc-frame-cycle-2 1.2s steps(1, end) infinite;
}
.npc--waitingForInput .npc__frame[data-frame-index="0"] { animation-delay: 0s; }
.npc--waitingForInput .npc__frame[data-frame-index="1"] { animation-delay: 0.6s; }

@keyframes npc-frame-cycle-4 {
  0%,   24.9% { visibility: visible; }
  25%,  99.9% { visibility: hidden; }
}

@keyframes npc-frame-cycle-2 {
  0%,   49.9% { visibility: visible; }
  50%,  99.9% { visibility: hidden; }
}
</style>
