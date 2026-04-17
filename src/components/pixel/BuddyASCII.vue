<script setup lang="ts">
/**
 * BuddyASCII — ASCII 艺术渲染器
 * 使用 <pre> 等宽字体，逐字符 <span> 分段上色（colored 模式）。
 * frames 为 ASCII 帧数组；多帧时按 steps(N, end) 切换。
 */
import { computed, ref, onMounted, onUnmounted } from 'vue';

interface Props {
  /** ASCII 帧数组，每帧为多行字符串（换行符分隔） */
  frames: string[];
  /** 字体尺寸（px），控制 <pre> 的 font-size，默认 10 */
  size?: number;
  /** 是否彩色渲染（对特定字符着色），默认 false */
  colored?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  size: 10,
  colored: false,
});

/** 当前帧索引 */
const currentFrameIdx = ref(0);
let timer: ReturnType<typeof setInterval> | null = null;

/** 帧间隔（ms）：多帧时 400ms 切换，单帧不切换 */
const FRAME_INTERVAL = 400;

onMounted(() => {
  if (props.frames.length > 1) {
    timer = setInterval(() => {
      currentFrameIdx.value = (currentFrameIdx.value + 1) % props.frames.length;
    }, FRAME_INTERVAL);
  }
});

onUnmounted(() => {
  if (timer !== null) clearInterval(timer);
});

/** 字符颜色映射（colored 模式） */
const CHAR_COLORS: Record<string, string> = {
  '@': 'var(--accent-purple)',
  '#': 'var(--accent-blue)',
  '*': 'var(--accent-yellow)',
  '~': 'var(--accent-green)',
  '^': 'var(--accent-orange)',
  '!': 'var(--accent-red)',
  '=': 'var(--accent-blue)',
  '-': 'var(--text-secondary)',
  '_': 'var(--text-secondary)',
  '|': 'var(--text-tertiary)',
  '/': 'var(--text-tertiary)',
  '\\': 'var(--text-tertiary)',
};

interface ColoredSpan {
  text: string;
  color: string | null;
}

/** 将一行文本解析为带颜色的 span 段 */
function parseLine(line: string): ColoredSpan[] {
  if (!props.colored) {
    return [{ text: line, color: null }];
  }
  const spans: ColoredSpan[] = [];
  let i = 0;
  while (i < line.length) {
    const ch = line[i];
    const color = CHAR_COLORS[ch] ?? null;
    // 合并相同颜色的连续字符
    let j = i + 1;
    while (j < line.length && (CHAR_COLORS[line[j]] ?? null) === color) {
      j++;
    }
    spans.push({ text: line.slice(i, j), color });
    i = j;
  }
  return spans;
}

/** 当前帧的行列表（每行为 span 数组） */
const parsedLines = computed<ColoredSpan[][]>(() => {
  const frame = props.frames[currentFrameIdx.value] ?? '';
  return frame.split('\n').map(parseLine);
});

const fontSizeStyle = computed(() => `font-size: ${props.size}px`);
</script>

<template>
  <pre
    class="buddy-ascii"
    :style="fontSizeStyle"
    role="img"
    aria-label="ASCII 角色"
  ><template
      v-for="(line, li) in parsedLines"
      :key="li"
    ><template
        v-for="(span, si) in line"
        :key="si"
      ><span
          v-if="span.color"
          class="buddy-ascii__span"
          :style="`color: ${span.color}`"
        >{{ span.text }}</span><span
          v-else
          class="buddy-ascii__span"
        >{{ span.text }}</span></template>
<template v-if="li < parsedLines.length - 1">
</template></template></pre>
</template>

<style scoped>
.buddy-ascii {
  font-family: var(--pixel-font, var(--font-mono, monospace));
  font-size: inherit;
  line-height: 1.2;
  white-space: pre;
  color: var(--text-primary);
  /* 禁止文本选中（纯装饰） */
  user-select: none;
  image-rendering: pixelated;
  /* 等宽排版关键：禁止连字 */
  font-variant-ligatures: none;
  font-feature-settings: "liga" 0;
}

.buddy-ascii__span {
  /* 继承父级字体，保持等宽 */
  font-family: inherit;
  font-size: inherit;
}
</style>
