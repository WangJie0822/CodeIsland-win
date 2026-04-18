import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { SessionSummary } from "@/types/generated";

export async function invokeGetSessions(): Promise<SessionSummary[]> {
  return invoke("get_sessions");
}

export async function invokeGetSessionCount(): Promise<number> {
  return invoke("get_session_count");
}

export async function invokeApprovePermission(sessionId: string): Promise<boolean> {
  return invoke("approve_permission", { sessionId });
}

export async function invokeDenyPermission(sessionId: string, reason?: string): Promise<boolean> {
  return invoke("deny_permission", { sessionId, reason });
}

export async function invokeSendToTerminal(sessionId: string, text: string): Promise<boolean> {
  return invoke("send_to_terminal", { sessionId, text });
}

export async function onSessionsUpdated(callback: () => void): Promise<UnlistenFn> {
  return listen("codeisland:sessions:updated", callback);
}

export async function invokeSetIgnoreCursorEvents(enabled: boolean): Promise<void> {
  return invoke("set_ignore_cursor_events", { windowLabel: "island", enabled });
}

export async function invokeSetWindowSize(width: number, height: number): Promise<void> {
  return invoke("set_window_size", { windowLabel: "island", width, height });
}

export async function invokeSetWindowPosition(x: number, y: number): Promise<void> {
  return invoke("set_window_position", { windowLabel: "island", x, y });
}

export async function invokeOpenViewWindow(label: string): Promise<void> {
  const { commands } = await import("@/types/generated");
  const result = await commands.openViewWindow(label);
  if (result.status === "error") {
    throw new Error(result.error);
  }
}

export async function invokeGetWindowPosition(label: string): Promise<{ x: number; y: number }> {
  const { commands } = await import("@/types/generated");
  const result = await commands.getWindowPosition(label);
  if (result.status === "error") {
    throw new Error(result.error);
  }
  return result.data;
}
