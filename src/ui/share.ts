import { invoke } from "@tauri-apps/api/core";
import { writeText as tauriWriteText } from "@tauri-apps/plugin-clipboard-manager";
import { currentGameId, currentMoveIndex } from "../state";
import { showToast } from "./toast";
import { fetchGameFen, fetchGamePgn } from "../api";
import { isTauri } from "../api";

export async function copyToClipboard(text: string, label: string): Promise<boolean> {
  try {
    if (isTauri) {
      await tauriWriteText(text);
      showToast(`✓ ${label} copied to clipboard!`);
      return true;
    }
    if (navigator.clipboard && typeof navigator.clipboard.writeText === "function") {
      await navigator.clipboard.writeText(text);
      showToast(`✓ ${label} copied to clipboard!`);
      return true;
    }
  } catch {
    // fallback
  }
  return false;
}

export function switchShareTab(tab: "fen" | "pgn"): void {
  const isFen = tab === "fen";
  const tabFenBtn = document.querySelector<HTMLButtonElement>("#tab-fen");
  const tabPgnBtn = document.querySelector<HTMLButtonElement>("#tab-pgn");
  const panelFenEl = document.querySelector<HTMLElement>("#panel-fen");
  const panelPgnEl = document.querySelector<HTMLElement>("#panel-pgn");
  if (tabFenBtn) {
    tabFenBtn.classList.toggle("active", isFen);
    tabFenBtn.setAttribute("aria-selected", String(isFen));
  }
  if (tabPgnBtn) {
    tabPgnBtn.classList.toggle("active", !isFen);
    tabPgnBtn.setAttribute("aria-selected", String(!isFen));
  }
  if (panelFenEl) {
    panelFenEl.hidden = !isFen;
    panelFenEl.classList.toggle("active", isFen);
  }
  if (panelPgnEl) {
    panelPgnEl.hidden = isFen;
    panelPgnEl.classList.toggle("active", !isFen);
  }
}

export async function openShareDialog(activeTab: "fen" | "pgn" = "pgn"): Promise<void> {
  const shareDialogEl = document.querySelector<HTMLDialogElement>("#share-dialog");
  if (!shareDialogEl) return;

  const fen = await fetchGameFen(currentMoveIndex.value, currentGameId.value);
  const shareFenTextEl = document.querySelector<HTMLTextAreaElement>("#share-fen-text");
  if (shareFenTextEl) {
    shareFenTextEl.value = fen;
  }
  const pgn = await fetchGamePgn(currentGameId.value);
  const sharePgnTextEl = document.querySelector<HTMLTextAreaElement>("#share-pgn-text");
  if (sharePgnTextEl) {
    sharePgnTextEl.value = pgn;
  }

  switchShareTab(activeTab);

  if (typeof shareDialogEl.showModal === "function" && !shareDialogEl.open) {
    shareDialogEl.showModal();
  }
}

export async function downloadPgn(): Promise<void> {
  const sharePgnTextEl = document.querySelector<HTMLTextAreaElement>("#share-pgn-text");
  if (!sharePgnTextEl) return;
  if (isTauri) {
    try {
      const savedPath = await invoke<string>("download_pgn", {
        game: currentGameId.value,
        pgn: sharePgnTextEl.value,
      });
      showToast(`✓ Saved to ${savedPath}`);
    } catch (err) {
      showToast(`✕ Failed to save PGN: ${String(err)}`);
    }
    return;
  }
  const blob = new Blob([sharePgnTextEl.value], { type: "application/x-chess-pgn" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `${currentGameId.value}.pgn`;
  a.click();
  URL.revokeObjectURL(url);
  showToast(`✓ Downloaded ${currentGameId.value}.pgn`);
}
