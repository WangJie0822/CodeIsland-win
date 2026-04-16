import { getSessions, type SessionSummary } from "./lib/events";

type Listener = () => void;

class Store {
  sessions: SessionSummary[] = [];
  sessionCount: number = 0;
  isExpanded: boolean = false;
  private listeners: Listener[] = [];

  subscribe(fn: Listener): () => void {
    this.listeners.push(fn);
    return () => { this.listeners = this.listeners.filter((l) => l !== fn); };
  }

  private notify() { this.listeners.forEach((fn) => fn()); }

  async refresh() {
    try {
      this.sessions = await getSessions();
      this.sessionCount = this.sessions.length;
      this.notify();
    } catch (e) { console.error("刷新会话失败:", e); }
  }

  setExpanded(expanded: boolean) { this.isExpanded = expanded; this.notify(); }

  get hasAttention(): boolean { return this.sessions.some((s) => s.needs_attention); }

  get overallStatus(): string {
    if (this.sessions.some((s) => s.phase === "waitingForApproval")) return "attention";
    if (this.sessions.some((s) => s.phase === "processing" || s.phase === "compacting")) return "processing";
    if (this.sessionCount > 0) return "idle";
    return "idle";
  }
}

export const store = new Store();
