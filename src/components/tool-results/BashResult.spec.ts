import { describe, it, expect } from "vitest";
import { mount } from "@vue/test-utils";
import BashResult from "./BashResult.vue";

describe("BashResult", () => {
  const input = { command: "ls -la /tmp" };

  it("pending 状态显示骨架", () => {
    const w = mount(BashResult, { props: { toolInput: input, toolStatus: "pending" } });
    expect(w.find(".skeleton-line").exists()).toBe(true);
  });

  it("success 状态显示命令和 exit 0", () => {
    const output = { stdout: "file1\nfile2\nfile3", stderr: "", exit_code: 0 };
    const w = mount(BashResult, {
      props: { toolInput: input, toolOutput: output, toolStatus: "success" },
    });
    expect(w.find(".cmd-text").text()).toContain("ls -la /tmp");
    expect(w.find(".exit-code.ok").text()).toContain("exit 0");
    expect(w.text()).toContain("file1");
  });

  it("exit_code 非 0 显示 fail 样式", () => {
    const output = { stdout: "", stderr: "not found", exit_code: 1 };
    const w = mount(BashResult, {
      props: { toolInput: input, toolOutput: output, toolStatus: "success" },
    });
    expect(w.find(".exit-code.fail").exists()).toBe(true);
  });

  it("error 状态显示错误条", () => {
    const w = mount(BashResult, {
      props: { toolInput: input, toolOutput: "permission denied", toolStatus: "error" },
    });
    expect(w.find(".error-bar").exists()).toBe(true);
  });

  it("长命令截断到 80 字符", () => {
    const longCmd = { command: "a".repeat(100) };
    const w = mount(BashResult, { props: { toolInput: longCmd, toolStatus: "success" } });
    const cmdText = w.find(".cmd-text").text();
    expect(cmdText.length).toBeLessThanOrEqual(82); // 80 + "…"
  });

  it("stdout 超 10 行显示截断提示", () => {
    const lines = Array.from({ length: 15 }, (_, i) => `line${i}`).join("\n");
    const output = { stdout: lines, exit_code: 0 };
    const w = mount(BashResult, {
      props: { toolInput: input, toolOutput: output, toolStatus: "success" },
    });
    expect(w.find(".more-hint").exists()).toBe(true);
  });
});
