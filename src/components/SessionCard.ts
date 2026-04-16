import { type SessionSummary, sendToTerminal } from "../lib/events";
import { createStatusDot } from "./StatusIndicator";
import { createApprovalButtons } from "./ApprovalButtons";
import { createAskUserOptions } from "./AskUserOptions";

export function createSessionCard(session: SessionSummary): HTMLElement {
  const card = document.createElement("div");
  card.className = "session-card";
  card.onclick = () => { sendToTerminal(session.session_id, ""); };

  const header = document.createElement("div");
  header.className = "session-header";

  const dotStatus = session.needs_attention ? "attention"
    : (session.phase === "processing" ? "processing" : "idle");
  header.appendChild(createStatusDot(dotStatus));

  const name = document.createElement("span");
  name.className = "session-name";
  name.textContent = session.project_name;
  header.appendChild(name);

  const cwd = document.createElement("span");
  cwd.className = "session-cwd";
  cwd.textContent = session.cwd;
  header.appendChild(cwd);
  card.appendChild(header);

  const status = document.createElement("div");
  status.className = `session-status${session.needs_attention ? " attention" : ""}`;
  status.textContent = formatStatus(session);
  card.appendChild(status);

  if (session.tool_name && session.tool_input && session.phase === "waitingForApproval") {
    const toolInfo = document.createElement("div");
    toolInfo.className = "tool-info";
    toolInfo.textContent = formatToolInput(session.tool_name, session.tool_input);
    card.appendChild(toolInfo);
  }

  if (session.phase === "waitingForApproval") {
    card.appendChild(createApprovalButtons(session.session_id));
  }

  if (session.tool_name === "AskUserQuestion" || session.tool_name === "AskUser") {
    const options = createAskUserOptions(session.session_id, session.tool_input);
    if (options) card.appendChild(options);
  }

  return card;
}

function formatStatus(session: SessionSummary): string {
  switch (session.phase) {
    case "processing": return `Processing${session.tool_name ? ` (${session.tool_name})` : ""}`;
    case "waitingForApproval": return `Awaiting approval: ${session.tool_name || "tool"}`;
    case "waitingForInput": return session.last_message || "Waiting for input";
    case "compacting": return "Compacting context...";
    case "idle": return session.last_message || "Idle";
    default: return session.phase;
  }
}

function formatToolInput(toolName: string, input: Record<string, unknown>): string {
  const parts: string[] = [`Tool: ${toolName}`];
  for (const [key, value] of Object.entries(input)) {
    const str = typeof value === "string" ? value : JSON.stringify(value);
    parts.push(`${key}: ${str.length > 100 ? str.slice(0, 100) + "..." : str}`);
  }
  return parts.join("\n");
}
