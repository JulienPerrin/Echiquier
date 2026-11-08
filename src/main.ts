import "gchessboard";
import type { GChessBoardElement } from "gchessboard";
import { invoke } from "@tauri-apps/api/core";

const board = document.querySelector<GChessBoardElement>("#board");
const moveAnnouncer = document.querySelector<HTMLParagraphElement>("#move-announcer");
const movesListEl = document.querySelector<HTMLOListElement>("#moves-list");
const startPosBtn = document.querySelector<HTMLButtonElement>("#start-pos-btn");
const prevMoveBtn = document.querySelector<HTMLButtonElement>("#prev-move");
const nextMoveBtn = document.querySelector<HTMLButtonElement>("#next-move");
const gameTitleEl = document.querySelector<HTMLHeadingElement>("#game-title");
const gamePlayersEl = document.querySelector<HTMLParagraphElement>("#game-players");
const gameHeadersEl = document.querySelector<HTMLDListElement>("#game-headers");
const gameResultContainer = document.querySelector<HTMLDivElement>("#game-result-container");
const gameSelectEl = document.querySelector<HTMLSelectElement>("#game-select");
const shareBtn = document.querySelector<HTMLButtonElement>("#share-btn");
const shareDialogEl = document.querySelector<HTMLDialogElement>("#share-dialog");
const closeShareDialogBtn = document.querySelector<HTMLButtonElement>("#close-share-dialog-btn");
const tabFenBtn = document.querySelector<HTMLButtonElement>("#tab-fen");
const tabPgnBtn = document.querySelector<HTMLButtonElement>("#tab-pgn");
const panelFenEl = document.querySelector<HTMLElement>("#panel-fen");
const panelPgnEl = document.querySelector<HTMLElement>("#panel-pgn");
const shareFenTextEl = document.querySelector<HTMLTextAreaElement>("#share-fen-text");
const sharePgnTextEl = document.querySelector<HTMLTextAreaElement>("#share-pgn-text");
const dialogCopyPgnBtn = document.querySelector<HTMLButtonElement>("#dialog-copy-pgn-btn");
const dialogCopyFenBtn = document.querySelector<HTMLButtonElement>("#dialog-copy-fen-btn");
const dialogDownloadPgnBtn = document.querySelector<HTMLButtonElement>("#dialog-download-pgn-btn");
const globalToastEl = document.querySelector<HTMLElement>("#global-toast");
const toastMessageEl = document.querySelector<HTMLElement>("#toast-message");
const toastCloseBtn = document.querySelector<HTMLButtonElement>("#toast-close-btn");

const reducedMotionQuery = window.matchMedia("(prefers-reduced-motion: reduce)");

interface GameInfo {
  id: string;
  title: string;
}

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

let currentGameId = "immortal";
let moves: string[] = [];
let gameResult: string | undefined;
let currentMoveIndex = 0;
const moveButtons = new Map<number, HTMLButtonElement>();

function syncBoardMotionPreference(): void {
  if (board) {
    board.animationDuration = reducedMotionQuery.matches ? 0 : 200;
  }
}

async function fetchGamesList(): Promise<GameInfo[]> {
  if ("__TAURI_INTERNALS__" in window) {
    return invoke<GameInfo[]>("list_games");
  }
  const response = await fetch("/api/games");
  return (await response.json()) as GameInfo[];
}

async function fetchGameHeaders(gameId: string = "immortal"): Promise<PgnHeaders> {
  if ("__TAURI_INTERNALS__" in window) {
    return invoke<PgnHeaders>("game_headers", { game: gameId });
  }
  const response = await fetch(`/api/headers?game=${encodeURIComponent(gameId)}`);
  return (await response.json()) as PgnHeaders;
}

function isHeaderDefined(val: string | undefined): boolean {
  if (typeof val !== "string") return false;
  const trimmed = val.trim();
  if (trimmed === "" || trimmed === "*") return false;
  return !/^[\s.?/—–-]*$/.test(trimmed);
}

