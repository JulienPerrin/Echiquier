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
const gameTitleEl = document.querySelector<HTMLHeadingElement>("#game-title");
const gamePlayersEl = document.querySelector<HTMLParagraphElement>("#game-players");
const gameHeadersEl = document.querySelector<HTMLDListElement>("#game-headers");
const gameResultContainer = document.querySelector<HTMLDivElement>("#game-result-container");

const reducedMotionQuery = window.matchMedia("(prefers-reduced-motion: reduce)");

interface PgnHeaders {
  Title?: string;
  Event?: string;
  Site?: string;
  Date?: string;
  Round?: string;
  White?: string;
  Black?: string;
  Result?: string;
  [key: string]: string | undefined;
}

let moves: string[] = [];
let gameResult: string | undefined;
let currentMoveIndex = 0;
const moveButtons = new Map<number, HTMLButtonElement>();

function syncBoardMotionPreference(): void {
  if (board) {
    board.animationDuration = reducedMotionQuery.matches ? 0 : 200;
  }
}

async function fetchImmortalHeaders(): Promise<PgnHeaders> {
  if ("__TAURI_INTERNALS__" in window) {
    return invoke<PgnHeaders>("immortal_headers");
  }
  const response = await fetch("/api/immortal_headers");
  return (await response.json()) as PgnHeaders;
}

function isHeaderDefined(val: string | undefined): boolean {
  if (typeof val !== "string") return false;
  const trimmed = val.trim();
  if (trimmed === "" || trimmed === "*") return false;
  return !/^[\s.?/—–-]*$/.test(trimmed);
}

function computeGameTitle(headers: PgnHeaders): string {
  if (isHeaderDefined(headers.Title)) {
    return headers.Title!;
  }

  const white = isHeaderDefined(headers.White) ? headers.White!.trim() : "";
  const black = isHeaderDefined(headers.Black) ? headers.Black!.trim() : "";

  let players = "";
  if (white && black) {
    players = `${white} vs ${black}`;
  } else if (white) {
    players = white;
  } else if (black) {
    players = black;
  }

  const site =
    isHeaderDefined(headers.Site) ? headers.Site!.trim()
    : isHeaderDefined(headers.Event) ? headers.Event!.trim()
    : "";
  const date = isHeaderDefined(headers.Date) ? headers.Date!.trim() : "";

  const locationAndDate = [site, date].filter(Boolean).join(", ");

  if (players && locationAndDate) {
    return `${players}, ${locationAndDate}`;
  }
  if (players) {
    return players;
  }
  if (locationAndDate) {
    return locationAndDate;
  }
  return "The Immortal Game";
}

function renderHeaders(headers: PgnHeaders): void {
  const resolvedTitle = computeGameTitle(headers);

  if (gameTitleEl) {
    gameTitleEl.textContent = resolvedTitle;
  }
  document.title = `${resolvedTitle} | Echiquier`;

  if (gamePlayersEl) {
    if (isHeaderDefined(headers.Title)) {
      const white = isHeaderDefined(headers.White) ? headers.White! : "White (?)";
      const black = isHeaderDefined(headers.Black) ? headers.Black! : "Black (?)";
      gamePlayersEl.textContent = `${white} vs ${black}`;
      gamePlayersEl.hidden = false;
    } else {
      gamePlayersEl.textContent = "";
      gamePlayersEl.hidden = true;
    }
  }

  if (!gameHeadersEl) return;
  gameHeadersEl.innerHTML = "";

  const tagKeys = ["Event", "Site", "Date", "Round", "Result"] as const;

  for (const tag of tagKeys) {
    const val = headers[tag];
    if (!isHeaderDefined(val)) {
      continue;
    }

    const item = document.createElement("div");
    item.className = "header-item";

    const dt = document.createElement("dt");
    dt.textContent = tag;
    item.appendChild(dt);

    const dd = document.createElement("dd");
    if (tag === "Result") {
      dd.className = "result-badge";
      dd.textContent = val!;
    } else {
      dd.textContent = val!;
    }

    item.appendChild(dd);
    gameHeadersEl.appendChild(item);
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

function renderGameResult(result?: string): void {
  if (!gameResultContainer) return;
  gameResultContainer.innerHTML = "";
  if (result && isHeaderDefined(result)) {
    const badge = document.createElement("span");
    badge.className = "game-result-badge";
    badge.textContent = result;
    gameResultContainer.appendChild(badge);
    gameResultContainer.setAttribute("aria-label", `Game result: ${result}`);
  }
}

async function initApp(): Promise<void> {
  syncBoardMotionPreference();
  reducedMotionQuery.addEventListener("change", syncBoardMotionPreference);

  const [fetchedHeaders, fetchedMoves] = await Promise.all([fetchImmortalHeaders(), fetchImmortalMoves()]);

  renderHeaders(fetchedHeaders);
  moves = fetchedMoves;
  gameResult = fetchedHeaders.Result;
  renderMovesList();
  renderGameResult(gameResult);

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
