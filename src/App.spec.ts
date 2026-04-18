import { describe, expect, it } from "vitest";
import { mount } from "@vue/test-utils";
import { createRouter, createMemoryHistory } from "vue-router";
import App from "./App.vue";

describe("App.vue", () => {
  it("渲染 RouterView 根组件", async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes: [{ path: "/", component: { template: "<div>home</div>" } }],
    });
    await router.push("/");
    const wrapper = mount(App, {
      global: { plugins: [router] },
    });
    // App.vue 已切换为 RouterView，data-testid='app-root' 已移除
    expect(wrapper.find("div").exists()).toBe(true);
  });
});
