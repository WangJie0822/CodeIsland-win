import { describe, it, expect } from "vitest";
import { resolveToolView, GenericResult } from "./index";

describe("resolveToolView registry", () => {
  const knownTools = [
    "Read",
    "Edit",
    "MultiEdit",
    "Write",
    "Bash",
    "Grep",
    "Glob",
    "Task",
    "TodoWrite",
    "WebSearch",
    "WebFetch",
  ];

  it.each(knownTools)("resolveToolView('%s') 返回非空组件", (toolName) => {
    const comp = resolveToolView(toolName);
    expect(comp).toBeTruthy();
  });

  it("未知 tool 返回 GenericResult", () => {
    const comp = resolveToolView("UnknownTool");
    // 两者都是 defineAsyncComponent 返回的对象；验证不为 null 且与已知工具结果有相同形状
    expect(comp).toBeTruthy();
    // GenericResult 本身也是 defineAsyncComponent 包装，结构一致
    expect(comp).toEqual(GenericResult);
  });

  it("已知 tool 不返回 GenericResult", () => {
    const readComp = resolveToolView("Read");
    expect(readComp).not.toEqual(GenericResult);
  });
});
