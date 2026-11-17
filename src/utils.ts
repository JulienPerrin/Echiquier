export function isHeaderDefined(val: string | undefined): boolean {
  if (typeof val !== "string") return false;
  const trimmed = val.trim();
  if (trimmed === "" || trimmed === "*") return false;
  return !/^[\s.?/—–-]*$/.test(trimmed);
}

export function formatPgnDate(dateStr: string): { datetime: string; formatted: string } | null {
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

export function computeGameTitle(
  headers: { [key: string]: string | undefined },
  fallbackTitle: string = "The Immortal Game"
): string {
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

  if (players && locationAndDate) return `${players}, ${locationAndDate}`;
  if (players) return players;
  if (locationAndDate) return locationAndDate;
  return fallbackTitle;
}