function formatPgnDate(dateStr: string): { datetime: string; formatted: string } | null {
  const parts = dateStr.split(/[./-]/);
  if (parts.length === 0) return null;

  const year = Number.parseInt(parts[0], 10);
  if (Number.isNaN(year) || year <= 0) return null;

  const monthStr = parts[1];
  const dayStr = parts[2];

  const month =
    monthStr && monthStr !== "?" && monthStr !== "??"
      ? Number.parseInt(monthStr, 10)
      : undefined;
  const day =
    dayStr && dayStr !== "?" && dayStr !== "??"
      ? Number.parseInt(dayStr, 10)
      : undefined;

  if (month && !Number.isNaN(month) && month >= 1 && month <= 12) {
    const mm = String(month).padStart(2, "0");
    if (day && !Number.isNaN(day) && day >= 1 && day <= 31) {
      const dd = String(day).padStart(2, "0");
      const iso = `${year}-${mm}-${dd}`;
      const dateObj = new Date(Date.UTC(year, month - 1, day));
      const formatted = new Intl.DateTimeFormat(undefined, {
        year: "numeric",
        month: "short",
        day: "numeric",
        timeZone: "UTC",
      }).format(dateObj);
      return { datetime: iso, formatted };
    }
    const iso = `${year}-${mm}`;
    const dateObj = new Date(Date.UTC(year, month - 1, 1));
    const formatted = new Intl.DateTimeFormat(undefined, {
      year: "numeric",
      month: "long",
      timeZone: "UTC",
    }).format(dateObj);
    return { datetime: iso, formatted };
  }

  return { datetime: String(year), formatted: String(year) };
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
  const parsedDate = isHeaderDefined(headers.Date) ? formatPgnDate(headers.Date!.trim()) : null;
  const dateStr = parsedDate ? parsedDate.formatted : isHeaderDefined(headers.Date) ? headers.Date!.trim() : "";

  const locationAndDate = [site, dateStr].filter(Boolean).join(", ");

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
  document.title = `${resolvedTitle} | Echiquier`;

  if (gameTitleEl) {
    gameTitleEl.innerHTML = "";
    if (isHeaderDefined(headers.Title)) {
      gameTitleEl.textContent = headers.Title!;
    } else {
      const white = isHeaderDefined(headers.White) ? headers.White!.trim() : "";
      const black = isHeaderDefined(headers.Black) ? headers.Black!.trim() : "";
      const players = white && black ? `${white} vs ${black}` : white || black;
      const site =
        isHeaderDefined(headers.Site) ? headers.Site!.trim()
        : isHeaderDefined(headers.Event) ? headers.Event!.trim()
        : "";
      const parsedDate = isHeaderDefined(headers.Date) ? formatPgnDate(headers.Date!.trim()) : null;

      const prefixParts = [players, site].filter(Boolean);
      if (prefixParts.length > 0 && parsedDate) {
        gameTitleEl.append(document.createTextNode(`${prefixParts.join(", ")}, `));
        const timeEl = document.createElement("time");
        timeEl.setAttribute("datetime", parsedDate.datetime);
        timeEl.textContent = parsedDate.formatted;
        gameTitleEl.appendChild(timeEl);
      } else if (parsedDate) {
        const timeEl = document.createElement("time");
        timeEl.setAttribute("datetime", parsedDate.datetime);
        timeEl.textContent = parsedDate.formatted;
        gameTitleEl.appendChild(timeEl);
      } else {
        gameTitleEl.textContent = resolvedTitle;
      }
    }
  }

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
    } else if (tag === "Date") {
      const parsed = formatPgnDate(val!);
      if (parsed) {
        const timeEl = document.createElement("time");
        timeEl.setAttribute("datetime", parsed.datetime);
        timeEl.textContent = parsed.formatted;
        dd.appendChild(timeEl);
      } else {
        dd.textContent = val!;
      }
    } else {
      dd.textContent = val!;
    }

    item.appendChild(dd);
    gameHeadersEl.appendChild(item);
  }
}

async function fetchGameMoves(gameId: string = "immortal"): Promise<string[]> {
  if ("__TAURI_INTERNALS__" in window) {
    return invoke<string[]>("game_moves", { game: gameId });
  }
  const response = await fetch(`/api/moves?game=${encodeURIComponent(gameId)}`);
  return (await response.json()) as string[];
}

async function fetchGameFen(moveIndex: number, gameId: string = "immortal"): Promise<string> {
  if ("__TAURI_INTERNALS__" in window) {
    return invoke<string>("game_fen", { moveIndex, game: gameId });
  }
  const response = await fetch(`/api/fen?move=${moveIndex}&game=${encodeURIComponent(gameId)}`);
  return (await response.text()).trim();
}

