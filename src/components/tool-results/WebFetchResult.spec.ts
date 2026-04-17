import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import WebFetchResult from "./WebFetchResult.vue";

describe("WebFetchResult", () => {
  const input = { url: "https://example.com/api/v1/resource" };

  it("pending 状态显示骨架", () => {
    const w = mount(WebFetchResult, { props: { toolInput: input, toolStatus: "pending" } });
    expect(w.find(".skeleton-line").exists()).toBe(true);
  });

  it("success 状态显示缩短后的 URL", () => {
    const w = mount(WebFetchResult, { props: { toolInput: input, toolStatus: "success" } });
    const urlText = w.find(".url-text").text();
    expect(urlText).toContain("example.com");
    expect(urlText).toContain("resource");
  });

  it("有输出时显示字节数", () => {
    const w = mount(WebFetchResult, {
      props: { toolInput: input, toolOutput: "hello", toolStatus: "success" },
    });
    expect(w.find(".size-text").exists()).toBe(true);
  });

  it("error 状态显示错误条", () => {
    const w = mount(WebFetchResult, {
      props: { toolInput: input, toolOutput: "fetch failed", toolStatus: "error" },
    });
    expect(w.find(".error-bar").text()).toContain("fetch failed");
  });

  it("非标准 URL 也能显示", () => {
    const w = mount(WebFetchResult, {
      props: { toolInput: { url: "not-a-url" }, toolStatus: "success" },
    });
    expect(w.find(".url-text").text()).toContain("not-a-url");
  });
});
