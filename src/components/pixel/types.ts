/**
 * 像素艺术组件动画状态类型
 * 供 A (session-card) / E (buddy) 等 agent 通过 @/components/pixel/types import 使用
 */
export type AnimationState =
  | 'idle'
  | 'processing'
  | 'waitingForApproval'
  | 'waitingForInput';
