import { headers } from "../state";
import { computeGameTitle, isHeaderDefined, formatPgnDate } from "../utils";

const gameTitleEl = document.querySelector<HTMLElement>("#game-title");
const gamePlayersEl = document.querySelector<HTMLElement>("#game-players");
const gameHeadersEl = document.querySelector<HTMLElement>("#game-headers");

export function renderHeaders(): void {
  const h = headers.value;
  if (!h) return;
  const resolvedTitle = computeGameTitle(h);
  document.title = `${resolvedTitle} | Echiquier`;

  if (gameTitleEl) {
    gameTitleEl.innerHTML = "";
    if (isHeaderDefined(h.Title)) {
      gameTitleEl.textContent = h.Title!;
    } else {
      const white = isHeaderDefined(h.White) ? h.White!.trim() : "";
      const black = isHeaderDefined(h.Black) ? h.Black!.trim() : "";
      const players = white && black ? `${white} vs ${black}` : white || black;
      const site =
        isHeaderDefined(h.Site) ? h.Site!.trim()
        : isHeaderDefined(h.Event) ? h.Event!.trim()
        : "";
      const parsedDate = isHeaderDefined(h.Date) ? formatPgnDate(h.Date!.trim()) : null;

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
    if (isHeaderDefined(h.Title)) {
      const white = isHeaderDefined(h.White) ? h.White! : "White (?)";
      const black = isHeaderDefined(h.Black) ? h.Black! : "Black (?)";
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
    const val = (h as any)[tag];
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
