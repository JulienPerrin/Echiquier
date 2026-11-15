import { invoke } from "@tauri-apps/api/core";

export interface GameInfo {
  id: string;
  title: string;
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
  if (isTauri) {
    return invoke<GameInfo[]>("list_games");
  }
  const response = await fetch("/api/games");
  return (await response.json()) as GameInfo[];
}

export async function fetchGameHeaders(gameId: string = "immortal"): Promise<PgnHeaders> {
  if (isTauri) {
    return invoke<PgnHeaders>("game_headers", { game: gameId });
  }
  const response = await fetch(`/api/headers?game=${encodeURIComponent(gameId)}`);
  return (await response.json()) as PgnHeaders;
}

export async function fetchGameMoves(gameId: string = "immortal"): Promise<string[]> {
  if (isTauri) {
    return invoke<string[]>("game_moves", { game: gameId });
  }
  const response = await fetch(`/api/moves?game=${encodeURIComponent(gameId)}`);
  return (await response.json()) as string[];
}

export async function fetchGameFen(moveIndex: number, gameId: string = "immortal"): Promise<string> {
  if (isTauri) {
    return invoke<string>("game_fen", { moveIndex, game: gameId });
  }
  const response = await fetch(`/api/fen?move=${moveIndex}&game=${encodeURIComponent(gameId)}`);
  return (await response.text()).trim();
}

export async function fetchGamePgn(gameId: string = "immortal"): Promise<string> {
  if (isTauri) {
    return invoke<string>("game_pgn", { game: gameId });
  }
  const response = await fetch(`/api/pgn?game=${encodeURIComponent(gameId)}`);
  return await response.text();
}
