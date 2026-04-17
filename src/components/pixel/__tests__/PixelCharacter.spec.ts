import { describe, it, expect, vi, afterEach } from 'vitest';
import { mount } from '@vue/test-utils';
import PixelCharacter from '../PixelCharacter.vue';
import type { SpriteData } from '../PixelCharacter.vue';

/** 最小 2×2 sprite，2 帧 */
const miniSprite: SpriteData = {
  width: 2,
  height: 2,
  palette: [
    { index: 1, color: 'var(--accent-green)' },
    { index: 2, color: 'var(--accent-red)' },
  ],
  frames: [
    [
      [1, 0],
      [0, 2],
    ],
    [
      [2, 0],
      [0, 1],
    ],
  ],
};

describe('PixelCharacter', () => {
  afterEach(() => {
    vi.useRealTimers();
  });

  it('渲染 SVG 元素', () => {
    const wrapper = mount(PixelCharacter, {
      props: { sprite: miniSprite },
    });
    expect(wrapper.find('svg.pixel-character').exists()).toBe(true);
  });

  it('默认 scale=2 时 SVG 尺寸为 4×4（2×2格*scale2）', () => {
    const wrapper = mount(PixelCharacter, {
      props: { sprite: miniSprite },
    });
    const svg = wrapper.find('svg');
    expect(svg.attributes('width')).toBe('4');
    expect(svg.attributes('height')).toBe('4');
  });

  it('自定义 scale=4 时尺寸翻倍', () => {
    const wrapper = mount(PixelCharacter, {
      props: { sprite: miniSprite, scale: 4 },
    });
    const svg = wrapper.find('svg');
    expect(svg.attributes('width')).toBe('8');
    expect(svg.attributes('height')).toBe('8');
  });

  it('viewBox 基于 sprite 宽高', () => {
    const wrapper = mount(PixelCharacter, {
      props: { sprite: miniSprite },
    });
    expect(wrapper.find('svg').attributes('viewBox')).toBe('0 0 2 2');
  });

  it('帧 0 中非零像素（颜色索引不为0）渲染为 rect', () => {
    const wrapper = mount(PixelCharacter, {
      props: { sprite: miniSprite, paused: true },
    });
    const rects = wrapper.findAll('rect');
    // 帧 0 有 2 个非零像素
    expect(rects).toHaveLength(2);
  });

  it('paused=true 时停在帧 0', async () => {
    vi.useFakeTimers();
    const wrapper = mount(PixelCharacter, {
      props: { sprite: miniSprite, paused: true, frameDuration: 100 },
    });
    await vi.advanceTimersByTimeAsync(300);
    await wrapper.vm.$nextTick();
    // 仍渲染帧 0（2 个 rect）
    expect(wrapper.findAll('rect')).toHaveLength(2);
  });

  it('多帧时定时器驱动切换', async () => {
    vi.useFakeTimers();
    const wrapper = mount(PixelCharacter, {
      props: { sprite: miniSprite, frameDuration: 100 },
    });

    // 帧 0: 像素 (0,0)=1, (1,1)=2 → 2 个 rect
    expect(wrapper.findAll('rect')).toHaveLength(2);

    await vi.advanceTimersByTimeAsync(100);
    await wrapper.vm.$nextTick();

    // 帧 1: 像素 (0,0)=2, (1,1)=1 → 2 个 rect（颜色不同）
    const rects = wrapper.findAll('rect');
    expect(rects).toHaveLength(2);
    // 帧切换后颜色应来自帧 1 的调色板映射
    const fillValues = rects.map((r) => r.attributes('fill'));
    // 帧 0 第一个像素是 index=1 → --accent-green
    // 帧 1 第一个像素是 index=2 → --accent-red
    expect(fillValues[0]).toBe('var(--accent-red)');
  });

  it('label prop 作为 aria-label', () => {
    const wrapper = mount(PixelCharacter, {
      props: { sprite: miniSprite, label: '测试角色' },
    });
    expect(wrapper.find('svg').attributes('aria-label')).toBe('测试角色');
  });

  it('单帧 sprite 不会计时切换', async () => {
    vi.useFakeTimers();
    const singleFrameSprite: SpriteData = {
      width: 2,
      height: 2,
      palette: [{ index: 1, color: 'red' }],
      frames: [[[1, 0], [0, 1]]],
    };
    const wrapper = mount(PixelCharacter, {
      props: { sprite: singleFrameSprite },
    });
    await vi.advanceTimersByTimeAsync(1000);
    await wrapper.vm.$nextTick();
    expect(wrapper.findAll('rect')).toHaveLength(2);
  });
});
