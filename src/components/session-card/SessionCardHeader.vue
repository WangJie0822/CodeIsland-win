<script setup lang="ts">
/**
 * SessionCardHeader — 会话卡片头部
 * 显示：头像（NeonPixelCat 或首字母 fallback）+ 项目名 + launcher 标签 + 时长 + 关闭按钮
 */
import { computed } from 'vue';
import type { SessionSummary } from '@/types/generated';
import type { AnimationState } from '@/components/pixel/types';
import NeonPixelCat from '@/components/pixel/NeonPixelCat.vue';

const props = defineProps<{
  session: SessionSummary;
}>();

const emit = defineEmits<{
  /** 点击关闭按钮（Stage 2 接入 close_session 命令） */
  close: [];
}>();

/** 从 phase 映射到 AnimationState（compacting / ended 降级到 idle） */
const catState = computed<AnimationState>(() => {
  const phase = props.session.phase;
  if (phase === 'processing') return 'processing';
  if (phase === 'waitingForApproval') return 'waitingForApproval';
  if (phase === 'waitingForInput') return 'waitingForInput';
  return 'idle';
});

/** 是否启用 Pixel Cat 头像（缺省为 true） */
const pixelCatEnabled = computed<boolean>(() => {
  return (props.session as any).pixel_cat_enabled ?? true;
});

/** launcher 标签（cmux / tmux / native，缺省为 native） */
const launcher = computed<string>(() => {
  return (props.session as any).launcher ?? 'native';
});

/** 是否展示 launcher 标签（native 时不展示） */
const showLauncher = computed(() => launcher.value !== 'native');

/** last_activity_ts 转为相对时间字符串（缺失时显示空） */
const durationText = computed<string>(() => {
  const ts: number | undefined = (props.session as any).last_activity_ts;
  if (!ts) return '';
  const diffMs = Date.now() - ts * 1000;
  const diffMin = Math.floor(diffMs / 60_000);
  if (diffMin < 1) return '<1m';
  if (diffMin < 60) return `${diffMin}m`;
  const diffH = Math.floor(diffMin / 60);
  if (diffH < 24) return `${diffH}h`;
  return `${Math.floor(diffH / 24)}d`;
});

/** 首字母头像（Pixel Cat 关闭时的 fallback） */
const avatarInitial = computed<string>(() => {
  return (props.session.project_name?.[0] ?? '?').toUpperCase();
});
</script>

<template>
  <div class="sch">
    <!-- 头像区 -->
    <div class="sch__avatar">
      <NeonPixelCat
        v-if="pixelCatEnabled"
        :state="catState"
        :size="32"
      />
      <div v-else class="sch__initial" aria-label="会话头像">
        {{ avatarInitial }}
      </div>
    </div>

    <!-- 信息区 -->
    <div class="sch__info">
      <span class="sch__project-name">{{ session.project_name }}</span>
      <span
        v-if="showLauncher"
        class="sch__launcher-badge pixel-badge"
        :title="`launcher: ${launcher}`"
      >{{ launcher }}</span>
    </div>

    <!-- 右侧控制区 -->
    <div class="sch__controls">
      <span v-if="durationText" class="sch__duration">{{ durationText }}</span>
      <button
        class="sch__close-btn"
        title="关闭会话（Stage 2 可用）"
        aria-label="关闭"
        @click.stop="emit('close')"
      >×</button>
    </div>
  </div>
</template>

<style scoped>
.sch {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 8px 4px;
}

.sch__avatar {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
}

.sch__initial {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: var(--accent-blue);
  color: var(--text-primary);
  font-family: var(--font-sans);
  font-size: 14px;
  font-weight: 600;
  display: flex;
  align-items: center;
  justify-content: center;
  user-select: none;
}

.sch__info {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 6px;
  min-width: 0;
}

.sch__project-name {
  font-family: var(--font-sans);
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.sch__launcher-badge {
  flex-shrink: 0;
}

.sch__controls {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-shrink: 0;
}

.sch__duration {
  font-family: var(--font-mono);
  font-size: 11px;
  color: var(--text-tertiary);
}

.sch__close-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  padding: 0;
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-tertiary);
  font-size: 14px;
  line-height: 1;
  cursor: pointer;
  transition: color var(--dur-fast), background var(--dur-fast);
}

.sch__close-btn:hover {
  color: var(--accent-red);
  background: rgba(244, 67, 54, 0.12);
}
</style>
