export function createStatusDot(status: string): HTMLElement {
  const dot = document.createElement("span");
  dot.className = `status-dot ${status}`;
  return dot;
}
