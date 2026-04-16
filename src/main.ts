import { store } from "./store";
import { onSessionsUpdated } from "./lib/events";
import { initIsland } from "./components/Island";

async function init() {
  console.log("Code Island starting...");
  initIsland();
  await store.refresh();
  await onSessionsUpdated(async () => { await store.refresh(); });
}

init().catch(console.error);
