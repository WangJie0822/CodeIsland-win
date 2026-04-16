import { approvePermission, denyPermission } from "../lib/events";

export function createApprovalButtons(sessionId: string): HTMLElement {
  const row = document.createElement("div");
  row.className = "approval-row";

  const allowBtn = document.createElement("button");
  allowBtn.className = "btn btn-allow";
  allowBtn.textContent = "Allow";
  allowBtn.onclick = async (e) => { e.stopPropagation(); await approvePermission(sessionId); };

  const denyBtn = document.createElement("button");
  denyBtn.className = "btn btn-deny";
  denyBtn.textContent = "Deny";
  denyBtn.onclick = async (e) => { e.stopPropagation(); await denyPermission(sessionId); };

  row.appendChild(allowBtn);
  row.appendChild(denyBtn);
  return row;
}
