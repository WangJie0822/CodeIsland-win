import { describe, it, expect } from "vitest";
import { router } from "./index";

describe("router", () => {
  it("/unknown-path 重定向到 /island", async () => {
    await router.push("/unknown-path");
    expect(router.currentRoute.value.path).toBe("/island");
  });

  it("/settings 路由匹配成功", async () => {
    await router.push("/settings");
    expect(router.currentRoute.value.path).toBe("/settings");
  });

  it("/buddy /usage /presets /notch-live-edit 均匹配成功", async () => {
    const labels = ["buddy", "usage", "presets", "notch-live-edit"] as const;
    for (const label of labels) {
      await router.push(`/${label}`);
      expect(router.currentRoute.value.path).toBe(`/${label}`);
    }
  });
});
