<script setup lang="ts">
/**
 * EmojiPixel — emoji 栅格化渲染组件
 * 将单个 emoji 绘制到 offscreen canvas (16×16)，读取像素数据，
 * 使用 SVG <rect> 网格渲染出像素化效果。
 */
import { ref, onMounted, watch, computed } from 'vue';

interface Props {
  /** 单个 emoji 字符，如 "🐱" */
  emoji: string;
  /** 每格像素尺寸（px），默认 2 → 渲染出 32×32 像素图 */
  scale?: number;
}

const props = withDefaults(defineProps<Props>(), {
  scale: 2,
});

const GRID = 16;

interface PixelColor {
  r: number;
  g: number;
  b: number;
  a: number;
}

/** 16×16 的颜色网格，null = 透明 */
const grid = ref<(PixelColor | null)[][]>([]);

/** 将 emoji 渲染到 offscreen canvas，采样 16×16 像素 */
function rasterize(emoji: string) {
  const canvas = document.createElement('canvas');
  canvas.width = GRID;
  canvas.height = GRID;
  const ctx = canvas.getContext('2d');
  if (!ctx) return;

  // 清空
  ctx.clearRect(0, 0, GRID, GRID);

  // 使用系统默认 emoji 字体（Segoe UI Emoji on Windows）
  const fontSize = Math.floor(GRID * 0.9);
  ctx.font = `${fontSize}px "Segoe UI Emoji", "Apple Color Emoji", serif`;
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  ctx.fillText(emoji, GRID / 2, GRID / 2);

  const imageData = ctx.getImageData(0, 0, GRID, GRID);
  const result: (PixelColor | null)[][] = [];

  for (let row = 0; row < GRID; row++) {
    const rowData: (PixelColor | null)[] = [];
    for (let col = 0; col < GRID; col++) {
      const idx = (row * GRID + col) * 4;
      const a = imageData.data[idx + 3];
      if (a < 16) {
        rowData.push(null);
      } else {
        rowData.push({
          r: imageData.data[idx],
          g: imageData.data[idx + 1],
          b: imageData.data[idx + 2],
          a,
        });
      }
    }
    result.push(rowData);
  }

  grid.value = result;
}

onMounted(() => rasterize(props.emoji));
watch(() => props.emoji, (e) => rasterize(e));

/** 有颜色的像素列表（减少 template 循环层级） */
const pixels = computed<{ x: number; y: number; fill: string }[]>(() => {
  const out: { x: number; y: number; fill: string }[] = [];
  for (let r = 0; r < grid.value.length; r++) {
    const row = grid.value[r];
    for (let c = 0; c < row.length; c++) {
      const px = row[c];
      if (px) {
        const alpha = (px.a / 255).toFixed(2);
        out.push({
          x: c,
          y: r,
          fill: `rgba(${px.r},${px.g},${px.b},${alpha})`,
        });
      }
    }
  }
  return out;
});

const svgSize = computed(() => GRID * props.scale);
</script>

<template>
  <svg
    class="emoji-pixel"
    :width="svgSize"
    :height="svgSize"
    :viewBox="`0 0 ${GRID} ${GRID}`"
    xmlns="http://www.w3.org/2000/svg"
    :aria-label="emoji"
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
.emoji-pixel {
  display: block;
  image-rendering: pixelated;
  image-rendering: crisp-edges;
}
</style>
