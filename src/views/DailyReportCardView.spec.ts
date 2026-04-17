import { describe, expect, it, vi, beforeEach } from "vitest";
import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import DailyReportCardView from "./DailyReportCardView.vue";
import { useUsageStore } from "@/stores/usage";
import type { UsageReport } from "@/stores/usage";

// Mock store 的 load/refresh，由测试直接设置 report 状态
vi.mock("@/stores/usage", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@/stores/usage")>();
  return {
    ...actual,
    useUsageStore: actual.useUsageStore,
  };
});

// 构造一个标准报告（percent 基于 token/quota 计算）
function makeReport(tokens5h: number, tokens7d: number, now = Date.now()): UsageReport {
  const quota5h = 200_000;
  const quota7d = 5_000_000;
  const resets5h = Math.floor(now / 1000) + 5 * 3600;
  const resets7d = Math.floor(now / 1000) + 7 * 86400;
  return {
    window_5h: {
      tokens: tokens5h,
      quota: quota5h,
      percent: tokens5h / quota5h,
      resets_at: resets5h,
    },
    window_7d: {
      tokens: tokens7d,
      quota: quota7d,
      percent: tokens7d / quota7d,
      resets_at: resets7d,
    },
    last_refresh_ts: Math.floor(now / 1000),
  };
}

describe("DailyReportCardView", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  // ── 空数据态 ────────────────────────────────────────────────────────────

  it("report 为 null 时显示 'No usage data yet'", () => {
    const store = useUsageStore();
    // store.load 设为 no-op，不改变 report
    store.load = vi.fn(async () => {});
    store.report = null;

    const wrapper = mount(DailyReportCardView);
    expect(wrapper.find("[data-testid='empty-state']").exists()).toBe(true);
    expect(wrapper.text()).toContain("No usage data yet");
  });

  it("tokens 均为 0 时显示 'No usage data yet'", () => {
    const store = useUsageStore();
    store.load = vi.fn(async () => {});
    store.report = makeReport(0, 0);

    const wrapper = mount(DailyReportCardView);
    expect(wrapper.find("[data-testid='empty-state']").exists()).toBe(true);
  });

  // ── 进度条颜色区间 ───────────────────────────────────────────────────────

  it("<50% 时进度条颜色为 accent-green", async () => {
    const store = useUsageStore();
    store.load = vi.fn(async () => {});
    // 10% → percent = 0.1
    store.report = makeReport(20_000, 20_000);

    const wrapper = mount(DailyReportCardView);
    const bar5h = wrapper.find("[data-testid='bar-5h']");
    expect(bar5h.attributes("style")).toContain("var(--accent-green)");
  });

  it("50~80% 时进度条颜色为 accent-yellow", async () => {
    const store = useUsageStore();
    store.load = vi.fn(async () => {});
    // 60% → tokens = 200_000 * 0.6 = 120_000
    store.report = makeReport(120_000, 120_000);

    const wrapper = mount(DailyReportCardView);
    const bar5h = wrapper.find("[data-testid='bar-5h']");
    expect(bar5h.attributes("style")).toContain("var(--accent-yellow)");
  });

  it(">80% 时进度条颜色为 accent-red", async () => {
    const store = useUsageStore();
    store.load = vi.fn(async () => {});
    // 90% → tokens = 200_000 * 0.9 = 180_000
    store.report = makeReport(180_000, 180_000);

    const wrapper = mount(DailyReportCardView);
    const bar5h = wrapper.find("[data-testid='bar-5h']");
    expect(bar5h.attributes("style")).toContain("var(--accent-red)");
  });

  // ── 剩余时间格式化 ────────────────────────────────────────────────────────

  it("resets_at 为未来 2h15m 时显示 '2h 15m'", () => {
    const store = useUsageStore();
    store.load = vi.fn(async () => {});

    const now = Date.now();
    const resets = Math.floor(now / 1000) + 2 * 3600 + 15 * 60;
    store.report = {
      window_5h: { tokens: 1000, quota: 200_000, percent: 0.005, resets_at: resets },
      window_7d: { tokens: 1000, quota: 5_000_000, percent: 0.0002, resets_at: resets },
      last_refresh_ts: Math.floor(now / 1000),
    };

    const wrapper = mount(DailyReportCardView);
    // 允许分钟偏差 ±1 min（测试执行时间差）
    const text = wrapper.text();
    expect(text).toMatch(/2h 1[45]m/);
  });

  it("resets_at 为 null 时显示 '未知'", () => {
    const store = useUsageStore();
    store.load = vi.fn(async () => {});
    store.report = {
      window_5h: { tokens: 1000, quota: 200_000, percent: 0.005, resets_at: null },
      window_7d: { tokens: 1000, quota: 5_000_000, percent: 0.0002, resets_at: null },
      last_refresh_ts: Math.floor(Date.now() / 1000),
    };

    const wrapper = mount(DailyReportCardView);
    expect(wrapper.text()).toContain("未知");
  });

  // ── 刷新按钮 ──────────────────────────────────────────────────────────────

  it("点击刷新按钮调用 store.refresh()", async () => {
    const store = useUsageStore();
    store.load = vi.fn(async () => {});
    store.refresh = vi.fn(async () => {});
    store.report = makeReport(1000, 1000);

    const wrapper = mount(DailyReportCardView);
    await wrapper.find("[data-testid='btn-refresh']").trigger("click");
    expect(store.refresh).toHaveBeenCalledTimes(1);
  });

  // ── 桶行渲染 ──────────────────────────────────────────────────────────────

  it("有数据时渲染 5h 和 7d 桶行", () => {
    const store = useUsageStore();
    store.load = vi.fn(async () => {});
    store.report = makeReport(1000, 2000);

    const wrapper = mount(DailyReportCardView);
    expect(wrapper.find("[data-testid='bucket-5h']").exists()).toBe(true);
    expect(wrapper.find("[data-testid='bucket-7d']").exists()).toBe(true);
  });
});
