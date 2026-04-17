import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import TaskResult from "./TaskResult.vue";

describe("TaskResult", () => {
  const input = { agent: "B-agent", description: "Implement tool result views" };

  it("pending 状态显示骨架", () => {
    const w = mount(TaskResult, { props: { toolInput: input, toolStatus: "pending" } });
    expect(w.find(".skeleton-line").exists()).toBe(true);
  });

  it("success 状态显示 agent 名、描述、状态", () => {
    const w = mount(TaskResult, { props: { toolInput: input, toolStatus: "success" } });
    expect(w.find(".agent-name").text()).toBe("B-agent");
    expect(w.find(".desc-text").text()).toContain("Implement tool result views");
    expect(w.find(".state-pill").text()).toBe("完成");
  });

  it("running 状态显示运行中", () => {
    const w = mount(TaskResult, { props: { toolInput: input, toolStatus: "running" } });
    expect(w.find(".state-pill").text()).toBe("运行中");
  });

  it("error 状态显示失败和错误条", () => {
    const w = mount(TaskResult, {
      props: { toolInput: input, toolOutput: "subagent failed", toolStatus: "error" },
    });
    expect(w.find(".state-pill").text()).toBe("失败");
    expect(w.find(".error-bar").text()).toContain("subagent failed");
  });
});
