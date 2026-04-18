import { describe, it, expect, vi, beforeEach } from "vitest";

const pushMock = vi.hoisted(() => vi.fn());

vi.mock("./router", () => ({
  router: {
    push: pushMock,
  },
}));

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: vi.fn(),
}));

import { resolveRoutePath } from "./main";

describe("resolveRoutePath", () => {
  beforeEach(() => {
    pushMock.mockClear();
  });

  it("label 为 'island' 时返回 '/island'", () => {
    expect(resolveRoutePath("island")).toBe("/island");
  });

  it("label 为 'settings' 时返回 '/settings'", () => {
    expect(resolveRoutePath("settings")).toBe("/settings");
  });

  it("label 为 'notch-live-edit' 时返回 '/notch-live-edit'", () => {
    expect(resolveRoutePath("notch-live-edit")).toBe("/notch-live-edit");
  });

  it("空 label 兜底 '/island'", () => {
    expect(resolveRoutePath("")).toBe("/island");
  });

  it("未知 label 兜底 '/island'", () => {
    expect(resolveRoutePath("foobar")).toBe("/island");
  });
});
