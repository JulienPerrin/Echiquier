---
name: echiquier-dev
description: >-
  Project-specific architecture and chess domain conventions for the Echiquier chessboard repository.
  Use this skill alongside env-rust whenever implementing new pieces, chess rules, board/position features, or game history in echiquier.
---

# Echiquier Architecture Skill

Use this skill alongside the global `env-rust` skill when working on the `echiquier` repository.

## 1. Piece Hierarchy (`PieceData`, `Piece`, `Movable`, and Child Pieces)
* **Base Struct (`PieceData`)**: Keep shared piece fields (`case`, `is_white`, `last_move`) in `PieceData` (`src/pieces/piece.rs`).
* **Enum (`Piece`)**: `Piece` (`src/pieces/piece.rs`) is an enum (`Pawn`, `Knight`, `Bishop`, `Rook`, `Queen`, `King`) that implements `Deref<Target = dyn Movable>` and `Display`. Through `Deref` coercion to `dyn Movable` (and transitively to `PieceData`), any `Piece` directly exposes both `Movable` methods (`candidate_moves`, `candidate_captures`, `allowed_moves`, `allowed_captures`, etc.) and `PieceData` fields (`case`, `is_white`, `last_move`) without repeating `match` blocks across the codebase. Use `std::mem::discriminant` when comparing piece variants (e.g., against `Piece::PAWN`, `Piece::KING`, etc.).
* **Shared Trait (`Movable`)**: Keep shared movement, board-boundary filtering, and validation logic (`allowed_moves`, `is_move_allowed`, `move_piece`) as default methods on `Movable` (`pub trait Movable: DerefMut<Target = PieceData> + Display` in `src/pieces/movable.rs`).
* **Child Pieces (`Pawn`, `King`, etc.)**:
  * Place each piece in its own module (`src/pieces/<piece>.rs`).
  * Wrap `pub piece: PieceData` and implement `Deref<Target = PieceData>` and `DerefMut`.
  * Implement `Display` directly on the child struct.
  * Implement `Movable` on the child struct by providing only `candidate_moves` (and `candidate_captures` when distinct, such as for `Pawn`).

## 2. Board, `Position`, and `Game` History
* `Position<'a>` (`src/echiquier/position.rs`) borrows `&'a Echiquier` and stores `[Option<Piece>; 64]` so positions remain lightweight and `Copy`.
* `Position` display prints ranks `8` down to `1` from top to bottom (White at the bottom) and files `A..H` along the bottom. `Display` renders a compact, colour-free grid using each piece's own `Display` (solid = White, outline = Black) and `·` for empty squares. No terminal colours.
* `move_piece` borrows `&Position<'a>` and returns a new `Position<'a>` snapshot so previous positions remain intact.
* `Game<'a>` (`src-tauri/game/game.rs`) stores `initial_position: Position<'a>` and `positions: Vec<Position<'a>>` to record the evolution of the game for replay and engine search.
* For interactive play from the UI, use `Game::new_standard()`, `Game::play_coordinate_move(&mut self, coord: &str)`, `Game::allowed_moves_map(&self)`, `Game::current_fen(&self)`, and `Game::is_white_turn(&self)` rather than duplicating chess rules in TypeScript.

## 3. Tauri IPC, CLI & Frontend Bridge
* Tauri commands live in `src-tauri/commands.rs` and are registered in `src-tauri/lib.rs` (`tauri::generate_handler![]`), with matching CLI subcommands in `src-tauri/main.rs` (`handle_cli_command`) and `/api/*` routes in `vite.config.ts`.
* `position_state(moves: Vec<String>)` (`PositionState { fen, turn, san_moves, allowed_moves }`) powers interactive `gchessboard` events (`movestart` → `setTargets`, `moveend` → `preventDefault`, `movefinished` → sync state) via `src/api.ts` (`fetchPositionState`).
