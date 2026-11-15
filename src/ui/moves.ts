import type { GChessBoardElement } from "gchessboard";
import { currentGameId, currentMoveIndex, moves, moveButtons } from "../state";
import { fetchGameFen } from "../api";

const board = document.querySelector<GChessBoardElement>("#board");
const moveAnnouncer = document.querySelector<HTMLElement>("#move-announcer");
const startPosBtn = document.querySelector<HTMLButtonElement>("#start-pos-btn");
const prevMoveBtn = document.querySelector<HTMLButtonElement>("#prev-move");
const nextMoveBtn = document.querySelector<HTMLButtonElement>("#next-move");
const shareFenTextEl = document.querySelector<HTMLTextAreaElement>("#share-fen-text");
const shareDialogEl = document.querySelector<HTMLDialogElement>("#share-dialog");
const reducedMotionQuery = window.matchMedia("(prefers-reduced-motion: reduce)");

export function describeMove(moveIndex: number): string {
  if (moveIndex === 0) {
    return "Initial position";
  }
  const moveStr = moves.value[moveIndex - 1] ?? "";
  const turn = Math.floor((moveIndex - 1) / 2) + 1;
  const side = (moveIndex - 1) % 2 === 0 ? "White" : "Black";
  return `Move ${turn} ${side}: ${moveStr}`;
}

export function createMoveButton(moveIndex: number, turn: number, side: "White" | "Black", moveText: string): HTMLButtonElement {
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

function updateActiveSelection(): void {
  const isStart = currentMoveIndex.value === 0;
  if (startPosBtn) {
    startPosBtn.classList.toggle("active", isStart);
    startPosBtn.setAttribute("aria-pressed", String(isStart));
  }

  for (const [idx, btn] of moveButtons.entries()) {
    const isActive = idx === currentMoveIndex.value;
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
    prevMoveBtn.setAttribute("aria-disabled", String(currentMoveIndex.value <= 0));
  }
  if (nextMoveBtn) {
    nextMoveBtn.setAttribute("aria-disabled", String(currentMoveIndex.value >= moves.value.length));
  }
}

export async function selectMove(moveIndex: number): Promise<void> {
  if (moveIndex < 0 || moveIndex > moves.value.length) {
    return;
  }
  currentMoveIndex.value = moveIndex;
  updateActiveSelection();
  const fen = await fetchGameFen(currentMoveIndex.value, currentGameId.value);
  if (board) {
    board.fen = fen;
  }
  if (shareFenTextEl && shareDialogEl?.open) {
    shareFenTextEl.value = fen;
  }
  if (moveAnnouncer) {
    moveAnnouncer.textContent = `${describeMove(currentMoveIndex.value)}`;
  }
}

export function renderMovesList(): void {
  const movesListEl = document.querySelector<HTMLElement>("#moves-list");
  if (!movesListEl) return;
  movesListEl.innerHTML = "";
  moveButtons.clear();

  for (let i = 0; i < moves.value.length; i += 2) {
    const turn = Math.floor(i / 2) + 1;
    const row = document.createElement("li");
    row.className = "move-row";

    const turnSpan = document.createElement("span");
    turnSpan.className = "turn-number";
    turnSpan.setAttribute("aria-hidden", "true");
    turnSpan.textContent = `${turn}.`;
    row.appendChild(turnSpan);

    const whiteBtn = createMoveButton(i + 1, turn, "White", moves.value[i]);
    row.appendChild(whiteBtn);

    if (i + 1 < moves.value.length) {
      const blackBtn = createMoveButton(i + 2, turn, "Black", moves.value[i + 1]);
      row.appendChild(blackBtn);
    }

    movesListEl.appendChild(row);
  }
}