async function fetchGamePgn(gameId: string = "immortal"): Promise<string> {
  if ("__TAURI_INTERNALS__" in window) {
    return invoke<string>("game_pgn", { game: gameId });
  }
  const response = await fetch(`/api/pgn?game=${encodeURIComponent(gameId)}`);
  return await response.text();
}

function showToast(message: string): void {
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
      // In case already open, popover state is retained
    }
  }
  globalToastEl.classList.add("visible");
}

function hideToast(): void {
  if (!globalToastEl) return;
  if (typeof globalToastEl.hidePopover === "function") {
    try {
      globalToastEl.hidePopover();
    } catch {
      // Already closed
    }
  }
  globalToastEl.classList.remove("visible");
}

async function copyToClipboard(text: string, label: string): Promise<boolean> {
  try {
    if (navigator.clipboard && typeof navigator.clipboard.writeText === "function") {
      await navigator.clipboard.writeText(text);
      showToast(`✓ ${label} copied to clipboard!`);
      return true;
    }
  } catch {
    // Clipboard failed or rejected
  }
  return false;
}

function switchShareTab(tab: "fen" | "pgn"): void {
  const isFen = tab === "fen";
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

async function openShareDialog(activeTab: "fen" | "pgn" = "pgn"): Promise<void> {
  if (!shareDialogEl) return;

  const fen = await fetchGameFen(currentMoveIndex, currentGameId);
  if (shareFenTextEl) {
    shareFenTextEl.value = fen;
  }
  const pgn = await fetchGamePgn(currentGameId);
  if (sharePgnTextEl) {
    sharePgnTextEl.value = pgn;
  }

  switchShareTab(activeTab);

  if (typeof shareDialogEl.showModal === "function" && !shareDialogEl.open) {
    shareDialogEl.showModal();
  }
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
  const fen = await fetchGameFen(currentMoveIndex, currentGameId);
  if (board) {
    board.fen = fen;
  }
  if (shareFenTextEl && shareDialogEl?.open) {
    shareFenTextEl.value = fen;
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

async function loadGame(gameId: string): Promise<void> {
  currentGameId = gameId;
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

  renderHeaders(fetchedHeaders);
  moves = fetchedMoves;
  gameResult = fetchedHeaders.Result;
  renderMovesList();
  renderGameResult(gameResult);
  await selectMove(0);
}

async function ensureGameSelectOptions(): Promise<void> {
  if (!gameSelectEl || gameSelectEl.options.length > 1) {
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
    // Keep existing fallback options
  }
}

async function initApp(): Promise<void> {
  syncBoardMotionPreference();
  reducedMotionQuery.addEventListener("change", syncBoardMotionPreference);

  await ensureGameSelectOptions();

  const urlParams = new URLSearchParams(window.location.search);
  const initialGame = urlParams.get("game") || "immortal";

  gameSelectEl?.addEventListener("change", () => {
    if (gameSelectEl) {
      const selectedGame = gameSelectEl.value;
      gameSelectEl.blur();
      void loadGame(selectedGame).then(() => {
        startPosBtn?.focus({ preventScroll: true });
      });
    }
  });

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
    if (shareFenTextEl) {
      await copyToClipboard(shareFenTextEl.value, "FEN");
    }
  });

  dialogCopyPgnBtn?.addEventListener("click", async () => {
    if (sharePgnTextEl) {
      await copyToClipboard(sharePgnTextEl.value, "PGN");
    }
  });

  dialogDownloadPgnBtn?.addEventListener("click", () => {
    if (!sharePgnTextEl) return;
    const blob = new Blob([sharePgnTextEl.value], { type: "application/x-chess-pgn" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `${currentGameId}.pgn`;
    a.click();
    URL.revokeObjectURL(url);
    showToast(`✓ Downloaded ${currentGameId}.pgn`);
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
      void selectMove(currentMoveIndex - 1);
    } else if (event.key === "ArrowRight") {
      event.preventDefault();
      void selectMove(currentMoveIndex + 1);
    }
  });

  window.addEventListener("popstate", () => {
    const game = new URLSearchParams(window.location.search).get("game") || "immortal";
    if (game !== currentGameId) {
      void loadGame(game);
    }
  });

  await loadGame(initialGame);
}

void initApp();
