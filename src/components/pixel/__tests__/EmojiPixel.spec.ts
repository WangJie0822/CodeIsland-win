import { describe, it, expect, beforeEach, vi } from 'vitest';
import { mount } from '@vue/test-utils';
import EmojiPixel from '../EmojiPixel.vue';

/**
 * jsdom 无 canvas 2D context，需要 mock getContext / fillText / getImageData。
 * 返回一个 4×4 实色区域（前 4 格非透明）。
 */
function mockCanvas() {
  const mockGetImageData = vi.fn(() => {
    const data = new Uint8ClampedArray(16 * 16 * 4).fill(0);
    // 前 4 个像素设为可见红色
    for (let i = 0; i < 4; i++) {
      data[i * 4 + 0] = 255; // r
      data[i * 4 + 1] = 0;   // g
      data[i * 4 + 2] = 0;   // b
      data[i * 4 + 3] = 255; // a
    }
    return { data };
  });

  const mockCtx = {
    clearRect: vi.fn(),
    fillText: vi.fn(),
    getImageData: mockGetImageData,
    font: '',
    textAlign: '',
    textBaseline: '',
  };

  vi.spyOn(HTMLCanvasElement.prototype, 'getContext').mockReturnValue(mockCtx as unknown as CanvasRenderingContext2D);
}

describe('EmojiPixel', () => {
  beforeEach(() => {
    mockCanvas();
  });

  it('渲染 SVG 元素', async () => {
    const wrapper = mount(EmojiPixel, {
      props: { emoji: '🐱' },
    });
    await wrapper.vm.$nextTick();
    expect(wrapper.find('svg.emoji-pixel').exists()).toBe(true);
  });

  it('默认 scale=2 时 SVG 尺寸为 32×32', async () => {
    const wrapper = mount(EmojiPixel, {
      props: { emoji: '🐱' },
    });
    await wrapper.vm.$nextTick();
    const svg = wrapper.find('svg');
    expect(svg.attributes('width')).toBe('32');
    expect(svg.attributes('height')).toBe('32');
  });

  it('自定义 scale=3 时 SVG 尺寸为 48×48', async () => {
    const wrapper = mount(EmojiPixel, {
      props: { emoji: '🐱', scale: 3 },
    });
    await wrapper.vm.$nextTick();
    const svg = wrapper.find('svg');
    expect(svg.attributes('width')).toBe('48');
    expect(svg.attributes('height')).toBe('48');
  });

  it('viewBox 为 0 0 16 16', async () => {
    const wrapper = mount(EmojiPixel, {
      props: { emoji: '🐱' },
    });
    await wrapper.vm.$nextTick();
    expect(wrapper.find('svg').attributes('viewBox')).toBe('0 0 16 16');
  });

  it('非透明像素渲染为 <rect>', async () => {
    const wrapper = mount(EmojiPixel, {
      props: { emoji: '🐱' },
    });
    await wrapper.vm.$nextTick();
    const rects = wrapper.findAll('rect');
    // mock 返回 4 个非透明像素
    expect(rects).toHaveLength(4);
  });

  it('aria-label 等于 emoji prop', async () => {
    const wrapper = mount(EmojiPixel, {
      props: { emoji: '🐱' },
    });
    await wrapper.vm.$nextTick();
    expect(wrapper.find('svg').attributes('aria-label')).toBe('🐱');
  });

  it('emoji prop 变化时重新栅格化', async () => {
    const wrapper = mount(EmojiPixel, {
      props: { emoji: '🐱' },
    });
    await wrapper.vm.$nextTick();

    await wrapper.setProps({ emoji: '🐶' });
    await wrapper.vm.$nextTick();

    expect(wrapper.find('svg').attributes('aria-label')).toBe('🐶');
  });
});
