import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import ProcessingSpinner from '../ProcessingSpinner.vue';

describe('ProcessingSpinner', () => {
  it('渲染画布 SVG', () => {
    const wrapper = mount(ProcessingSpinner);
    expect(wrapper.find('svg.psp__canvas').exists()).toBe(true);
  });

  it('默认 size=16', () => {
    const wrapper = mount(ProcessingSpinner);
    const canvas = wrapper.find('svg.psp__canvas');
    expect(canvas.attributes('width')).toBe('16');
    expect(canvas.attributes('height')).toBe('16');
  });

  it('自定义 size=32', () => {
    const wrapper = mount(ProcessingSpinner, {
      props: { size: 32 },
    });
    const canvas = wrapper.find('svg.psp__canvas');
    expect(canvas.attributes('width')).toBe('32');
    expect(canvas.attributes('height')).toBe('32');
  });

  it('viewBox 为 0 0 8 8', () => {
    const wrapper = mount(ProcessingSpinner);
    expect(wrapper.find('svg.psp__canvas').attributes('viewBox')).toBe('0 0 8 8');
  });

  it('渲染 4 帧 <use> 元素', () => {
    const wrapper = mount(ProcessingSpinner);
    const uses = wrapper.findAll('svg.psp__canvas use');
    expect(uses).toHaveLength(4);
  });

  it('4 个 use 元素帧索引为 0-3', () => {
    const wrapper = mount(ProcessingSpinner);
    const uses = wrapper.findAll('svg.psp__canvas use');
    const frameIndices = uses.map((u) => u.attributes('data-frame'));
    expect(frameIndices).toEqual(['0', '1', '2', '3']);
  });

  it('SVG 雪碧图有 4 个 symbol', () => {
    const wrapper = mount(ProcessingSpinner);
    const symbols = wrapper.findAll('symbol');
    expect(symbols).toHaveLength(4);
  });

  it('color prop 以 style color 传递', () => {
    const wrapper = mount(ProcessingSpinner, {
      props: { color: '#00ff00' },
    });
    const style = wrapper.find('.psp').attributes('style') ?? '';
    // jsdom 会将 #00ff00 规范化为 rgb(0, 255, 0)，两种形式均接受
    expect(style.includes('color: #00ff00') || style.includes('color: rgb(0, 255, 0)')).toBe(true);
  });

  it('duration prop 以 --psp-dur CSS 变量传递', () => {
    const wrapper = mount(ProcessingSpinner, {
      props: { duration: 800 },
    });
    expect(wrapper.find('.psp').attributes('style')).toContain('--psp-dur: 800ms');
  });

  it('根元素有 psp 类名', () => {
    const wrapper = mount(ProcessingSpinner);
    expect(wrapper.find('.psp').exists()).toBe(true);
  });

  it('defs SVG 存在', () => {
    const wrapper = mount(ProcessingSpinner);
    expect(wrapper.find('svg.psp__defs').exists()).toBe(true);
  });
});
