import { describe, expect, it } from "vitest";
import {
  COLLAPSED,
  EXPANDED,
  computeNotchPath,
  interpolateGeometry,
  type NotchGeometry,
} from "./notch-shape";

describe("interpolateGeometry", () => {
  it("t=0 返回 start", () => {
    expect(interpolateGeometry(COLLAPSED, EXPANDED, 0)).toEqual(COLLAPSED);
  });

  it("t=1 返回 end", () => {
    expect(interpolateGeometry(COLLAPSED, EXPANDED, 1)).toEqual(EXPANDED);
  });

  it("t=0.5 线性插值", () => {
    const mid = interpolateGeometry(COLLAPSED, EXPANDED, 0.5);
    expect(mid.width).toBe((COLLAPSED.width + EXPANDED.width) / 2);
    expect(mid.height).toBe((COLLAPSED.height + EXPANDED.height) / 2);
    expect(mid.topCornerRadius).toBeCloseTo(
      (COLLAPSED.topCornerRadius + EXPANDED.topCornerRadius) / 2,
    );
    expect(mid.bottomCornerRadius).toBeCloseTo(
      (COLLAPSED.bottomCornerRadius + EXPANDED.bottomCornerRadius) / 2,
    );
  });

  it("t 被 clamp 在 [0,1]", () => {
    expect(interpolateGeometry(COLLAPSED, EXPANDED, -0.5)).toEqual(COLLAPSED);
    expect(interpolateGeometry(COLLAPSED, EXPANDED, 1.5)).toEqual(EXPANDED);
  });
});

describe("computeNotchPath", () => {
  it("生成闭合 SVG path，以 M 开头 Z 结尾", () => {
    const path = computeNotchPath(EXPANDED);
    expect(path.startsWith("M")).toBe(true);
    expect(path.trim().endsWith("Z")).toBe(true);
  });

  it("路径含 4 段圆弧", () => {
    const path = computeNotchPath(EXPANDED);
    const arcs = path.match(/A\s/g) ?? [];
    expect(arcs.length).toBe(4);
  });

  it("不同尺寸生成不同路径", () => {
    expect(computeNotchPath(COLLAPSED)).not.toBe(computeNotchPath(EXPANDED));
  });

  it("零圆角退化为矩形（仍闭合）", () => {
    const rect: NotchGeometry = {
      width: 100,
      height: 50,
      topCornerRadius: 0,
      bottomCornerRadius: 0,
    };
    const path = computeNotchPath(rect);
    expect(path.startsWith("M")).toBe(true);
    expect(path.trim().endsWith("Z")).toBe(true);
  });
});
