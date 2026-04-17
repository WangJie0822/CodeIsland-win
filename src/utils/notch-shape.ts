export interface NotchGeometry {
  width: number;
  height: number;
  topCornerRadius: number;
  bottomCornerRadius: number;
}

export const COLLAPSED: NotchGeometry = {
  width: 220,
  height: 32,
  topCornerRadius: 6,
  bottomCornerRadius: 14,
};

export const EXPANDED: NotchGeometry = {
  width: 760,
  height: 520,
  topCornerRadius: 19,
  bottomCornerRadius: 24,
};

function clamp01(t: number): number {
  if (t <= 0) return 0;
  if (t >= 1) return 1;
  return t;
}

function lerp(a: number, b: number, t: number): number {
  return a + (b - a) * t;
}

export function interpolateGeometry(
  start: NotchGeometry,
  end: NotchGeometry,
  t: number,
): NotchGeometry {
  const k = clamp01(t);
  if (k === 0) return { ...start };
  if (k === 1) return { ...end };
  return {
    width: lerp(start.width, end.width, k),
    height: lerp(start.height, end.height, k),
    topCornerRadius: lerp(start.topCornerRadius, end.topCornerRadius, k),
    bottomCornerRadius: lerp(start.bottomCornerRadius, end.bottomCornerRadius, k),
  };
}

/**
 * 生成虚拟刘海 SVG path。
 *
 * 路径顺时针，从左上起点，依次经过：
 *   顶边 → 右上圆弧（外凸） → 右侧直线 → 右下反向圆弧（内凹） →
 *   底边 → 左下反向圆弧（内凹） → 左侧直线 → 左上圆弧（外凸） → Z
 */
export function computeNotchPath(g: NotchGeometry): string {
  const w = g.width;
  const h = g.height;
  const tr = Math.max(0, Math.min(g.topCornerRadius, Math.min(w / 2, h)));
  const br = Math.max(0, Math.min(g.bottomCornerRadius, Math.min(w / 2, h)));

  const startX = tr;
  const startY = 0;

  const parts: string[] = [];
  parts.push(`M ${startX} ${startY}`);
  parts.push(`L ${w - tr} 0`);
  if (tr > 0) parts.push(`A ${tr} ${tr} 0 0 1 ${w} ${tr}`);
  parts.push(`L ${w} ${h - br}`);
  if (br > 0) parts.push(`A ${br} ${br} 0 0 0 ${w - br} ${h}`);
  parts.push(`L ${br} ${h}`);
  if (br > 0) parts.push(`A ${br} ${br} 0 0 0 0 ${h - br}`);
  parts.push(`L 0 ${tr}`);
  if (tr > 0) parts.push(`A ${tr} ${tr} 0 0 1 ${tr} 0`);
  parts.push(`Z`);

  return parts.join(" ");
}
