import "gchessboard";
import type { GChessBoardElement } from "gchessboard";
import { invoke } from "@tauri-apps/api/core";

const board = document.querySelector<GChessBoardElement>("#board");
const apiStatus = document.querySelector<HTMLElement>("#api-status");
const moveAnnouncer = document.querySelector<HTMLParagraphElement>("#move-announcer");
const movesListEl = document.querySelector<HTMLOListElement>("#moves-list");
const startPosBtn = document.querySelector<HTMLButtonElement>("#start-pos-btn");
const prevMoveBtn = document.querySelector<HTMLButtonElement>("#prev-move");
const nextMoveBtn = document.querySelector<HTMLButtonElement>("#next-move");

const reducedMotionQuery = window.matchMedia("(prefers-reduced-motion: reduce)");

let moves: string[] = [];
let currentMoveIndex = 0;
const moveButtons = new Map<number, HTMLButtonElement>();

function syncBoardMotionPreference(): void {
  if (board) {
    board.animationDuration = reducedMotionQuery.matches ? 0 : 200;
  }
}

async function fetchImmortalMoves(): Promise<string[]> {
  if ("__TAURI_INTERNALS__" in window) {
    return invoke<string[]>("immortal_moves");
  }
  const response = await fetch("/api/immortal_moves");
  return (await response.json()) as string[];
}

async function fetchImmortalFen(moveIndex: number): Promise<string> {
  if ("__TAURI_INTERNALS__" in window) {
    return invoke<string>("immortal_fen", { moveIndex });
  }
  const response = await fetch(`/api/immortal_fen?move=${moveIndex}`);
  return (await response.text()).trim();
}

function updateActiveSelection(): void {
  const isStart = currentMoveIndex === 0;
  if (startPosBtn) {
    startPosBtn.classList.toggle("active", isStart);
    startPosBtn.setAttribute("aria-pressed", String(isStart));
  }

  for (const [idx, btn] of moveButtons.entries()) {
    const isActive = idx === currentMoveIndex;
    btn.classList.toggle("active", isActive);
    btn.setAttribute("aria-pressed", String(isActive));
    if (isActive) {
      btn.scrollIntoView({
        block: "nearest",
        behavior: reducedMotionQuery.matches ? "instant" : "smooth",
      });
    }
  }

  if (prevMoveBtn) {
    prevMoveBtn.setAttribute("aria-disabled", String(currentMoveIndex <= 0));
  }
  if (nextMoveBtn) {
    nextMoveBtn.setAttribute("aria-disabled", String(currentMoveIndex >= moves.length));
  }
}

function describeMove(moveIndex: number): string {
  if (moveIndex === 0) {
    return "Initial position";
  }
  const moveStr = moves[moveIndex - 1] ?? "";
  const turn = Math.floor((moveIndex - 1) / 2) + 1;
  const side = (moveIndex - 1) % 2 === 0 ? "White" : "Black";
  return `Move ${turn} ${side}: ${moveStr}`;
}

async function selectMove(moveIndex: number): Promise<void> {
  if (moveIndex < 0 || moveIndex > moves.length) {
    return;
  }
  currentMoveIndex = moveIndex;
  updateActiveSelection();
  const fen = await fetchImmortalFen(currentMoveIndex);
  if (board) {
    board.fen = fen;
  }
  if (apiStatus) {
    apiStatus.textContent = fen;
  }
  if (moveAnnouncer) {
    moveAnnouncer.textContent = `${describeMove(currentMoveIndex)}`;
  }
}

function createMoveButton(moveIndex: number, turn: number, side: "White" | "Black", moveText: string): HTMLButtonElement {
  const btn = document.createElement("button");
  btn.type = "button";
  btn.className = "move-btn";
  btn.setAttribute("aria-pressed", "false");

  const srPrefix = document.createElement("span");
  srPrefix.className = "visually-hidden";
  srPrefix.textContent = `Move ${turn} ${side}: `;

  btn.append(srPrefix, document.createTextNode(moveText));
  btn.addEventListener("click", () => {
    void selectMove(moveIndex);
  });
  moveButtons.set(moveIndex, btn);
  return btn;
}

function renderMovesList(): void {
  if (!movesListEl) return;
  movesListEl.innerHTML = "";
  moveButtons.clear();

  for (let i = 0; i < moves.length; i += 2) {
    const turn = Math.floor(i / 2) + 1;
    const row = document.createElement("li");
    row.className = "move-row";

    const turnSpan = document.createElement("span");
    turnSpan.className = "turn-number";
    turnSpan.setAttribute("aria-hidden", "true");
    turnSpan.textContent = `${turn}.`;
    row.appendChild(turnSpan);

    const whiteBtn = createMoveButton(i + 1, turn, "White", moves[i]);
    row.appendChild(whiteBtn);

    if (i + 1 < moves.length) {
      const blackBtn = createMoveButton(i + 2, turn, "Black", moves[i + 1]);
      row.appendChild(blackBtn);
    }

    movesListEl.appendChild(row);
  }
}

async function initApp(): Promise<void> {
  syncBoardMotionPreference();
  reducedMotionQuery.addEventListener("change", syncBoardMotionPreference);

  moves = await fetchImmortalMoves();
  renderMovesList();

  startPosBtn?.addEventListener("click", () => {
    void selectMove(0);
  });

  prevMoveBtn?.addEventListener("click", () => {
    if (prevMoveBtn.getAttribute("aria-disabled") === "true") return;
    void selectMove(currentMoveIndex - 1);
  });

  nextMoveBtn?.addEventListener("click", () => {
    if (nextMoveBtn.getAttribute("aria-disabled") === "true") return;
    void selectMove(currentMoveIndex + 1);
  });

  window.addEventListener("keydown", (event) => {
    if (event.target instanceof HTMLElement && event.target.closest("g-chess-board")) {
      return;
    }
    if (event.key === "ArrowLeft") {
      event.preventDefault();
      void selectMove(currentMoveIndex - 1);
    } else if (event.key === "ArrowRight") {
      event.preventDefault();
      void selectMove(currentMoveIndex + 1);
    }
  });

  await selectMove(0);
}

void initApp();
