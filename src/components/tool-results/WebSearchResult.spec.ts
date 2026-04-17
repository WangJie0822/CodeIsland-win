import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import WebSearchResult from "./WebSearchResult.vue";

describe("WebSearchResult", () => {
  const input = { query: "tauri window transparent" };

  it("pending 状态显示骨架", () => {
    const w = mount(WebSearchResult, { props: { toolInput: input, toolStatus: "pending" } });
    expect(w.find(".skeleton-line").exists()).toBe(true);
  });

  it("success 状态显示查询词", () => {
    const w = mount(WebSearchResult, { props: { toolInput: input, toolStatus: "success" } });
    expect(w.find(".query-text").text()).toContain("tauri window transparent");
  });

  it("有结果数组时显示结果数", () => {
    const output = [{ title: "r1" }, { title: "r2" }];
    const w = mount(WebSearchResult, {
      props: { toolInput: input, toolOutput: output, toolStatus: "success" },
    });
    expect(w.find(".count-text").text()).toContain("2 个结果");
  });

  it("error 状态显示错误条", () => {
    const w = mount(WebSearchResult, {
      props: { toolInput: input, toolOutput: "search failed", toolStatus: "error" },
    });
    expect(w.find(".error-bar").text()).toContain("search failed");
  });
});
