<script setup lang="ts">
/**
 * PixelCharacter — 通用像素角色渲染器
 * 接收 sprite 数据（调色板 + 帧索引数组），按 steps(N, end) 逐帧动画。
 */
import { ref, computed, onMounted, onUnmounted } from 'vue';

/** 单色调色板项 */
export interface PaletteEntry {
  /** 颜色索引（>= 1；0 保留为透明） */
  index: number;
  /** CSS 颜色字符串，支持 CSS 变量引用（如 "var(--accent-green)"） */
  color: string;
}

/** Sprite 数据：调色板 + 帧（每帧为宽×高的颜色索引二维数组） */
export interface SpriteData {
  /** 精灵宽度（格数） */
  width: number;
  /** 精灵高度（格数） */
  height: number;
  /** 调色板，index 0 = 透明 */
  palette: PaletteEntry[];
  /** 帧数组，每帧为 height×width 的颜色索引二维数组 */
  frames: number[][][];
}

interface Props {
  /** sprite 数据 */
  sprite: SpriteData;
  /** 每格渲染尺寸（px），默认 2 */
  scale?: number;
  /** 帧切换间隔（ms），默认 200 */
  frameDuration?: number;
  /** 是否循环播放，默认 true */
  loop?: boolean;
  /** 是否暂停（停在第一帧），默认 false */
  paused?: boolean;
  /** 无障碍标签 */
  label?: string;
}

const props = withDefaults(defineProps<Props>(), {
  scale: 2,
  frameDuration: 200,
  loop: true,
  paused: false,
  label: '像素角色',
});

const currentFrame = ref(0);
let timer: ReturnType<typeof setInterval> | null = null;

function startTimer() {
  if (timer !== null) return;
  if (props.sprite.frames.length <= 1) return;
  timer = setInterval(() => {
    const next = currentFrame.value + 1;
    if (next >= props.sprite.frames.length) {
      if (props.loop) {
        currentFrame.value = 0;
      } else {
        stopTimer();
      }
    } else {
      currentFrame.value = next;
    }
  }, props.frameDuration);
}

function stopTimer() {
  if (timer !== null) {
    clearInterval(timer);
    timer = null;
  }
}

onMounted(() => {
  if (!props.paused) startTimer();
});

onUnmounted(() => stopTimer());

/** 颜色索引 → CSS 颜色字符串映射 */
const colorMap = computed<Record<number, string>>(() => {
  const map: Record<number, string> = {};
  for (const entry of props.sprite.palette) {
    map[entry.index] = entry.color;
  }
  return map;
});

/** 当前帧可见像素列表 */
const pixels = computed<{ x: number; y: number; fill: string }[]>(() => {
  const frame = props.sprite.frames[currentFrame.value];
  if (!frame) return [];
  const out: { x: number; y: number; fill: string }[] = [];
  for (let r = 0; r < frame.length; r++) {
    const row = frame[r];
    for (let c = 0; c < row.length; c++) {
      const idx = row[c];
      if (idx !== 0 && colorMap.value[idx]) {
        out.push({ x: c, y: r, fill: colorMap.value[idx] });
      }
    }
  }
  return out;
});

const svgWidth = computed(() => props.sprite.width * props.scale);
const svgHeight = computed(() => props.sprite.height * props.scale);
</script>

<template>
  <svg
    class="pixel-character"
    :width="svgWidth"
    :height="svgHeight"
    :viewBox="`0 0 ${sprite.width} ${sprite.height}`"
    xmlns="http://www.w3.org/2000/svg"
    :aria-label="label"
    role="img"
  >
    <rect
      v-for="(px, i) in pixels"
      :key="i"
      :x="px.x"
      :y="px.y"
      width="1"
      height="1"
      :fill="px.fill"
    />
  </svg>
</template>

<style scoped>
.pixel-character {
  display: block;
  image-rendering: pixelated;
  image-rendering: crisp-edges;
}
</style>
