import { signal } from "@preact/signals-core";

export const currentGameId = signal<string>("immortal");
export const headers = signal<Record<string, string | undefined> | null>(null);
export const moves = signal<string[]>([]);
export const gameResult = signal<string | undefined>(undefined);
export const currentMoveIndex = signal<number>(0);
export const moveButtons = new Map<number, HTMLButtonElement>();
