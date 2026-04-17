import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { mount } from '@vue/test-utils';
import { createPinia, setActivePinia } from 'pinia';
import BuddyCardView from './BuddyCardView.vue';
import { statBarColor, statBarWidth } from '@/utils/buddy-stats';

// ── 全局 mock ────────────────────────────────────────────────────────────────

// mock @tauri-apps/api/core invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// mock @tauri-apps/api/event listen（防止 DOM/env 报错）
vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
}));

// BuddyASCII 使用 setInterval，jsdom 不需要实际渲染，stub 掉
vi.mock('@/components/pixel/BuddyASCII.vue', () => ({
  default: {
    name: 'BuddyASCII',
    props: ['frames', 'size', 'colored'],
    template: '<pre class="buddy-ascii">{{ frames[0] }}</pre>',
  },
}));

import { invoke } from '@tauri-apps/api/core';
import { useBuddyStore } from '@/stores/buddy';

// ── 测试数据 ─────────────────────────────────────────────────────────────────

const mockBuddyState = {
  current: 'kris',
  roster: [
    {
      id: 'kris',
      name: 'Kris',
      display_name: 'Kris',
      rarity: 'Common' as const,
      ascii_art: [' (•‿•) \n /|=|\\\n  / \\  '],
      description: '你忠诚的初始伙伴。',
      stats: { debug: 40, patience: 80, chaos: 20, wisdom: 50, sneak: 30 },
      unlocked_at: 1700000000,
    },
  ],
  completed_sessions: 5,
};

const mockEpicBuddy = {
  ...mockBuddyState,
  current: 'debug_dragon',
  roster: [
    {
      id: 'debug_dragon',
      name: 'DebugDragon',
      display_name: '调试龙',
      rarity: 'Epic' as const,
      ascii_art: [' <>=<>=<> \n  (O_O)  '],
      description: '喷出的不是火焰，而是精准的 breakpoint。',
      stats: { debug: 95, patience: 65, chaos: 60, wisdom: 80, sneak: 40 },
      unlocked_at: 1700000000,
    },
  ],
  completed_sessions: 55,
};

// ── 辅助函数单元测试 ──────────────────────────────────────────────────────────

describe('statBarColor()', () => {
  it('值 < 50 返回绿色 token', () => {
    expect(statBarColor(0)).toBe('var(--accent-green)');
    expect(statBarColor(49)).toBe('var(--accent-green)');
  });

  it('值 50-80 返回黄色 token', () => {
    expect(statBarColor(50)).toBe('var(--accent-yellow)');
    expect(statBarColor(80)).toBe('var(--accent-yellow)');
  });

  it('值 > 80 返回红色 token', () => {
    expect(statBarColor(81)).toBe('var(--accent-red)');
    expect(statBarColor(100)).toBe('var(--accent-red)');
  });
});

describe('statBarWidth()', () => {
  it('正常值返回百分比字符串', () => {
    expect(statBarWidth(50)).toBe('50%');
    expect(statBarWidth(0)).toBe('0%');
    expect(statBarWidth(100)).toBe('100%');
  });

  it('超出范围时裁剪', () => {
    expect(statBarWidth(150)).toBe('100%');
    expect(statBarWidth(-10)).toBe('0%');
  });
});

// ── 组件渲染测试 ──────────────────────────────────────────────────────────────

