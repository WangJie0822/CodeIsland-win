/**
 * Buddy 属性条辅助函数
 * 独立模块，供 BuddyCardView.vue 和 BuddyCardView.spec.ts 共同引用
 */

/**
 * 属性条颜色：< 50 → 绿 / 50-80 → 黄 / > 80 → 红
 */
export function statBarColor(value: number): string {
  if (value > 80) return 'var(--accent-red)';
  if (value >= 50) return 'var(--accent-yellow)';
  return 'var(--accent-green)';
}

/**
 * 属性条宽度百分比字符串（0-100 裁剪）
 */
export function statBarWidth(value: number): string {
  return `${Math.min(100, Math.max(0, value))}%`;
}
