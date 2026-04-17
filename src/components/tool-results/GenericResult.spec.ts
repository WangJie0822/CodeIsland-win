import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import GenericResult from "./GenericResult.vue";

const baseInput: Record<string, unknown> = { foo: "bar" };

describe("GenericResult", () => {
  it("pending 状态显示骨架", () => {
    const w = mount(GenericResult, {
      props: { toolInput: baseInput, toolStatus: "pending" },
    });
    expect(w.find(".skeleton-line").exists()).toBe(true);
    expect(w.find(".tool-name-badge").exists()).toBe(false);
  });

  it("success 状态显示 tool_name 和输出预览", () => {
    const w = mount(GenericResult, {
      props: {
        toolInput: baseInput,
        toolOutput: "some output text",
        toolStatus: "success",
        toolName: "MyTool",
      },
    });
    expect(w.text()).toContain("MyTool");
    expect(w.text()).toContain("some output text");
  });

  it("error 状态显示错误条", () => {
    const w = mount(GenericResult, {
      props: {
        toolInput: baseInput,
        toolOutput: "something broke",
        toolStatus: "error",
        toolName: "MyTool",
      },
    });
    expect(w.find(".error-bar").exists()).toBe(true);
    expect(w.find(".error-bar").text()).toContain("something broke");
  });

  it("运行状态显示内容（非骨架）", () => {
    const w = mount(GenericResult, {
      props: { toolInput: baseInput, toolStatus: "running", toolName: "X" },
    });
    expect(w.find(".skeleton-line").exists()).toBe(false);
    expect(w.find(".tool-name-badge").exists()).toBe(true);
  });

  it("500 字符截断", () => {
    const longOutput = "x".repeat(600);
    const w = mount(GenericResult, {
      props: {
        toolInput: baseInput,
        toolOutput: longOutput,
        toolStatus: "success",
        toolName: "T",
      },
    });
    const preText = w.find(".mono-pre").text();
    expect(preText.length).toBeLessThanOrEqual(510); // 500 + 省略号
  });
});
