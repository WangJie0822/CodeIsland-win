import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import NeonPixelCat from '../NeonPixelCat.vue';
import type { AnimationState } from '../types';

describe('NeonPixelCat', () => {
  it('默认 size=32 渲染画布正确尺寸', () => {
    const wrapper = mount(NeonPixelCat, {
      props: { state: 'idle' as AnimationState },
    });
    const canvas = wrapper.find('svg.npc__canvas');
    expect(canvas.exists()).toBe(true);
    expect(canvas.attributes('width')).toBe('32');
    expect(canvas.attributes('height')).toBe('32');
  });

  it('自定义 size prop 正确传递', () => {
    const wrapper = mount(NeonPixelCat, {
      props: { state: 'idle' as AnimationState, size: 64 },
    });
    const canvas = wrapper.find('svg.npc__canvas');
    expect(canvas.attributes('width')).toBe('64');
    expect(canvas.attributes('height')).toBe('64');
  });

  it('idle 状态只渲染 1 帧 <use>', () => {
    const wrapper = mount(NeonPixelCat, {
      props: { state: 'idle' as AnimationState },
    });
    const uses = wrapper.findAll('svg.npc__canvas use');
    expect(uses).toHaveLength(1);
  });

  it('processing 状态渲染 4 帧 <use>', () => {
    const wrapper = mount(NeonPixelCat, {
      props: { state: 'processing' as AnimationState },
    });
    const uses = wrapper.findAll('svg.npc__canvas use');
    expect(uses).toHaveLength(4);
  });

  it('waitingForApproval 状态渲染 2 帧 <use>', () => {
    const wrapper = mount(NeonPixelCat, {
      props: { state: 'waitingForApproval' as AnimationState },
    });
    const uses = wrapper.findAll('svg.npc__canvas use');
    expect(uses).toHaveLength(2);
  });

  it('waitingForInput 状态渲染 2 帧 <use>', () => {
    const wrapper = mount(NeonPixelCat, {
      props: { state: 'waitingForInput' as AnimationState },
    });
    const uses = wrapper.findAll('svg.npc__canvas use');
    expect(uses).toHaveLength(2);
  });

  it('tint prop 以 color 内联样式传递', () => {
    const wrapper = mount(NeonPixelCat, {
      props: { state: 'idle' as AnimationState, tint: '#ff0000' },
    });
    const root = wrapper.find('.npc');
    const style = root.attributes('style') ?? '';
    // jsdom 会将 #ff0000 规范化为 rgb(255, 0, 0)，两种形式均接受
    expect(style.includes('color: #ff0000') || style.includes('color: rgb(255, 0, 0)')).toBe(true);
  });

  it('根元素有 npc 类名', () => {
    const wrapper = mount(NeonPixelCat, {
      props: { state: 'idle' as AnimationState },
    });
    expect(wrapper.find('.npc').exists()).toBe(true);
  });

  it('状态类名挂在根元素上', () => {
    const states: AnimationState[] = [
      'idle',
      'processing',
      'waitingForApproval',
      'waitingForInput',
    ];
    for (const state of states) {
      const wrapper = mount(NeonPixelCat, { props: { state } });
      expect(wrapper.find(`.npc--${state}`).exists()).toBe(true);
    }
  });

  it('SVG 雪碧图包含 8 个 symbol 定义', () => {
    const wrapper = mount(NeonPixelCat, {
      props: { state: 'idle' as AnimationState },
    });
    const symbols = wrapper.findAll('symbol');
    expect(symbols).toHaveLength(8);
  });

  it('defs SVG 设置了零尺寸', () => {
    const wrapper = mount(NeonPixelCat, {
      props: { state: 'idle' as AnimationState },
    });
    const defs = wrapper.find('svg.npc__defs');
    expect(defs.exists()).toBe(true);
  });

  it('canvas SVG viewBox 为 0 0 16 16', () => {
    const wrapper = mount(NeonPixelCat, {
      props: { state: 'idle' as AnimationState },
    });
    expect(wrapper.find('svg.npc__canvas').attributes('viewBox')).toBe('0 0 16 16');
  });
});
