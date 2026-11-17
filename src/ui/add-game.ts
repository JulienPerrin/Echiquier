import type { GChessBoardElement } from "gchessboard";
import { fetchPositionState, importPgn, type PositionState } from "../api";
import {
  SAVED_GAMES_STORAGE_KEY,
  type SavedGame,
  getSavedGames,
  saveLocalGame,
} from "../storage";
import type { PgnHeaders } from "../types";
import { computeGameTitle } from "../utils";
import { showToast } from "./toast";

export { SAVED_GAMES_STORAGE_KEY, type SavedGame, getSavedGames };
export const START_FEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

const playedCoordMoves: string[] = [];
let sanMoves: string[] = [];
let allowedMoves: Record<string, string[]> = {};
let lastAutoResult: string | null = null;

function updateMoveControls(): void {
  const isInitial = playedCoordMoves.length === 0;
  const undoBtn = document.querySelector<HTMLButtonElement>("#add-undo-btn");
  if (undoBtn) {
    undoBtn.disabled = isInitial;
  }
  const importBtn = document.querySelector<HTMLButtonElement>("#add-import-pgn-btn");
  if (importBtn) {
    importBtn.hidden = !isInitial;
  }
  const saveBtn = document.querySelector<HTMLButtonElement>("#add-save-game-btn");
  if (saveBtn) {
    saveBtn.hidden = isInitial;
  }
}

function createPlayedMoveButton(
  moveText: string,
  moveIndex: number,
  isLast: boolean,
  onSelect?: (index: number) => void
): HTMLButtonElement {
  const btn = document.createElement("button");
  btn.type = "button";
  btn.className = isLast ? "move-btn active" : "move-btn";
  btn.setAttribute("aria-pressed", String(isLast));
  btn.textContent = moveText;
  if (onSelect) {
    btn.addEventListener("click", () => onSelect(moveIndex));
  }
  return btn;
}

export function renderAddGameMoves(
  movesList: string[],
  onSelectMove?: (index: number) => void
): void {
  const listEl = document.querySelector<HTMLElement>("#add-moves-list");
  if (!listEl) return;
  listEl.innerHTML = "";

  if (movesList.length === 0) {
    const emptyItem = document.createElement("li");
    emptyItem.className = "empty-moves";
    emptyItem.textContent = "No moves played yet.";
    listEl.appendChild(emptyItem);
    return;
  }

  for (let i = 0; i < movesList.length; i += 2) {
    const turn = Math.floor(i / 2) + 1;
    const row = document.createElement("li");
    row.className = "move-row";

    const turnSpan = document.createElement("span");
    turnSpan.className = "turn-number";
    turnSpan.setAttribute("aria-hidden", "true");
    turnSpan.textContent = `${turn}.`;
    row.appendChild(turnSpan);

    row.appendChild(
      createPlayedMoveButton(movesList[i], i, i === movesList.length - 1, onSelectMove)
    );

    if (i + 1 < movesList.length) {
      row.appendChild(
        createPlayedMoveButton(
          movesList[i + 1],
          i + 1,
          i + 1 === movesList.length - 1,
          onSelectMove
        )
      );
    }

    listEl.appendChild(row);
  }
}

function readFormHeaders(form: HTMLFormElement): PgnHeaders {
  const data = new FormData(form);
  const read = (key: string): string | undefined => {
    const val = String(data.get(key) ?? "").trim();
    return val === "" ? undefined : val;
  };
  const rawDate = read("date");
  const resultSelect = form.querySelector<HTMLSelectElement>("#add-header-result");
  return {
    Title: read("title"),
    White: read("white"),
    Black: read("black"),
    Event: read("event"),
    Site: read("site"),
    Date: rawDate ? rawDate.replaceAll("-", ".") : undefined,
    Round: read("round"),
    Result: read("result") ?? (resultSelect?.value || undefined),
  };
}

function populateFormHeaders(headers: Record<string, string>): void {
  const setVal = (selector: string, val?: string) => {
    const el = document.querySelector<HTMLInputElement | HTMLSelectElement>(selector);
    if (el && val !== undefined) {
      el.value = val;
    }
  };
  setVal("#add-header-title", headers.Title ?? "");
  setVal("#add-header-white", headers.White ?? "");
  setVal("#add-header-black", headers.Black ?? "");
  setVal("#add-header-event", headers.Event ?? "");
  setVal("#add-header-site", headers.Site ?? "");
  setVal("#add-header-date", headers.Date ? headers.Date.replaceAll(".", "-") : "");
  setVal("#add-header-round", headers.Round ?? "");
  if (headers.Result) {
    setVal("#add-header-result", headers.Result);
  }
}

function applyState(board: GChessBoardElement, state: PositionState): void {
  allowedMoves = state.allowed_moves;
  sanMoves = Array.isArray(state.san_moves) ? state.san_moves : [];
  board.fen = state.fen;
  board.turn = state.turn === "black" ? "black" : "white";
  renderAddGameMoves(sanMoves, (index) => {
    playedCoordMoves.length = index + 1;
    updateMoveControls();
    void syncPositionState(board);
  });
  updateMoveControls();
  const resultSelect = document.querySelector<HTMLSelectElement>("#add-header-result");
  if (state.result) {
    if (resultSelect) {
      resultSelect.value = state.result;
      resultSelect.disabled = true;
    }
    lastAutoResult = state.result;
  } else {
    if (resultSelect) {
      resultSelect.disabled = false;
      if (lastAutoResult !== null && resultSelect.value === lastAutoResult) {
        resultSelect.value = "*";
      }
    }
    lastAutoResult = null;
  }
}

