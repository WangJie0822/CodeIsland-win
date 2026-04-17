import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import ReadResult from "./ReadResult.vue";

describe("ReadResult", () => {
  const baseInput = { file_path: "C:/Users/wj/project/src/main.ts", offset: 1, limit: 50 };

  it("pending 状态显示骨架", () => {
    const w = mount(ReadResult, { props: { toolInput: baseInput, toolStatus: "pending" } });
    expect(w.find(".skeleton-line").exists()).toBe(true);
  });

  it("success 状态显示路径缩写和行范围", () => {
    const w = mount(ReadResult, {
      props: { toolInput: baseInput, toolOutput: "line1\nline2\nline3", toolStatus: "success" },
    });
    // 路径缩写
    expect(w.find(".path-text").text()).toContain("src/main.ts");
    // 行范围
    expect(w.find(".range-text").text()).toBe("L1-50");
    // 总行数
    expect(w.find(".total-text").text()).toContain("行");
  });

  it("展开后显示内容", async () => {
    const w = mount(ReadResult, {
      props: { toolInput: baseInput, toolOutput: "line1\nline2", toolStatus: "success" },
    });
    await w.find(".expand-btn").trigger("click");
    expect(w.find(".mono-pre").exists()).toBe(true);
    expect(w.text()).toContain("line1");
  });

  it("error 状态显示错误条", () => {
    const w = mount(ReadResult, {
      props: { toolInput: baseInput, toolOutput: "file not found", toolStatus: "error" },
    });
    expect(w.find(".error-bar").text()).toContain("file not found");
  });

  it("无 limit 时行范围显示 L1+", () => {
    const input = { file_path: "a/b.ts", offset: 5 };
    const w = mount(ReadResult, { props: { toolInput: input, toolStatus: "success" } });
    expect(w.find(".range-text").text()).toBe("L5+");
  });
});
