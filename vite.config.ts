import { execFileSync, execSync } from "node:child_process";
import type { Plugin } from "vite";
import { defineConfig } from "vite";

const BIN_PATH = "./target/debug/chessboard";

function rustApiBridge(): Plugin {
  return {
    name: "rust-api-bridge",
    buildStart() {
      execSync("cargo build -q");
    },
    configureServer(server) {
      server.middlewares.use("/api/hello_world", (_req, res) => {
        const output = execFileSync(BIN_PATH, ["fen", "0"], { encoding: "utf-8" }).trim();
        res.setHeader("Content-Type", "text/plain; charset=utf-8");
        res.end(output);
      });

      server.middlewares.use("/api/immortal_moves", (_req, res) => {
        const output = execFileSync(BIN_PATH, ["moves"], { encoding: "utf-8" }).trim();
        const moves = output ? output.split("\n") : [];
        res.setHeader("Content-Type", "application/json; charset=utf-8");
        res.end(JSON.stringify(moves));
      });

      server.middlewares.use("/api/immortal_fen", (req, res) => {
        const url = new URL(req.url ?? "", "http://localhost");
        const moveParam = url.searchParams.get("move") ?? "0";
        const moveIndex = Number.parseInt(moveParam, 10);
        const safeIndex = Number.isNaN(moveIndex) || moveIndex < 0 ? "0" : String(moveIndex);
        const output = execFileSync(BIN_PATH, ["fen", safeIndex], { encoding: "utf-8" }).trim();
        res.setHeader("Content-Type", "text/plain; charset=utf-8");
        res.end(output);
      });
    },
  };
}

export default defineConfig({
  plugins: [rustApiBridge()],
});