async function syncPositionState(board: GChessBoardElement): Promise<void> {
  try {
    const state = await fetchPositionState(playedCoordMoves);
    if (state && typeof state.fen === "string" && state.allowed_moves) {
      applyState(board, state);
    }
  } catch {
    // keep current board state if backend is unavailable
  }
}

function formatDefaultSavedTitle(now: Date = new Date()): string {
  const pad = (n: number) => String(n).padStart(2, "0");
  const date = `${now.getFullYear()}-${pad(now.getMonth() + 1)}-${pad(now.getDate())}`;
  const time = `${pad(now.getHours())}:${pad(now.getMinutes())}`;
  return `Game saved ${date} ${time}`;
}

export function syncAddGameTabTitle(): void {
  const form = document.querySelector<HTMLFormElement>("#add-game-form");
  if (!form) return;
  const title = computeGameTitle(readFormHeaders(form), "Add Game");
  document.title = `${title} | Echiquier`;
}

export async function initAddGame(onSaved?: (game: SavedGame) => void): Promise<void> {
  playedCoordMoves.length = 0;
  sanMoves = [];
  allowedMoves = {};
  lastAutoResult = null;

  updateMoveControls();
  renderAddGameMoves(sanMoves);

  const form = document.querySelector<HTMLFormElement>("#add-game-form");
  form?.addEventListener("input", () => {
    syncAddGameTabTitle();
  });
  form?.addEventListener("submit", (event) => {
    event.preventDefault();
    if (playedCoordMoves.length === 0) return;
    const headers = readFormHeaders(form);
    if (computeGameTitle(headers, "") === "") {
      headers.Title = formatDefaultSavedTitle();
    }
    const newGame: SavedGame = {
      id: `local-${Date.now()}`,
      headers,
      moves: [...sanMoves],
      coordMoves: [...playedCoordMoves],
    };
    saveLocalGame(newGame);
    onSaved?.(newGame);
    showToast("✓ Saved game to local storage!");
  });

  const board = document.querySelector<GChessBoardElement>("#add-game-board");
  if (board) {
    board.fen = START_FEN;
    board.turn = "white";

    board.addEventListener("movestart", (e) => {
      e.detail.setTargets((allowedMoves[e.detail.from] ?? []) as any);
    });

    board.addEventListener("moveend", (e) => {
      const targets = allowedMoves[e.detail.from] ?? [];
      if (!targets.includes(e.detail.to)) {
        e.preventDefault();
      }
    });

    board.addEventListener("movefinished", (e) => {
      playedCoordMoves.push(`${e.detail.from}${e.detail.to}`);
      updateMoveControls();
      void syncPositionState(board);
    });

    const undoBtn = document.querySelector<HTMLButtonElement>("#add-undo-btn");
    undoBtn?.addEventListener("click", () => {
      if (playedCoordMoves.length === 0) return;
      playedCoordMoves.pop();
      updateMoveControls();
      void syncPositionState(board);
    });

    const importBtn = document.querySelector<HTMLButtonElement>("#add-import-pgn-btn");
    const importBox = document.querySelector<HTMLElement>("#add-import-pgn-box");
    const importFile = document.querySelector<HTMLInputElement>("#add-import-pgn-file");
    const importText = document.querySelector<HTMLTextAreaElement>("#add-import-pgn-text");
    const importSubmit = document.querySelector<HTMLButtonElement>("#add-import-pgn-submit");
    const importCancel = document.querySelector<HTMLButtonElement>("#add-import-pgn-cancel");

    const loadPgnText = async (raw: string) => {
      const trimmed = raw.trim();
      if (!trimmed || playedCoordMoves.length > 0) return;
      try {
        const imported = await importPgn(trimmed);
        playedCoordMoves.length = 0;
        playedCoordMoves.push(...imported.coord_moves);
        populateFormHeaders(imported.headers);
        syncAddGameTabTitle();
        applyState(board, imported.state);
        if (importBox) importBox.hidden = true;
        if (importText) importText.value = "";
        showToast("✓ Imported PGN!");
      } catch {
        showToast("✕ Could not parse PGN");
      }
    };

    importBtn?.addEventListener("click", () => {
      if (importBox) importBox.hidden = false;
      importBtn.hidden = true;
      importText?.focus();
    });

    importCancel?.addEventListener("click", () => {
      if (importBox) importBox.hidden = true;
      updateMoveControls();
    });

    importFile?.addEventListener("change", async () => {
      const file = importFile.files?.[0];
      if (!file) return;
      const content = await file.text();
      importFile.value = "";
      await loadPgnText(content);
    });

    importSubmit?.addEventListener("click", async () => {
      await loadPgnText(importText?.value ?? "");
    });

    await syncPositionState(board);
  }
}
