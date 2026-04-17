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
import ApprovalButtons from "./ApprovalButtons.vue";
import AskUserOptions from "./AskUserOptions.vue";
import SessionCardHeader from "./SessionCardHeader.vue";
import SessionCardMeta from "./SessionCardMeta.vue";
import SessionCardBody from "./SessionCardBody.vue";
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

// ──────────────────────────────────────────────
// SessionCard — 5 phase 状态测试
// ──────────────────────────────────────────────
describe("SessionCard — phase 状态", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("phase=idle: 渲染项目名，无审批按钮", () => {
    const wrapper = mount(SessionCard, { props: { session: makeSession() } });
    // 重构后 cwd 不直接展示（Header 只显示 project_name），仍保留 project_name 检查
    expect(wrapper.text()).toContain("demo");
    expect(wrapper.text()).not.toContain("Allow");
  });

  it("phase=processing: 显示摘要文本、无审批按钮", () => {
    const wrapper = mount(SessionCard, {
      props: {
        session: makeSession({ phase: "processing", tool_name: "Read" }),
      },
    });
    expect(wrapper.text()).toContain("Processing");
    expect(wrapper.text()).not.toContain("Allow");
    expect(wrapper.text()).not.toContain("Deny");
  });

  it("phase=waitingForApproval: 显示 Allow / Deny", () => {
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

  it("phase=waitingForInput: 显示 last_message，无审批按钮", () => {
    const wrapper = mount(SessionCard, {
      props: {
        session: makeSession({
          phase: "waitingForInput",
          last_message: "请输入确认信息",
          tool_name: "AskUser",
          tool_input: { options: [] },
        }),
      },
    });
    // 不显示 Allow/Deny（AskUser 走选项区）
    expect(wrapper.text()).not.toContain("Allow");
  });

  it("phase=compacting: 显示压缩摘要，无审批按钮", () => {
    const wrapper = mount(SessionCard, {
      props: {
        session: makeSession({ phase: "compacting" }),
      },
    });
    expect(wrapper.text()).toContain("压缩上下文");
    expect(wrapper.text()).not.toContain("Allow");
    expect(wrapper.text()).not.toContain("Deny");
  });

  it("phase=ended: 不显示审批按钮", () => {
    const wrapper = mount(SessionCard, {
      props: {
        session: makeSession({ phase: "ended", last_message: "任务完成" }),
      },
    });
    expect(wrapper.text()).not.toContain("Allow");
    expect(wrapper.text()).not.toContain("Deny");
  });
});

// ──────────────────────────────────────────────
// ApprovalButtons 单元测试
// ──────────────────────────────────────────────
describe("ApprovalButtons", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("点击 Allow 调用 approve", async () => {
    const { invokeApprovePermission } = await import("@/lib/tauri");
    const wrapper = mount(ApprovalButtons, {
      props: { sessionId: "s-42" },
    });
    await wrapper.find(".btn-allow").trigger("click");
    expect(invokeApprovePermission).toHaveBeenCalledWith("s-42");
  });

  it("点击 Deny 调用 deny", async () => {
    const { invokeDenyPermission } = await import("@/lib/tauri");
    const wrapper = mount(ApprovalButtons, {
      props: { sessionId: "s-42" },
    });
    await wrapper.find(".btn-deny").trigger("click");
    expect(invokeDenyPermission).toHaveBeenCalledWith("s-42", undefined);
  });

  it("Enter 键触发 allow", async () => {
    const { invokeApprovePermission } = await import("@/lib/tauri");
    vi.mocked(invokeApprovePermission).mockClear();
    mount(ApprovalButtons, { props: { sessionId: "s-kb" } });
    const event = new KeyboardEvent("keydown", { key: "Enter", bubbles: true });
    window.dispatchEvent(event);
    // 等待微任务
    await Promise.resolve();
    expect(invokeApprovePermission).toHaveBeenCalledWith("s-kb");
  });

  it("Escape 键触发 deny", async () => {
    const { invokeDenyPermission } = await import("@/lib/tauri");
    vi.mocked(invokeDenyPermission).mockClear();
    mount(ApprovalButtons, { props: { sessionId: "s-kb2" } });
    const event = new KeyboardEvent("keydown", { key: "Escape", bubbles: true });
    window.dispatchEvent(event);
    await Promise.resolve();
    expect(invokeDenyPermission).toHaveBeenCalledWith("s-kb2", undefined);
  });
});

