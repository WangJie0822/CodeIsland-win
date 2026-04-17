import { describe, expect, it, vi, beforeEach } from "vitest";
import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";

vi.mock("@/lib/tauri", () => ({
  invokeGetSessions: vi.fn(),
  invokeApprovePermission: vi.fn(async () => true),
  invokeDenyPermission: vi.fn(async () => true),
  invokeSendToTerminal: vi.fn(async () => true),
  onSessionsUpdated: vi.fn(async () => () => {}),
}));

import SessionCard from "./SessionCard.vue";
import type { SessionSummary } from "@/types/generated";

function makeSession(partial: Partial<SessionSummary> = {}): SessionSummary {
  return {
    session_id: "s-1",
    project_name: "demo",
    cwd: "C:/tmp/demo",
    phase: "idle",
    needs_attention: false,
    last_message: null,
    tool_name: null,
    tool_input: null,
    ...partial,
  } as SessionSummary;
}

describe("SessionCard", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("渲染项目名与 cwd", () => {
    const wrapper = mount(SessionCard, { props: { session: makeSession() } });
    expect(wrapper.text()).toContain("demo");
    expect(wrapper.text()).toContain("C:/tmp/demo");
  });

  it("phase 为 waitingForApproval 时显示 Allow / Deny", () => {
    const wrapper = mount(SessionCard, {
      props: {
        session: makeSession({
          phase: "waitingForApproval",
          needs_attention: true,
          tool_name: "Bash",
          tool_input: { command: "dir" },
        }),
      },
    });
    expect(wrapper.text()).toContain("Allow");
    expect(wrapper.text()).toContain("Deny");
  });

  it("phase 为 processing 时不显示审批按钮", () => {
    const wrapper = mount(SessionCard, {
      props: { session: makeSession({ phase: "processing", tool_name: "Read" }) },
    });
    expect(wrapper.text()).not.toContain("Allow");
  });
});
