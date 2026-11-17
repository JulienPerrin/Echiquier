import { syncAddGameTabTitle } from "./add-game";
import { renderHeaders } from "./headers";

export type AppTab = "view" | "add";

export function switchAppTab(tab: AppTab): void {
  const isView = tab === "view";
  const viewTab = document.querySelector<HTMLButtonElement>("#app-tab-view");
  const addTab = document.querySelector<HTMLButtonElement>("#app-tab-add");
  const viewPanel = document.querySelector<HTMLElement>("#panel-view-game");
  const addPanel = document.querySelector<HTMLElement>("#panel-add-game");

  if (viewTab) {
    viewTab.setAttribute("aria-selected", String(isView));
    viewTab.tabIndex = isView ? 0 : -1;
  }
  if (addTab) {
    addTab.setAttribute("aria-selected", String(!isView));
    addTab.tabIndex = !isView ? 0 : -1;
  }
  if (viewPanel) {
    viewPanel.hidden = !isView;
  }
  if (addPanel) {
    addPanel.hidden = isView;
  }
  if (isView) {
    renderHeaders();
  } else {
    syncAddGameTabTitle();
  }
}

export function initAppTabs(): void {
  const tablist = document.querySelector<HTMLElement>("#app-tablist");
  const viewTab = document.querySelector<HTMLButtonElement>("#app-tab-view");
  const addTab = document.querySelector<HTMLButtonElement>("#app-tab-add");

  viewTab?.addEventListener("click", () => switchAppTab("view"));
  addTab?.addEventListener("click", () => switchAppTab("add"));

  tablist?.addEventListener("keydown", (event) => {
    if (event.key !== "ArrowLeft" && event.key !== "ArrowRight") {
      return;
    }
    event.preventDefault();
    const targetTab: AppTab = document.activeElement === viewTab ? "add" : "view";
    switchAppTab(targetTab);
    const nextBtn = targetTab === "view" ? viewTab : addTab;
    nextBtn?.focus();
  });
}
