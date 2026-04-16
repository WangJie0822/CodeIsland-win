export function flashBorder(element: HTMLElement) {
  element.classList.add("alert");
  setTimeout(() => element.classList.remove("alert"), 2000);
}