describe('BuddyCardView.vue', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.mocked(invoke).mockResolvedValue(mockBuddyState);
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('挂载时调用 get_buddy', async () => {
    mount(BuddyCardView);
    await vi.waitFor(() => {
      expect(invoke).toHaveBeenCalledWith('get_buddy');
    });
  });

  it('加载后渲染当前 buddy 名称', async () => {
    const wrapper = mount(BuddyCardView);
    // 手动注入 store 状态以绕过异步 invoke
    const store = useBuddyStore();
    store.$patch({
      current: mockBuddyState.roster[0],
      roster: mockBuddyState.roster,
      completedSessions: mockBuddyState.completed_sessions,
    });
    await wrapper.vm.$nextTick();

    expect(wrapper.find('.buddy-card__name').text()).toBe('Kris');
  });

  it('渲染稀有度标签（Common）', async () => {
    const wrapper = mount(BuddyCardView);
    const store = useBuddyStore();
    store.$patch({
      current: mockBuddyState.roster[0],
      roster: mockBuddyState.roster,
    });
    await wrapper.vm.$nextTick();

    const rarityEl = wrapper.find('[data-testid="buddy-rarity"]');
    expect(rarityEl.exists()).toBe(true);
    expect(rarityEl.attributes('data-rarity')).toBe('Common');
    // Common 使用 --text-secondary
    expect(rarityEl.attributes('style')).toContain('--text-secondary');
  });

  it('稀有度 Epic 使用 --accent-purple 颜色', async () => {
    vi.mocked(invoke).mockResolvedValue(mockEpicBuddy);
    const wrapper = mount(BuddyCardView);
    const store = useBuddyStore();
    store.$patch({
      current: mockEpicBuddy.roster[0],
      roster: mockEpicBuddy.roster,
    });
    await wrapper.vm.$nextTick();

    const rarityEl = wrapper.find('[data-testid="buddy-rarity"]');
    expect(rarityEl.attributes('style')).toContain('--accent-purple');
  });

  it('渲染 5 个属性条', async () => {
    const wrapper = mount(BuddyCardView);
    const store = useBuddyStore();
    store.$patch({
      current: mockBuddyState.roster[0],
      roster: mockBuddyState.roster,
    });
    await wrapper.vm.$nextTick();

    const bars = wrapper.findAll('[data-testid="stat-bar"]');
    expect(bars).toHaveLength(5);
  });

  it('属性条颜色按区间正确设置', async () => {
    const wrapper = mount(BuddyCardView);
    const store = useBuddyStore();
    // kris: debug=40(绿), patience=80(黄), chaos=20(绿), wisdom=50(黄), sneak=30(绿)
    store.$patch({
      current: mockBuddyState.roster[0],
      roster: mockBuddyState.roster,
    });
    await wrapper.vm.$nextTick();

    const bars = wrapper.findAll('[data-testid="stat-bar"]');
    // debug=40 → 绿
    expect(bars[0].attributes('style')).toContain('--accent-green');
    // patience=80 → 黄（80 属于 50-80）
    expect(bars[1].attributes('style')).toContain('--accent-yellow');
    // chaos=20 → 绿
    expect(bars[2].attributes('style')).toContain('--accent-green');
  });

  it('属性条宽度与属性值匹配', async () => {
    const wrapper = mount(BuddyCardView);
    const store = useBuddyStore();
    store.$patch({
      current: mockBuddyState.roster[0],
      roster: mockBuddyState.roster,
    });
    await wrapper.vm.$nextTick();

    const debugBar = wrapper.find('[data-stat="debug"] [data-testid="stat-bar"]');
    expect(debugBar.attributes('style')).toContain('width: 40%');
  });

  it('buddy 为 null 时显示加载中', () => {
    const wrapper = mount(BuddyCardView);
    // store 默认 current=null
    expect(wrapper.text()).toContain('加载中');
  });

  it('包含 ASCII 艺术区域', async () => {
    const wrapper = mount(BuddyCardView);
    const store = useBuddyStore();
    store.$patch({
      current: mockBuddyState.roster[0],
      roster: mockBuddyState.roster,
    });
    await wrapper.vm.$nextTick();

    expect(wrapper.find('[data-testid="buddy-ascii"]').exists()).toBe(true);
  });

  it('包含描述文本', async () => {
    const wrapper = mount(BuddyCardView);
    const store = useBuddyStore();
    store.$patch({
      current: mockBuddyState.roster[0],
      roster: mockBuddyState.roster,
    });
    await wrapper.vm.$nextTick();

    expect(wrapper.find('.buddy-card__description').text()).toBe('你忠诚的初始伙伴。');
  });

  it('Epic 属性值 95 对应红色条', async () => {
    const wrapper = mount(BuddyCardView);
    const store = useBuddyStore();
    store.$patch({
      current: mockEpicBuddy.roster[0],
      roster: mockEpicBuddy.roster,
    });
    await wrapper.vm.$nextTick();

    // debug=95 → 红
    const debugBar = wrapper.find('[data-stat="debug"] [data-testid="stat-bar"]');
    expect(debugBar.attributes('style')).toContain('--accent-red');
  });
});
