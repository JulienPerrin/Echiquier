import type { GChessBoardElement } from "gchessboard";

export function syncBoardMotionPreference(): void {
  const board = document.querySelector<GChessBoardElement>("#board");
  const reducedMotionQuery = window.matchMedia("(prefers-reduced-motion: reduce)");
  if (board) {
    board.animationDuration = reducedMotionQuery.matches ? 0 : 200;
  }
}
