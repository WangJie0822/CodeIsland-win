import { describe, expect, it, vi, beforeEach } from "vitest";
import { createPinia, setActivePinia } from "pinia";

vi.mock("@/lib/tauri", () => ({
  invokeGetSessions: vi.fn(async () => [
    {
      session_id: "s-1",
      project_name: "demo",
      cwd: "C:\\\\tmp\\\\demo",
      phase: "waitingForApproval",
      needs_attention: true,
      last_message: null,
      tool_name: "Bash",
      tool_input: { command: "dir" },
    },
  ]),
  invokeApprovePermission: vi.fn(async () => true),
  invokeDenyPermission: vi.fn(async () => true),
  invokeSendToTerminal: vi.fn(async () => true),
  onSessionsUpdated: vi.fn(async () => () => {}),
}));

import { useSessionsStore } from "./sessions";

describe("useSessionsStore", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("refresh() 从 invoke 加载会话", async () => {
    const store = useSessionsStore();
    expect(store.list).toEqual([]);
    await store.refresh();
    expect(store.list.length).toBe(1);
    expect(store.list[0].session_id).toBe("s-1");
  });

  it("hasAttention 反映 needs_attention", async () => {
    const store = useSessionsStore();
    await store.refresh();
    expect(store.hasAttention).toBe(true);
  });

  it("overallStatus 为 attention 当存在 waitingForApproval", async () => {
    const store = useSessionsStore();
    await store.refresh();
    expect(store.overallStatus).toBe("attention");
  });
});
