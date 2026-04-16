import { sendToTerminal } from "../lib/events";

export function createAskUserOptions(sessionId: string, toolInput: Record<string, unknown> | null): HTMLElement | null {
  if (!toolInput) return null;
  const options = (toolInput as any).options as string[] | undefined;
  if (!options || options.length === 0) return null;

  const container = document.createElement("div");
  container.className = "askuser-options";

  for (const option of options) {
    const btn = document.createElement("button");
    btn.className = "btn-option";
    btn.textContent = option;
    btn.onclick = async (e) => { e.stopPropagation(); await sendToTerminal(sessionId, option); };
    container.appendChild(btn);
  }
  return container;
}
