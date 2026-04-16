import { getCurrentWindow } from "@tauri-apps/api/window";
import { store } from "../store";
import { createSessionCard } from "./SessionCard";
import { createStatusDot } from "./StatusIndicator";

const island = document.getElementById("island")!;
const collapsedEl = document.getElementById("island-collapsed")!;
const expandedEl = document.getElementById("island-expanded")!;
const sessionList = document.getElementById("session-list")!;

let collapseTimer: number | null = null;
let prevHasAttention = false;

export function initIsland() {
  island.addEventListener("mouseenter", () => {
    if (collapseTimer) { clearTimeout(collapseTimer); collapseTimer = null; }
    store.setExpanded(true);
  });

  island.addEventListener("mouseleave", () => {
    collapseTimer = window.setTimeout(() => { store.setExpanded(false); }, 300);
  });

  const expandedHeader = document.getElementById("expanded-header")!;
  const closeBtn = document.createElement("button");
  closeBtn.className = "btn-close";
  closeBtn.textContent = "\u00d7";
  closeBtn.title = "隐藏窗口";
  closeBtn.onclick = async (e) => {
    e.stopPropagation();
    await getCurrentWindow().hide();
  };
  expandedHeader.appendChild(closeBtn);

  store.subscribe(render);
}

function render() {
  const { isExpanded, sessions, sessionCount } = store;

  island.classList.toggle("collapsed", !isExpanded);
  island.classList.toggle("expanded", isExpanded);

  if (store.hasAttention && !prevHasAttention) {
    island.classList.add("alert");
    setTimeout(() => { island.classList.remove("alert"); }, 5000);
  }
  prevHasAttention = store.hasAttention;

  collapsedEl.innerHTML = "";
  collapsedEl.appendChild(createStatusDot(store.overallStatus));

  const countSpan = document.createElement("span");
  countSpan.className = "session-count";
  countSpan.textContent = `${sessionCount} session${sessionCount !== 1 ? "s" : ""}`;
  collapsedEl.appendChild(countSpan);

  const labelSpan = document.createElement("span");
  labelSpan.className = "status-label";
  labelSpan.textContent = store.overallStatus;
  collapsedEl.appendChild(labelSpan);

  sessionList.innerHTML = "";
  if (sessions.length === 0) {
    const empty = document.createElement("div");
    empty.className = "empty-state";
    empty.textContent = "No active sessions";
    sessionList.appendChild(empty);
  } else {
    for (const session of sessions) {
      sessionList.appendChild(createSessionCard(session));
    }
  }

  if (isExpanded) {
    const height = Math.min(island.scrollHeight + 16, window.innerHeight * 0.8);
    island.style.height = `${height}px`;
  } else {
    island.style.height = "";
  }
}
