import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import TodoWriteResult from "./TodoWriteResult.vue";

const todos = [
  { id: "1", content: "Implement A", status: "completed" },
  { id: "2", content: "Implement B", status: "in_progress" },
  { id: "3", content: "Implement C", status: "pending" },
];

describe("TodoWriteResult", () => {
  const input = { todos };

  it("pending 状态显示骨架", () => {
    const w = mount(TodoWriteResult, { props: { toolInput: input, toolStatus: "pending" } });
    expect(w.find(".skeleton-line").exists()).toBe(true);
  });

  it("success 状态显示 todo 列表及统计", () => {
    const w = mount(TodoWriteResult, { props: { toolInput: input, toolStatus: "success" } });
    expect(w.find(".stat.done").text()).toContain("1 完成");
    expect(w.find(".stat.wip").text()).toContain("1 进行");
    expect(w.find(".stat.todo").text()).toContain("1 待处理");
    const items = w.findAll(".todo-item");
    expect(items.length).toBe(3);
  });

  it("completed 项显示 ✓ 符号", () => {
    const w = mount(TodoWriteResult, { props: { toolInput: input, toolStatus: "success" } });
    const markers = w.findAll(".todo-marker");
    expect(markers[0].text()).toBe("✓");
    expect(markers[1].text()).toBe("→");
    expect(markers[2].text()).toBe("·");
  });

  it("error 状态显示错误条", () => {
    const w = mount(TodoWriteResult, {
      props: { toolInput: input, toolOutput: "todo write failed", toolStatus: "error" },
    });
    expect(w.find(".error-bar").text()).toContain("todo write failed");
  });
});
