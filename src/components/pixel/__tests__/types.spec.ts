import { describe, it, expect } from 'vitest';
import type { AnimationState } from '../types';

describe('AnimationState 类型', () => {
  it('包含全部 4 个合法值', () => {
    const validStates: AnimationState[] = [
      'idle',
      'processing',
      'waitingForApproval',
      'waitingForInput',
    ];
    expect(validStates).toHaveLength(4);
  });

  it('每个状态都是字符串', () => {
    const states: AnimationState[] = [
      'idle',
      'processing',
      'waitingForApproval',
      'waitingForInput',
    ];
    for (const s of states) {
      expect(typeof s).toBe('string');
    }
  });
});
