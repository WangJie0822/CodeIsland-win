<script setup lang="ts">
/**
 * BuddyCardView — 当前 Buddy 信息卡片
 * 显示：大 ASCII 艺术 + 5 属性条 + 名称 + 描述 + 稀有度色彩
 */
import { computed, onMounted, onUnmounted, ref } from 'vue';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import BuddyASCII from '@/components/pixel/BuddyASCII.vue';
import { useBuddyStore, type Buddy, type Rarity } from '@/stores/buddy';
import { statBarColor, statBarWidth } from '@/utils/buddy-stats';

const buddyStore = useBuddyStore();

/** 解锁 toast */
const unlockedToast = ref<string | null>(null);
let toastTimer: ReturnType<typeof setTimeout> | null = null;
let unlisten: UnlistenFn | null = null;

/** 显示解锁浮层 */
function showUnlockToast(name: string): void {
  unlockedToast.value = `已解锁 ${name}!`;
  if (toastTimer !== null) clearTimeout(toastTimer);
  toastTimer = setTimeout(() => {
    unlockedToast.value = null;
    toastTimer = null;
  }, 3000);
}

onMounted(async () => {
  await buddyStore.load();

  // 监听 buddy 解锁事件
  unlisten = await listen<{ display_name: string }>('codeisland:buddy:unlocked', (event) => {
    showUnlockToast(event.payload.display_name);
    void buddyStore.load();
  });
});

onUnmounted(() => {
  if (unlisten) unlisten();
  if (toastTimer !== null) clearTimeout(toastTimer);
});

const buddy = computed<Buddy | null>(() => buddyStore.current);

// ── 稀有度配色映射 ─────────────────────────────────────────────────────────
const RARITY_COLOR: Record<Rarity, string> = {
  Common: 'var(--text-secondary)',
  Rare: 'var(--accent-blue)',
  Epic: 'var(--accent-purple)',
  Legendary: 'var(--accent-yellow)',
};

const RARITY_LABEL: Record<Rarity, string> = {
  Common: '普通',
  Rare: '稀有',
  Epic: '史诗',
  Legendary: '传说',
};

function rarityColor(rarity: Rarity): string {
  return RARITY_COLOR[rarity] ?? 'var(--text-secondary)';
}

function rarityLabel(rarity: Rarity): string {
  return RARITY_LABEL[rarity] ?? rarity;
}


const STAT_LABELS: Record<string, string> = {
  debug: 'Debug',
  patience: 'Patience',
  chaos: 'Chaos',
  wisdom: 'Wisdom',
  sneak: 'Sneak',
};

/** 当前 buddy 的属性条数据 */
const statEntries = computed(() => {
  if (!buddy.value) return [];
  const s = buddy.value.stats;
  return [
    { key: 'debug', label: STAT_LABELS.debug, value: s.debug },
    { key: 'patience', label: STAT_LABELS.patience, value: s.patience },
    { key: 'chaos', label: STAT_LABELS.chaos, value: s.chaos },
    { key: 'wisdom', label: STAT_LABELS.wisdom, value: s.wisdom },
    { key: 'sneak', label: STAT_LABELS.sneak, value: s.sneak },
  ];
});
</script>

<template>
  <div class="buddy-card" data-testid="buddy-card">
    <!-- 空状态 -->
    <div v-if="!buddy" class="buddy-card__empty">
      加载中...
    </div>

    <template v-else>
      <!-- ASCII 艺术 -->
      <div class="buddy-card__ascii" data-testid="buddy-ascii">
        <BuddyASCII
          :frames="buddy.ascii_art"
          :size="12"
          :colored="true"
        />
      </div>

      <!-- 名称 + 稀有度 -->
      <div class="buddy-card__header">
        <span class="buddy-card__name">{{ buddy.display_name }}</span>
        <span
          class="buddy-card__rarity"
          :style="{ color: rarityColor(buddy.rarity) }"
          :data-rarity="buddy.rarity"
          data-testid="buddy-rarity"
        >{{ rarityLabel(buddy.rarity) }}</span>
      </div>

      <!-- 描述 -->
      <p class="buddy-card__description">{{ buddy.description }}</p>

      <!-- 属性条 -->
      <div class="buddy-card__stats" data-testid="buddy-stats">
        <div
          v-for="stat in statEntries"
          :key="stat.key"
          class="stat-row"
          :data-stat="stat.key"
        >
          <span class="stat-row__label">{{ stat.label }}</span>
          <div class="stat-row__track" aria-label="属性条背景">
            <div
              class="stat-row__fill"
              :style="{
                width: statBarWidth(stat.value),
                backgroundColor: statBarColor(stat.value),
              }"
              :data-value="stat.value"
              data-testid="stat-bar"
            />
          </div>
          <span class="stat-row__value">{{ stat.value }}</span>
        </div>
      </div>
    </template>

    <!-- 解锁 toast -->
    <Transition name="toast-fade">
      <div v-if="unlockedToast" class="buddy-card__toast" data-testid="unlock-toast">
        🎉 {{ unlockedToast }}
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.buddy-card {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: var(--radius-sm);
  position: relative;
  min-width: 200px;
}

.buddy-card__empty {
  color: var(--text-secondary);
  text-align: center;
  padding: 16px;
}

.buddy-card__ascii {
  display: flex;
  justify-content: center;
  padding: 4px 0;
}

.buddy-card__header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 8px;
}

.buddy-card__name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.buddy-card__rarity {
  font-size: 11px;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.buddy-card__description {
  font-size: 11px;
  color: var(--text-secondary);
  line-height: 1.4;
}

/* ── 属性条 ──────────────────────────────────────────── */
.buddy-card__stats {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.stat-row {
  display: flex;
  align-items: center;
  gap: 6px;
}

.stat-row__label {
  width: 60px;
  font-size: 10px;
  color: var(--text-secondary);
  flex-shrink: 0;
}

.stat-row__track {
  flex: 1;
  height: 6px;
  background: var(--text-tertiary);
  border-radius: 3px;
  overflow: hidden;
}

.stat-row__fill {
  height: 100%;
  border-radius: 3px;
  transition: width 0.3s var(--ease-notch);
}

.stat-row__value {
  width: 28px;
  font-size: 10px;
  color: var(--text-secondary);
  text-align: right;
  flex-shrink: 0;
}

/* ── 解锁 Toast ──────────────────────────────────────── */
.buddy-card__toast {
  position: absolute;
  bottom: -40px;
  left: 50%;
  transform: translateX(-50%);
  background: var(--bg-notch);
  color: var(--text-primary);
  padding: 6px 14px;
  border-radius: var(--radius-sm);
  font-size: 12px;
  white-space: nowrap;
  border: 1px solid var(--accent-yellow);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.4);
}

.toast-fade-enter-active,
.toast-fade-leave-active {
  transition: opacity var(--dur-fast);
}

.toast-fade-enter-from,
.toast-fade-leave-to {
  opacity: 0;
}
</style>
