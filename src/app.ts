import "gchessboard";
import { effect } from "@preact/signals-core";
import { currentGameId, headers, moves, gameResult, currentMoveIndex } from "./state";
import { syncBoardMotionPreference } from "./ui/board";
import { hideToast } from "./ui/toast";
import { openShareDialog, switchShareTab, copyToClipboard, downloadPgn } from "./ui/share";
import { renderHeaders } from "./ui/headers";
import { renderGameResult } from "./ui/result";
import { renderMovesList, selectMove } from "./ui/moves";
import { initAppTabs } from "./ui/tabs";
import { initAddGame } from "./ui/add-game";
import { computeGameTitle } from "./utils";
import { fetchGamesList, fetchGameHeaders, fetchGameMoves } from "./api";

const reducedMotionQuery = window.matchMedia("(prefers-reduced-motion: reduce)");

const gameSelectEl = document.querySelector<HTMLSelectElement>("#game-select");
const shareBtn = document.querySelector<HTMLButtonElement>("#share-btn");
const shareDialogEl = document.querySelector<HTMLDialogElement>("#share-dialog");
const closeShareDialogBtn = document.querySelector<HTMLButtonElement>("#close-share-dialog-btn");
const tabFenBtn = document.querySelector<HTMLButtonElement>("#tab-fen");
const tabPgnBtn = document.querySelector<HTMLButtonElement>("#tab-pgn");
const dialogCopyFenBtn = document.querySelector<HTMLButtonElement>("#dialog-copy-fen-btn");
const dialogCopyPgnBtn = document.querySelector<HTMLButtonElement>("#dialog-copy-pgn-btn");
const dialogDownloadPgnBtn = document.querySelector<HTMLButtonElement>("#dialog-download-pgn-btn");
const toastCloseBtn = document.querySelector<HTMLButtonElement>("#toast-close-btn");
const startPosBtn = document.querySelector<HTMLButtonElement>("#start-pos-btn");
const prevMoveBtn = document.querySelector<HTMLButtonElement>("#prev-move");
const nextMoveBtn = document.querySelector<HTMLButtonElement>("#next-move");

async function loadGame(gameId: string): Promise<void> {
  currentGameId.value = gameId;
  if (gameSelectEl && gameSelectEl.value !== gameId) {
    gameSelectEl.value = gameId;
  }
  const url = new URL(window.location.href);
  if (gameId === "immortal") {
    url.searchParams.delete("game");
  } else {
    url.searchParams.set("game", gameId);
  }
  window.history.replaceState({}, "", url.toString());

  const [fetchedHeaders, fetchedMoves] = await Promise.all([
    fetchGameHeaders(gameId),
    fetchGameMoves(gameId),
  ]);

  headers.value = fetchedHeaders;
  moves.value = fetchedMoves;
  gameResult.value = fetchedHeaders.Result;

  await selectMove(0);
}

async function ensureGameSelectOptions(): Promise<void> {
  if (!gameSelectEl) {
    return;
  }
  try {
    const games = await fetchGamesList();
    gameSelectEl.innerHTML = "";
    for (const g of games) {
      const opt = document.createElement("option");
      opt.value = g.id;
      opt.textContent = g.title;
      gameSelectEl.appendChild(opt);
    }
  } catch {
    // keep fallback options
  }
}

export async function initApp(): Promise<void> {
  initAppTabs();
  void initAddGame((saved) => {
    if (!gameSelectEl) return;
    const opt = document.createElement("option");
    opt.value = saved.id;
    opt.textContent = computeGameTitle(saved.headers);
    gameSelectEl.appendChild(opt);
  });
  syncBoardMotionPreference();
  reducedMotionQuery.addEventListener("change", syncBoardMotionPreference);

  await ensureGameSelectOptions();

  const urlParams = new URLSearchParams(window.location.search);
  const initialGame = urlParams.get("game") || "immortal";

  gameSelectEl?.addEventListener("change", () => {
    if (!gameSelectEl) return;
    const selectedGame = gameSelectEl.value;
    gameSelectEl.blur();
    void loadGame(selectedGame).then(() => {
      startPosBtn?.focus({ preventScroll: true });
    });
  });

  startPosBtn?.addEventListener("click", () => {
    void selectMove(0);
  });

  prevMoveBtn?.addEventListener("click", () => {
    if (prevMoveBtn.getAttribute("aria-disabled") === "true") return;
    void selectMove(currentMoveIndex.value - 1);
  });

  nextMoveBtn?.addEventListener("click", () => {
    if (nextMoveBtn.getAttribute("aria-disabled") === "true") return;
    void selectMove(currentMoveIndex.value + 1);
  });

  shareBtn?.addEventListener("click", () => {
    void openShareDialog("pgn");
  });

  closeShareDialogBtn?.addEventListener("click", () => {
    shareDialogEl?.close();
  });

  toastCloseBtn?.addEventListener("click", () => {
    hideToast();
  });

  tabFenBtn?.addEventListener("click", () => switchShareTab("fen"));
  tabPgnBtn?.addEventListener("click", () => switchShareTab("pgn"));

  dialogCopyFenBtn?.addEventListener("click", async () => {
    if (document.querySelector<HTMLTextAreaElement>("#share-fen-text")) {
      await copyToClipboard((document.querySelector<HTMLTextAreaElement>("#share-fen-text") as HTMLTextAreaElement).value, "FEN");
    }
  });

  dialogCopyPgnBtn?.addEventListener("click", async () => {
    if (document.querySelector<HTMLTextAreaElement>("#share-pgn-text")) {
      await copyToClipboard((document.querySelector<HTMLTextAreaElement>("#share-pgn-text") as HTMLTextAreaElement).value, "PGN");
    }
  });

  dialogDownloadPgnBtn?.addEventListener("click", async () => {
    await downloadPgn();
  });

  shareDialogEl?.addEventListener("click", (event) => {
    if (event.target === shareDialogEl) {
      shareDialogEl.close();
    }
  });

  window.addEventListener("keydown", (event) => {
    if (
      event.target instanceof HTMLElement &&
      (event.target.closest("g-chess-board") ||
        event.target.closest("select") ||
        event.target.closest("dialog") ||
        event.target.closest("textarea"))
    ) {
      return;
    }
    if (event.key === "ArrowLeft") {
      event.preventDefault();
      void selectMove(currentMoveIndex.value - 1);
    } else if (event.key === "ArrowRight") {
      event.preventDefault();
      void selectMove(currentMoveIndex.value + 1);
    }
  });

  window.addEventListener("popstate", () => {
    const game = new URLSearchParams(window.location.search).get("game") || "immortal";
    if (game !== currentGameId.value) {
      void loadGame(game);
    }
  });

  await loadGame(initialGame);
}

// Effects
effect(() => renderHeaders());
effect(() => renderMovesList());
effect(() => renderGameResult());

initApp();
