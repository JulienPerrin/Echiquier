import { execSync } from "node:child_process";
import type { Plugin } from "vite";
import { defineConfig } from "vite";

function rustApiBridge(): Plugin {
  return {
    name: "rust-api-bridge",
    configureServer(server) {
      server.middlewares.use("/api/hello_world", (_req, res) => {
        const output = execSync("cargo run -q", { encoding: "utf-8" });
        const firstLine = output.split("\n")[0]?.trim() ?? "";
        res.setHeader("Content-Type", "text/plain; charset=utf-8");
        res.end(firstLine);
      });
    },
  };
}

export default defineConfig({
  plugins: [rustApiBridge()],
});
