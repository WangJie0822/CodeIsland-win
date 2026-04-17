import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import NotchView from "./views/NotchView.vue";

const pinia = createPinia();

const app = createApp({
  components: { App, NotchView },
  template: "<App><NotchView /></App>",
});

app.use(pinia);
app.mount("#app");
