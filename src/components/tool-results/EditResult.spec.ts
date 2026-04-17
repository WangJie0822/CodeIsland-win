import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import EditResult from "./EditResult.vue";

describe("EditResult", () => {
  const singleEdit = {
    file_path: "C:/project/src/utils.ts",
    old_string: "foo",
    new_string: "bar",
  };

  it("pending 状态显示骨架", () => {
    const w = mount(EditResult, { props: { toolInput: singleEdit, toolStatus: "pending" } });
    expect(w.find(".skeleton-line").exists()).toBe(true);
  });

  it("success 状态显示路径和改动段数", () => {
    const w = mount(EditResult, {
      props: { toolInput: singleEdit, toolStatus: "success" },
    });
    expect(w.find(".path-text").text()).toContain("utils.ts");
    expect(w.find(".seg-text").text()).toContain("1 段改动");
  });

  it("MultiEdit 多段改动显示正确段数", () => {
    const multiEdit = {
      file_path: "C:/project/a.ts",
      edits: [
        { old_string: "a", new_string: "A" },
        { old_string: "b", new_string: "B" },
        { old_string: "c", new_string: "C" },
      ],
    };
    const w = mount(EditResult, { props: { toolInput: multiEdit, toolStatus: "success" } });
    expect(w.find(".seg-text").text()).toContain("3 段改动");
  });

  it("展开显示 diff 内容", async () => {
    const w = mount(EditResult, {
      props: {
        toolInput: singleEdit,
        toolOutput: "-foo\n+bar",
        toolStatus: "success",
      },
    });
    await w.find(".expand-btn").trigger("click");
    expect(w.text()).toContain("-foo");
  });

  it("error 状态显示错误条", () => {
    const w = mount(EditResult, {
      props: { toolInput: singleEdit, toolOutput: "edit failed", toolStatus: "error" },
    });
    expect(w.find(".error-bar").text()).toContain("edit failed");
  });
});
