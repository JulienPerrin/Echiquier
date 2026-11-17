import type { PgnHeaders } from "./types";

export const SAVED_GAMES_STORAGE_KEY = "echiquier.savedGames";

export interface SavedGame {
  id: string;
  headers: PgnHeaders;
  moves: string[];
  coordMoves?: string[];
}

export function getSavedGames(): SavedGame[] {
  const raw = localStorage.getItem(SAVED_GAMES_STORAGE_KEY);
  if (!raw) return [];
  try {
    const parsed = JSON.parse(raw);
    return Array.isArray(parsed) ? (parsed as SavedGame[]) : [];
  } catch {
    return [];
  }
}

export function findSavedGame(id: string): SavedGame | undefined {
  return getSavedGames().find((g) => g.id === id);
}

export function saveLocalGame(game: SavedGame): void {
  const existing = getSavedGames();
  localStorage.setItem(SAVED_GAMES_STORAGE_KEY, JSON.stringify([...existing, game]));
}

export function formatSavedGamePgn(game: SavedGame): string {
  const headerLines = Object.entries(game.headers)
    .filter((entry): entry is [string, string] => typeof entry[1] === "string" && entry[1].trim() !== "")
    .map(([k, v]) => `[${k} "${v}"]`);

  const moveTokens: string[] = [];
  for (let i = 0; i < game.moves.length; i += 2) {
    const turn = Math.floor(i / 2) + 1;
    moveTokens.push(`${turn}.`, game.moves[i]);
    if (i + 1 < game.moves.length) {
      moveTokens.push(game.moves[i + 1]);
    }
  }
  if (game.headers.Result) {
    moveTokens.push(game.headers.Result);
  }

  return [headerLines.join("\n"), moveTokens.join(" ")].filter(Boolean).join("\n\n");
}
