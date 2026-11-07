# Echiquier

A chessboard written in RUST

## Ideas

- chessboard contains squares 1 to 64
- square have a rank (row) and a file (column) each 1 to 1
- pieces are on a specific square 1 to 1
- position is a chessboard 1 to 1 and pieces 1 to 2..64
- game is a succession of positions 1 to 1..n
- usually there's the starting position at the begining
- to go from one position to another you play a move
  - there's allowed moves
  - some moves are allowed depending of the position of other peaces
    - can't move over a piece
    - can take another piece
    - in check, need to get out of check
  - some moves are allowed depending on the previous moves
    - en-passant depends on the last move
    - castle depends on the fact that the king or the rook has not already moved
  - if no moves is available, the game end, we are either :
    - in pat
    - or mate if king is in check

## TODO

- [x] Create 64-square chessboard (`Echiquier` and `Case`)
- [x] Create `Position` with `[Option<Piece>; 64]` and console `Display`
  - [x] Query pieces by color and optional kind (`get_pieces`)
- [x] Implement `Pawn` 1-square forward movement (up for white, down for black)
- [x] Track `last_move: Option<PieceMove>` on `Piece`
- [x] Improve `Position` display with Unicode chess symbols (white/black pieces)
- [x] Plain console board for `cargo run`: outline vs solid glyphs and `·` for empty squares
- [x] Complete `Pawn` rules:
  - [x] 2-square initial move when `last_move.is_none()`
  - [x] Prevent moving onto an occupied square
  - [x] Diagonal capture
  - [x] En-passant capture (only immediately on the turn after an opponent's pawn moves 2 squares)
  - [x] Promotion on the last rank (to `Queen`, `Rook`, `Bishop`, or `Knight`)
- [x] Implement remaining pieces:
  - [x] `Knight`
    - [x] move on any sqare that is not on its row, column or diagonal (2 squares in one direction, 1 square perpendicular)
    - [x] can jump over other pieces
  - [x] `Bishop`
    - [x] move diagonally any number of squares
    - [x] cannot jump over pieces
  - [x] `Rook`
    - [x] move horizontally or vertically any number of squares
    - [x] cannot jump over pieces
    - [x] track movement (`last_move.is_none()`) for castling
  - [x] `Queen`
    - [x] move horizontally, vertically, or diagonally any number of squares
    - [x] cannot jump over pieces
  - [x] `King`
    - [x] move one square any direction
    - [x] cannot move onto an attacked square
    - [x] castling (kingside and queenside, when king and rook have `last_move.is_none()`, path is clear, and not passing through check)
    - [x] `is_in_check`
    - [x] `is_in_checkmate`
    - [x] `is_in_stalemate` (`pat`)
- [x] Implement board collision & capture rules
  - [x] cannot jump over pieces except `Knight` (`sliding_candidate_moves` stops at the first occupied square)
  - [x] cannot capture own pieces
    - [x] captures of own-colour pieces are filtered out (`candidate_captures`)
    - [x] reject non-capture moves onto an occupied square (`allowed_moves` excludes occupied target squares)
- [x] Implement `Pin` rule (a piece cannot move if doing so exposes its own `King` to check)
- [x] Create starting `Position` with all 32 pieces on their initial squares
  - [x] Declare `INITIAL_POSITION` as a `const` in `game.rs` with all `Pawn`s and `King`s
- [x] Create `Game` struct holding the starting position and history of `Position`s for replay
  - [x] track whose turn it is to play (White or Black)
  - [x] determine game outcome (`Result::WhiteWin`, `Result::BlackWin`, `Result::Draw`) via `is_game_finished` and `game_result`
  - [ ] support chess notation (FEN / Algebraic notation / PGN)
    - [x] Algebraic notation for moves with disambiguation, captures, check (`+`), checkmate (`#`), draw (`=`)
    - [x] Parse algebraic notation move text into a `PositionChange` struct with disambiguation and permissive capture support (ignoring annotations `!`, `?`, `+`, `#`)
    - [ ] Handle `=` in notation (can mean either draw or draw offer / demand of draw), and support draws and resignations
- [ ] Build chess engine / GUI (`macroquad`)
