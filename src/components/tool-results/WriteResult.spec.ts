import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import WriteResult from "./WriteResult.vue";

describe("WriteResult", () => {
  const input = { file_path: "C:/project/src/new-file.ts", content: "hello world" };

  it("pending 状态显示骨架", () => {
    const w = mount(WriteResult, { props: { toolInput: input, toolStatus: "pending" } });
    expect(w.find(".skeleton-line").exists()).toBe(true);
  });

  it("success 状态显示路径、NEW 徽章和字节数", () => {
    const w = mount(WriteResult, { props: { toolInput: input, toolStatus: "success" } });
    expect(w.find(".new-badge").text()).toBe("NEW");
    expect(w.find(".path-text").text()).toContain("new-file.ts");
    expect(w.find(".size-text").exists()).toBe(true);
  });

  it("error 状态显示错误条", () => {
    const w = mount(WriteResult, {
      props: { toolInput: input, toolOutput: "write failed", toolStatus: "error" },
    });
    expect(w.find(".error-bar").text()).toContain("write failed");
  });
});
