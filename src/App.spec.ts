import { describe, expect, it } from "vitest";
import { mount } from "@vue/test-utils";
import App from "./App.vue";

describe("App.vue", () => {
  it("渲染根容器", () => {
    const wrapper = mount(App);
    expect(wrapper.find("[data-testid='app-root']").exists()).toBe(true);
  });
});
