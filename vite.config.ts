import { execFileSync, execSync } from "node:child_process";
import fs from "node:fs";
import type { Plugin } from "vite";
import { defineConfig } from "vite";

const BIN_PATH = "./target/debug/chessboard";

function ensureBinary(): void {
  if (!fs.existsSync(BIN_PATH)) {
    execSync("cargo build -q");
  }
}

function runChessboard(args: string[]): string {
  ensureBinary();
  return execFileSync(BIN_PATH, args, { encoding: "utf-8" });
}

function formatPgnDateString(dateStr: string): string {
  const parts = dateStr.split(/[./-]/);
  if (parts.length === 0) return dateStr;
  const year = Number.parseInt(parts[0], 10);
  if (Number.isNaN(year) || year <= 0) return dateStr;
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
    if (day && !Number.isNaN(day) && day >= 1 && day <= 31) {
      const dateObj = new Date(Date.UTC(year, month - 1, day));
      return new Intl.DateTimeFormat(undefined, {
        year: "numeric",
        month: "short",
        day: "numeric",
        timeZone: "UTC",
      }).format(dateObj);
    }
    const dateObj = new Date(Date.UTC(year, month - 1, 1));
    return new Intl.DateTimeFormat(undefined, {
      year: "numeric",
      month: "long",
      timeZone: "UTC",
    }).format(dateObj);
  }
  return String(year);
}

function getGameMetadata() {
  try {
    const output = runChessboard(["headers"]).trim();
    const headers = JSON.parse(output) as Record<string, string>;
    const isDefined = (val?: string) =>
      typeof val === "string" &&
      val.trim() !== "" &&
      val.trim() !== "*" &&
      !/^[\s.?/—–-]*$/.test(val.trim());

    const formattedDate = isDefined(headers.Date) ? formatPgnDateString(headers.Date.trim()) : "";

    const title = isDefined(headers.Title)
      ? headers.Title
      : isDefined(headers.White) && isDefined(headers.Black)
        ? `${headers.White} - ${headers.Black}${isDefined(headers.Site) ? `, ${headers.Site}` : ""}${formattedDate ? `, ${formattedDate}` : ""}`
        : "The Immortal Game";

    const players =
      isDefined(headers.White) && isDefined(headers.Black)
        ? `${headers.White} (White) vs ${headers.Black} (Black)`
        : "";

    return { title, players };
  } catch {
    return {
      title: "The Immortal Game",
      players: "Adolf Anderssen (White) vs Lionel Kieseritzky (Black)",
    };
  }
}

function getGameOptionsHtml() {
  try {
    const output = runChessboard(["games"]).trim();
    const games = JSON.parse(output) as Array<{ id: string; title: string }>;
    return games
      .map(
        (g) =>
          `<option value="${g.id}"${g.id === "immortal" ? " selected" : ""}>${g.title}</option>`,
      )
      .join("\n              ");
  } catch {
    return `<option value="immortal" selected>The Immortal Game</option>`;
  }
}

