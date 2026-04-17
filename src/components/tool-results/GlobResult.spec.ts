import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import GlobResult from "./GlobResult.vue";

describe("GlobResult", () => {
  const input = { pattern: "**/*.vue" };

  it("pending 状态显示骨架", () => {
    const w = mount(GlobResult, { props: { toolInput: input, toolStatus: "pending" } });
    expect(w.find(".skeleton-line").exists()).toBe(true);
  });

  it("success 状态显示 glob 和匹配数", () => {
    const output = ["src/A.vue", "src/B.vue", "src/C.vue"];
    const w = mount(GlobResult, {
      props: { toolInput: input, toolOutput: output, toolStatus: "success" },
    });
    expect(w.find(".glob-text").text()).toBe("**/*.vue");
    expect(w.find(".count-text").text()).toContain("3 个匹配");
  });

  it("超 5 个时显示前 5 + 提示", () => {
    const output = Array.from({ length: 8 }, (_, i) => `src/file${i}.vue`);
    const w = mount(GlobResult, {
      props: { toolInput: input, toolOutput: output, toolStatus: "success" },
    });
    expect(w.findAll(".file-item").length).toBe(5);
    expect(w.find(".more-hint").exists()).toBe(true);
  });

  it("error 状态显示错误条", () => {
    const w = mount(GlobResult, {
      props: { toolInput: input, toolOutput: "invalid glob", toolStatus: "error" },
    });
    expect(w.find(".error-bar").text()).toContain("invalid glob");
  });
});
