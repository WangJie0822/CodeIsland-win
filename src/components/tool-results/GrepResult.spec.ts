import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import GrepResult from "./GrepResult.vue";

describe("GrepResult", () => {
  const input = { pattern: "useState" };

  it("pending 状态显示骨架", () => {
    const w = mount(GrepResult, { props: { toolInput: input, toolStatus: "pending" } });
    expect(w.find(".skeleton-line").exists()).toBe(true);
  });

  it("success 状态显示 pattern 和文件数", () => {
    const output = "src/a.ts:1:useState\nsrc/b.ts:3:useState";
    const w = mount(GrepResult, {
      props: { toolInput: input, toolOutput: output, toolStatus: "success" },
    });
    expect(w.find(".pattern-text").text()).toBe("useState");
    expect(w.find(".count-text").text()).toContain("2 个文件");
  });

  it("显示前 5 个文件路径", () => {
    const files = Array.from({ length: 7 }, (_, i) => `src/file${i}.ts:1:x`).join("\n");
    const w = mount(GrepResult, {
      props: { toolInput: input, toolOutput: files, toolStatus: "success" },
    });
    const items = w.findAll(".file-item");
    expect(items.length).toBe(5);
    expect(w.find(".more-hint").exists()).toBe(true);
  });

  it("error 状态显示错误条", () => {
    const w = mount(GrepResult, {
      props: { toolInput: input, toolOutput: "grep error", toolStatus: "error" },
    });
    expect(w.find(".error-bar").text()).toContain("grep error");
  });
});
