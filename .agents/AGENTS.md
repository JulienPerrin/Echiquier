# Agent Guide (`echiquier`)

## 0. Project & Environment Skills

- **`echiquier-dev`** ([SKILL.md](skills/echiquier-dev/SKILL.md)): Read before working on chess engine rules, board/position features, game history, or frontend/backend chess APIs.
- **Global skills:** Use `env-rust` + `tauri-v2` for Rust/Tauri changes, `modern-web-guidance` before HTML/CSS/client-JS UI work, and `coverage-100` for bringing files to 100% test coverage.

## 1. Watching & Running TypeScript Tests (`src/`)

- **Watch mode (low-token / failure-only):** Run `npx vitest --watch --reporter=dot --silent` as a background daemon (`IsDaemon: true`, e.g. via a dedicated watcher subagent). Only report failing test details to keep context/credits minimal.
- **One-shot & coverage:** Run `npm test` (`vitest run`) or `npm run coverage` (`vitest run --coverage`, configured in `vitest.config.ts` with `coverage.include: ['src/**/*.ts']`).
- **Naming convention:** Name each test file after its TypeScript counterpart (`<module>.spec.ts` next to `<module>.ts`, e.g. `src/ui/add-game.ts` → `src/ui/add-game.spec.ts`).
- **Build check:** Run `npx vite build` to verify the production bundle.

## 2. Working on `src/` (Frontend)

- **Stack:** Vanilla TypeScript, Vite, `@preact/signals-core` (`src/state.ts`), and `gchessboard`.
- **Dual-mode API (`src/api.ts`):** All backend calls go through `src/api.ts`, which switches between Tauri `invoke(...)` (when `"__TAURI_INTERNALS__" in window`) and `/api/*` endpoints served by the Vite dev bridge (`vite.config.ts`).
- **Chess rules:** Never import an external JS chess library. Legal moves, FEN, turn state, and SAN move formatting are computed by `src-tauri` (e.g. `fetchPositionState` → `position_state`).

## 3. Working on `src-tauri/` (Rust Backend)

- **Architecture:** Chess engine modules live in `src-tauri/echiquier/`, `src-tauri/pieces/`, and `src-tauri/game/`; Tauri v2 IPC commands live in `src-tauri/commands.rs` and are registered in `src-tauri/lib.rs` (`tauri::generate_handler![]`).
- **Keep browser & desktop in sync:** When adding a Tauri command in `commands.rs` / `lib.rs`, also add:
  1. A matching CLI subcommand in `src-tauri/main.rs` (`handle_cli_command`), and
  2. A `/api/*` middleware route in `vite.config.ts` (`rustApiBridge`).
- **Verification:** Always run `rtk cargo test` and `rtk cargo check` with zero warnings, and write `#[cfg(test)] mod tests` for all new Rust functions.