// ──────────────────────────────────────────────
// AskUserOptions 单元测试
// ──────────────────────────────────────────────
describe("AskUserOptions", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("渲染选项列表", () => {
    const wrapper = mount(AskUserOptions, {
      props: {
        sessionId: "s-ask",
        toolInput: { options: ["是", "否", "取消"] },
      },
    });
    expect(wrapper.text()).toContain("是");
    expect(wrapper.text()).toContain("否");
    expect(wrapper.text()).toContain("取消");
  });

  it("点击选项调用 sendToTerminal", async () => {
    const { invokeSendToTerminal } = await import("@/lib/tauri");
    vi.mocked(invokeSendToTerminal).mockClear();
    const wrapper = mount(AskUserOptions, {
      props: {
        sessionId: "s-ask2",
        toolInput: { options: ["确认", "取消"] },
      },
    });
    await wrapper.findAll(".auo__btn")[0].trigger("click");
    expect(invokeSendToTerminal).toHaveBeenCalledWith("s-ask2", "确认");
  });

  it("toolInput 为 null 时不渲染", () => {
    const wrapper = mount(AskUserOptions, {
      props: { sessionId: "s-ask3", toolInput: null },
    });
    expect(wrapper.find(".auo").exists()).toBe(false);
  });

  it("options 为空数组时不渲染", () => {
    const wrapper = mount(AskUserOptions, {
      props: { sessionId: "s-ask4", toolInput: { options: [] } },
    });
    expect(wrapper.find(".auo").exists()).toBe(false);
  });
});

// ──────────────────────────────────────────────
// SessionCardHeader 单元测试
// ──────────────────────────────────────────────
describe("SessionCardHeader", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("显示项目名", () => {
    const wrapper = mount(SessionCardHeader, {
      props: { session: makeSession({ project_name: "MyProject" }) },
    });
    expect(wrapper.text()).toContain("MyProject");
  });

  it("pixel_cat_enabled=false 时渲染首字母头像", () => {
    const session = { ...makeSession({ project_name: "Alpha" }), pixel_cat_enabled: false };
    const wrapper = mount(SessionCardHeader, { props: { session } });
    expect(wrapper.find(".sch__initial").exists()).toBe(true);
    expect(wrapper.find(".sch__initial").text()).toBe("A");
  });

  it("pixel_cat_enabled=true（默认）时渲染 NeonPixelCat", () => {
    const wrapper = mount(SessionCardHeader, {
      props: { session: makeSession() },
    });
    expect(wrapper.find(".sch__initial").exists()).toBe(false);
    expect(wrapper.find(".npc").exists()).toBe(true);
  });

  it("launcher=cmux 时显示 launcher 标签", () => {
    const session = { ...makeSession(), launcher: "cmux" };
    const wrapper = mount(SessionCardHeader, { props: { session } });
    expect(wrapper.text()).toContain("cmux");
  });

  it("launcher=native（默认）时不显示 launcher 标签", () => {
    const wrapper = mount(SessionCardHeader, { props: { session: makeSession() } });
    expect(wrapper.find(".sch__launcher-badge").exists()).toBe(false);
  });
});

// ──────────────────────────────────────────────
// SessionCardMeta 单元测试
// ──────────────────────────────────────────────
describe("SessionCardMeta", () => {
  it("processing 显示 Processing 摘要", () => {
    const wrapper = mount(SessionCardMeta, {
      props: { session: makeSession({ phase: "processing", tool_name: "Bash" }) },
    });
    expect(wrapper.text()).toContain("Processing");
    expect(wrapper.text()).toContain("Bash");
  });

  it("waitingForApproval 显示等待审批文本", () => {
    const wrapper = mount(SessionCardMeta, {
      props: {
        session: makeSession({ phase: "waitingForApproval", tool_name: "Edit" }),
      },
    });
    expect(wrapper.text()).toContain("等待审批");
    expect(wrapper.text()).toContain("Edit");
  });

  it("idle 且无 last_message 时不渲染", () => {
    const wrapper = mount(SessionCardMeta, {
      props: { session: makeSession({ phase: "idle", last_message: null }) },
    });
    expect(wrapper.find(".scm").exists()).toBe(false);
  });
});

// ──────────────────────────────────────────────
// SessionCardBody 单元测试
// ──────────────────────────────────────────────
describe("SessionCardBody", () => {
  it("有 tool_name 时渲染工具名", () => {
    const wrapper = mount(SessionCardBody, {
      props: {
        session: makeSession({
          phase: "processing",
          tool_name: "Read",
          tool_input: { file_path: "/tmp/foo.txt" },
        }),
      },
    });
    expect(wrapper.find(".scb").exists()).toBe(true);
    expect(wrapper.text()).toContain("Read");
  });

  it("tool_name 为 null 时不渲染", () => {
    const wrapper = mount(SessionCardBody, {
      props: { session: makeSession({ tool_name: null }) },
    });
    expect(wrapper.find(".scb").exists()).toBe(false);
  });
});
