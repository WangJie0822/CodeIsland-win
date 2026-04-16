import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export interface SessionSummary {
  session_id: string;
  project_name: string;
  cwd: string;
  phase: string;
  needs_attention: boolean;
  last_message: string | null;
  tool_name: string | null;
  tool_input: Record<string, unknown> | null;
}

export async function getSessions(): Promise<SessionSummary[]> {
  return invoke("get_sessions");
}

export async function getSessionCount(): Promise<number> {
  return invoke("get_session_count");
}

export async function approvePermission(sessionId: string): Promise<boolean> {
  return invoke("approve_permission", { sessionId });
}

export async function denyPermission(sessionId: string, reason?: string): Promise<boolean> {
  return invoke("deny_permission", { sessionId, reason });
}

export async function sendToTerminal(sessionId: string, text: string): Promise<boolean> {
  return invoke("send_to_terminal", { sessionId, text });
}

export function onSessionsUpdated(callback: () => void): Promise<() => void> {
  return listen("sessions-updated", callback).then((unlisten) => unlisten);
}
