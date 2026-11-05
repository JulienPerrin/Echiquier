---
name: echiquier-dev
description: >-
  Development workflow, Rust architecture, and coding conventions for the Echiquier chessboard project.
  Use this skill whenever implementing new pieces, chess rules, board/position features, or refactoring code in the echiquier repository.
---

# Echiquier Development Skill

Follow these guidelines whenever making changes to the `echiquier` project.

## 1. Respect `README.md` and Maintain the `## TODO` List
* Always read [README.md](../../../README.md) before starting a feature to align with the project's design ideas and roadmap.
* After completing any feature or rule, update the `## TODO` section in `README.md` (mark completed items `[x]` and add new sub-items if discovered).
* Always use `*` (stars) for markdown bullet lists in `README.md`.

## 2. Minimize Redundancy with Traits & Composition
* **Base Struct (`Piece`)**: Keep shared piece fields (`case`, `kind`, `is_white`, `last_move`) in `Piece` (`src/pieces/piece.rs`). Do not put `match self.kind` blocks in `Piece`.
* **Shared Trait (`Movable`)**: Put shared movement, filtering, and validation logic (`allowed_moves`, `is_move_allowed`, `move_piece`) as default methods on the `Movable` trait (`pub trait Movable: DerefMut<Target = Piece> + Display` in `src/pieces/movable.rs`).
* **Child Pieces (`Pawn`, `King`, etc.)**:
  * Place each piece in its own module (`src/pieces/<piece>.rs`).
  * Wrap `pub piece: Piece` and implement `Deref<Target = Piece>` and `DerefMut` to inherit `Piece` fields without duplicating them.
  * Implement `Display` directly on the child struct (enforced by `Movable: Display`).
  * Implement `Movable` on the child struct by providing only piece-specific `candidate_moves` (relying on `Movable`'s default `allowed_moves` and `move_piece`).

## 3. Performance & Game History
* `Position<'a>` borrows `&'a Echiquier` and stores `[Option<Piece>; 64]` so positions remain lightweight and `Copy`.
* Borrow `&Position` for read-only move queries (`candidate_moves`, `allowed_moves`, `is_move_allowed`).
* Return a new `Position<'a>` snapshot from `move_piece` instead of mutating in place, preserving previous positions for game history and engine search.

## 4. Formatting & Code Style
* Preserve existing formatting and intentional blank lines/line jumps in edited files.
* Write idiomatic Rust (`Option<T>`, supertrait bounds to enforce compile-time contracts).

## 5. Verification (`rtk cargo test` & Warnings)
* Always write unit tests (`#[cfg(test)] mod tests`) for every new piece or rule.
* Use `rtk` to run shell commands (e.g., `rtk cargo test`, `rtk cargo run`, `rtk git status`) to keep command output compact.
* Ensure all tests pass with zero compiler warnings.

## 6. Git Policy (Read-Only)
* **Never** run `git commit`, `git push`, `git add`, `git restore`, `git reset`, `git checkout`, or any command that modifies git state or history. The user handles all git operations manually.
