import { createApp } from "vue";
import { createPinia } from "pinia";
import { getCurrentWindow } from "@tauri-apps/api/window";
import App from "./App.vue";
import { router } from "./router";

const KNOWN_LABELS = new Set([
  "island",
  "settings",
  "buddy",
  "usage",
  "presets",
  "notch-live-edit",
]);

export function resolveRoutePath(label: string): string {
  if (!label || !KNOWN_LABELS.has(label)) {
    return "/island";
  }
  return `/${label}`;
}

async function bootstrap(): Promise<void> {
  const win = getCurrentWindow();
  const label = win?.label ?? "";
  const path = resolveRoutePath(label);

  await router.push(path);

  const app = createApp(App);
  app.use(createPinia());
  app.use(router);
  app.mount("#app");
}

// 仅在非测试环境下自动启动
if (import.meta.env.MODE !== "test") {
  void bootstrap();
}
