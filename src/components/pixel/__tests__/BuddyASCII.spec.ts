import { describe, it, expect, vi, afterEach } from 'vitest';
import { mount } from '@vue/test-utils';
import BuddyASCII from '../BuddyASCII.vue';

describe('BuddyASCII', () => {
  afterEach(() => {
    vi.useRealTimers();
  });

  const singleFrame = ['  /\\_/\\  \n ( o.o ) \n  > ^ <  '];
  const multiFrame = [
    '  /\\_/\\  \n ( o.o ) \n  > ^ <  ',
    '  /\\_/\\  \n ( -.- ) \n  > ^ <  ',
  ];

  it('渲染 <pre> 元素', () => {
    const wrapper = mount(BuddyASCII, {
      props: { frames: singleFrame },
    });
    expect(wrapper.find('pre.buddy-ascii').exists()).toBe(true);
  });

  it('单帧内容正确渲染', () => {
    const wrapper = mount(BuddyASCII, {
      props: { frames: singleFrame },
    });
    const text = wrapper.find('pre').text();
    expect(text).toContain('/\\_/\\');
  });

  it('size prop 设置 font-size 样式', () => {
    const wrapper = mount(BuddyASCII, {
      props: { frames: singleFrame, size: 12 },
    });
    expect(wrapper.find('pre').attributes('style')).toContain('font-size: 12px');
  });

  it('默认 size=10', () => {
    const wrapper = mount(BuddyASCII, {
      props: { frames: singleFrame },
    });
    expect(wrapper.find('pre').attributes('style')).toContain('font-size: 10px');
  });

  it('colored=false 时不生成有颜色 span', () => {
    const frame = ['@#*~'];
    const wrapper = mount(BuddyASCII, {
      props: { frames: [frame[0]], colored: false },
    });
    const coloredSpans = wrapper.findAll('span[style]');
    expect(coloredSpans).toHaveLength(0);
  });

  it('colored=true 时对特殊字符生成有颜色 span', () => {
    // "@" 应映射到 --accent-purple
    const wrapper = mount(BuddyASCII, {
      props: { frames: ['@'], colored: true },
    });
    const coloredSpans = wrapper.findAll('span[style]');
    expect(coloredSpans.length).toBeGreaterThan(0);
    const styles = coloredSpans.map((s) => s.attributes('style'));
    expect(styles.some((s) => s?.includes('--accent-purple'))).toBe(true);
  });

  it('多帧时定时器驱动帧切换（mock timer）', async () => {
    vi.useFakeTimers();
    const wrapper = mount(BuddyASCII, {
      props: { frames: multiFrame },
    });

    // 初始帧 0
    expect(wrapper.find('pre').text()).toContain('o.o');

    // 推进 400ms（FRAME_INTERVAL）
    await vi.advanceTimersByTimeAsync(400);
    await wrapper.vm.$nextTick();

    // 应切换到帧 1
    expect(wrapper.find('pre').text()).toContain('-.-');
  });

  it('pre 元素有 buddy-ascii 类名', () => {
    const wrapper = mount(BuddyASCII, {
      props: { frames: singleFrame },
    });
    expect(wrapper.find('pre').classes()).toContain('buddy-ascii');
  });
});
