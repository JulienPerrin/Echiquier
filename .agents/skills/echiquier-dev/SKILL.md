---
name: echiquier-dev
description: >-
  Project-specific architecture and chess domain conventions for the Echiquier chessboard repository.
  Use this skill alongside env-rust whenever implementing new pieces, chess rules, board/position features, or game history in echiquier.
---

# Echiquier Architecture Skill

Use this skill alongside the global `env-rust` skill when working on the `echiquier` repository.

## 1. Piece Hierarchy (`Piece`, `Movable`, and Child Pieces)
* **Base Struct (`Piece`)**: Keep shared piece fields (`case`, `kind`, `is_white`, `last_move`) in `Piece` (`src/pieces/piece.rs`). Do not put `match self.kind` blocks or `impl Display for Piece` in `src/pieces/piece.rs`.
* **Shared Trait (`Movable`)**: Keep shared movement, board-boundary filtering, and validation logic (`allowed_moves`, `is_move_allowed`, `move_piece`) as default methods on `Movable` (`pub trait Movable: DerefMut<Target = Piece> + Display` in `src/pieces/movable.rs`).
* **Child Pieces (`Pawn`, `King`, etc.)**:
  * Place each piece in its own module (`src/pieces/<piece>.rs`).
  * Wrap `pub piece: Piece` and implement `Deref<Target = Piece>` and `DerefMut`.
  * Implement `Display` directly on the child struct.
  * Implement `Movable` on the child struct by providing only `candidate_moves`.

## 2. Board, `Position`, and `Game` History
* `Position<'a>` (`src/echiquier/position.rs`) borrows `&'a Echiquier` and stores `[Option<Piece>; 64]` so positions remain lightweight and `Copy`.
* `Position` display prints ranks `H` down to `A` from top to bottom (White at the bottom) and files `1..8` along the bottom, with `x` for empty squares.
* `move_piece` borrows `&Position<'a>` and returns a new `Position<'a>` snapshot so previous positions remain intact.
* `Game<'a>` (`src/echiquier/game.rs`) stores `initial_position: Position<'a>` and `positions: Vec<Position<'a>>` to record the evolution of the game for replay and engine search.