function rustApiBridge(): Plugin {
  return {
    name: "rust-api-bridge",
    buildStart() {
      ensureBinary();
    },
    transformIndexHtml(html) {
      const { title, players } = getGameMetadata();
      const gameOptions = getGameOptionsHtml();
      return html
        .replace(/%TITLE%/g, title)
        .replace(/%PLAYERS%/g, players)
        .replace(/%GAME_OPTIONS%/g, gameOptions);
    },
    configureServer(server) {
      ensureBinary();

      server.middlewares.use("/api/hello_world", (_req, res) => {
        try {
          const output = runChessboard(["fen", "0"]).trim();
          res.setHeader("Content-Type", "text/plain; charset=utf-8");
          res.end(output);
        } catch (err) {
          res.statusCode = 500;
          res.end(String(err));
        }
      });

      server.middlewares.use("/api/games", (_req, res) => {
        try {
          const output = runChessboard(["games"]).trim();
          res.setHeader("Content-Type", "application/json; charset=utf-8");
          res.end(output);
        } catch (err) {
          res.statusCode = 500;
          res.end(JSON.stringify({ error: String(err) }));
        }
      });

      server.middlewares.use("/api/pgn", (req, res) => {
        try {
          const url = new URL(req.url ?? "", "http://localhost");
          const game = url.searchParams.get("game") || "immortal";
          const output = runChessboard(["pgn", game]);
          res.setHeader("Content-Type", "text/plain; charset=utf-8");
          res.end(output);
        } catch (err) {
          res.statusCode = 500;
          res.end(String(err));
        }
      });

      server.middlewares.use("/api/moves", (req, res) => {
        try {
          const url = new URL(req.url ?? "", "http://localhost");
          const game = url.searchParams.get("game") || "immortal";
          const output = runChessboard(["moves", game]).trim();
          const moves = output ? output.split("\n") : [];
          res.setHeader("Content-Type", "application/json; charset=utf-8");
          res.end(JSON.stringify(moves));
        } catch (err) {
          res.statusCode = 500;
          res.end(JSON.stringify({ error: String(err) }));
        }
      });

      server.middlewares.use("/api/immortal_moves", (req, res) => {
        try {
          const url = new URL(req.url ?? "", "http://localhost");
          const game = url.searchParams.get("game") || "immortal";
          const output = runChessboard(["moves", game]).trim();
          const moves = output ? output.split("\n") : [];
          res.setHeader("Content-Type", "application/json; charset=utf-8");
          res.end(JSON.stringify(moves));
        } catch (err) {
          res.statusCode = 500;
          res.end(JSON.stringify({ error: String(err) }));
        }
      });

      server.middlewares.use("/api/headers", (req, res) => {
        try {
          const url = new URL(req.url ?? "", "http://localhost");
          const game = url.searchParams.get("game") || "immortal";
          const output = runChessboard(["headers", game]).trim();
          res.setHeader("Content-Type", "application/json; charset=utf-8");
          res.end(output);
        } catch (err) {
          res.statusCode = 500;
          res.end(JSON.stringify({ error: String(err) }));
        }
      });

      server.middlewares.use("/api/immortal_headers", (req, res) => {
        try {
          const url = new URL(req.url ?? "", "http://localhost");
          const game = url.searchParams.get("game") || "immortal";
          const output = runChessboard(["headers", game]).trim();
          res.setHeader("Content-Type", "application/json; charset=utf-8");
          res.end(output);
        } catch (err) {
          res.statusCode = 500;
          res.end(JSON.stringify({ error: String(err) }));
        }
      });

      server.middlewares.use("/api/fen", (req, res) => {
        try {
          const url = new URL(req.url ?? "", "http://localhost");
          const moveParam = url.searchParams.get("move") ?? "0";
          const game = url.searchParams.get("game") || "immortal";
          const moveIndex = Number.parseInt(moveParam, 10);
          const safeIndex = Number.isNaN(moveIndex) || moveIndex < 0 ? "0" : String(moveIndex);
          const output = runChessboard(["fen", safeIndex, game]).trim();
          res.setHeader("Content-Type", "text/plain; charset=utf-8");
          res.end(output);
        } catch (err) {
          res.statusCode = 500;
          res.end(String(err));
        }
      });

      server.middlewares.use("/api/immortal_fen", (req, res) => {
        try {
          const url = new URL(req.url ?? "", "http://localhost");
          const moveParam = url.searchParams.get("move") ?? "0";
          const game = url.searchParams.get("game") || "immortal";
          const moveIndex = Number.parseInt(moveParam, 10);
          const safeIndex = Number.isNaN(moveIndex) || moveIndex < 0 ? "0" : String(moveIndex);
          const output = runChessboard(["fen", safeIndex, game]).trim();
          res.setHeader("Content-Type", "text/plain; charset=utf-8");
          res.end(output);
        } catch (err) {
          res.statusCode = 500;
          res.end(String(err));
        }
      });
    },
  };
}

export default defineConfig({
  plugins: [rustApiBridge()],
  // Tauri expects a fixed dev port (see `build.devUrl` in src-tauri/tauri.conf.json)
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**", "**/target/**"],
    },
  },
});
