const globalToastEl = document.querySelector<HTMLElement>("#global-toast");
const toastMessageEl = document.querySelector<HTMLElement>("#toast-message");

export function showToast(message: string): void {
  if (!globalToastEl) return;
  if (toastMessageEl) {
    toastMessageEl.textContent = message;
  } else {
    globalToastEl.textContent = message;
  }
  if (typeof globalToastEl.showPopover === "function") {
    try {
      globalToastEl.showPopover();
    } catch {
      // ignore
    }
  }
  globalToastEl.classList.add("visible");
}

export function hideToast(): void {
  if (!globalToastEl) return;
  if (typeof globalToastEl.hidePopover === "function") {
    try {
      globalToastEl.hidePopover();
    } catch {
      // ignore
    }
  }
  globalToastEl.classList.remove("visible");
}
