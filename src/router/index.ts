import { createRouter, createMemoryHistory, type RouteRecordRaw } from "vue-router";
import NotchView from "@/views/NotchView.vue";
import SettingsView from "@/views/SettingsView.vue";
import BuddyCardView from "@/views/BuddyCardView.vue";
import DailyReportCardView from "@/views/DailyReportCardView.vue";
import LaunchPresetsView from "@/views/LaunchPresetsView.vue";
import NotchLiveEditView from "@/views/NotchLiveEditView.vue";

const routes: RouteRecordRaw[] = [
  { path: "/island", component: NotchView },
  { path: "/settings", component: SettingsView },
  { path: "/buddy", component: BuddyCardView },
  { path: "/usage", component: DailyReportCardView },
  { path: "/presets", component: LaunchPresetsView },
  { path: "/notch-live-edit", component: NotchLiveEditView },
  { path: "/:pathMatch(.*)*", redirect: "/island" },
];

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
});
