import { gameResult } from "../state";
import { isHeaderDefined } from "../utils";

const gameResultContainer = document.querySelector<HTMLElement>("#game-result-container");

export function renderGameResult(): void {
  if (!gameResultContainer) return;
  gameResultContainer.innerHTML = "";
  const result = gameResult.value;
  if (result && isHeaderDefined(result)) {
    const badge = document.createElement("span");
    badge.className = "game-result-badge";
    badge.textContent = result;
    gameResultContainer.appendChild(badge);
    gameResultContainer.setAttribute("aria-label", `Game result: ${result}`);
  }
}
