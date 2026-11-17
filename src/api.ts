import { invoke } from "@tauri-apps/api/core";
import { findSavedGame, formatSavedGamePgn, getSavedGames } from "./storage";
import { computeGameTitle } from "./utils";

export interface GameInfo {
  id: string;
  title: string;
}

export interface PositionState {
  fen: string;
  turn: "white" | "black";
  san_moves: string[];
  allowed_moves: Record<string, string[]>;
  result?: "1-0" | "0-1" | "1/2-1/2" | null;
}

export interface ImportedPgn {
  headers: Record<string, string>;
  coord_moves: string[];
  state: PositionState;
}

export interface PgnHeaders {
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

export const isTauri = "__TAURI_INTERNALS__" in window;

export async function fetchGamesList(): Promise<GameInfo[]> {
  const remoteGames = isTauri
    ? await invoke<GameInfo[]>("list_games")
    : ((await (await fetch("/api/games")).json()) as GameInfo[]);
  const localGames: GameInfo[] = getSavedGames().map((g) => ({
    id: g.id,
    title: computeGameTitle(g.headers, "Saved Game"),
  }));
  return [...remoteGames, ...localGames];
}

export async function fetchGameHeaders(gameId: string = "immortal"): Promise<PgnHeaders> {
  const saved = findSavedGame(gameId);
  if (saved) return saved.headers;
  if (isTauri) {
    return invoke<PgnHeaders>("game_headers", { game: gameId });
  }
  const response = await fetch(`/api/headers?game=${encodeURIComponent(gameId)}`);
  return (await response.json()) as PgnHeaders;
}

export async function fetchGameMoves(gameId: string = "immortal"): Promise<string[]> {
  const saved = findSavedGame(gameId);
  if (saved) return saved.moves;
  if (isTauri) {
    return invoke<string[]>("game_moves", { game: gameId });
  }
  const response = await fetch(`/api/moves?game=${encodeURIComponent(gameId)}`);
  return (await response.json()) as string[];
}

export async function fetchGameFen(moveIndex: number, gameId: string = "immortal"): Promise<string> {
  const saved = findSavedGame(gameId);
  if (saved) {
    const state = await fetchPositionState((saved.coordMoves ?? []).slice(0, moveIndex));
    return state.fen;
  }
  if (isTauri) {
    return invoke<string>("game_fen", { moveIndex, game: gameId });
  }
  const response = await fetch(`/api/fen?move=${moveIndex}&game=${encodeURIComponent(gameId)}`);
  return (await response.text()).trim();
}

export async function fetchGamePgn(gameId: string = "immortal"): Promise<string> {
  const saved = findSavedGame(gameId);
  if (saved) return formatSavedGamePgn(saved);
  if (isTauri) {
    return invoke<string>("game_pgn", { game: gameId });
  }
  const response = await fetch(`/api/pgn?game=${encodeURIComponent(gameId)}`);
  return await response.text();
}

export async function fetchPositionState(moves: string[] = []): Promise<PositionState> {
  if (isTauri) {
    return invoke<PositionState>("position_state", { moves });
  }
  const response = await fetch(`/api/position_state?moves=${encodeURIComponent(moves.join(","))}`);
  return (await response.json()) as PositionState;
}

export async function importPgn(pgn: string): Promise<ImportedPgn> {
  if (isTauri) {
    return invoke<ImportedPgn>("import_pgn", { pgn });
  }
  const response = await fetch("/api/import_pgn", { method: "POST", body: pgn });
  if (!response.ok) {
    throw new Error("Invalid PGN");
  }
  return (await response.json()) as ImportedPgn;
}
