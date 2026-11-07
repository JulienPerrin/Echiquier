use std::clone::Clone;
use std::cmp::PartialEq;
use std::fmt::{self, Debug, Display};
use std::marker::Copy;

use crate::echiquier::case::Case;
use crate::echiquier::echiquier::Echiquier;
use crate::echiquier::piece_move::PieceMove;
use crate::pieces::king::King;
use crate::pieces::piece::{Piece, PieceData};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position<'a> {
    pub echiquier: &'a Echiquier,
    pub pieces: [Option<Piece>; 64],
    pub last_moved_piece: Option<Piece>,
    pub halfmove_clock: u16,
    pub fullmove_number: u16,
}

impl<'a> Position<'a> {
    #[allow(dead_code)]
    pub const fn new(echiquier: &'a Echiquier) -> Position<'a> {
        Position {
            echiquier,
            pieces: [None; 64],
            last_moved_piece: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    pub fn set_piece(&mut self, piece: impl Into<Piece>) {
        let piece = piece.into();
        let index = (piece.case.rank * 8 + piece.case.file) as usize;
        self.pieces[index] = Some(piece);
    }

    pub fn get_piece(&self, case: &Case) -> Option<Piece> {
        let index = (case.rank * 8 + case.file) as usize;
        self.pieces[index]
    }

    pub fn get_relative_piece(&self, case: &Case, piece_move: PieceMove) -> Option<Piece> {
        let target_case = self.echiquier.get_relative_case(case, piece_move)?;
        self.get_piece(&target_case)
    }

    pub fn remove_piece(&mut self, piece: &PieceData) {
        let index = (piece.case.rank * 8 + piece.case.file) as usize;
        self.pieces[index] = None;
    }

    pub fn is_case_occupied(&self, case: &Case) -> bool {
        self.get_piece(case).is_some()
    }

    pub fn is_case_attacked(&self, case: &Case, is_white: bool) -> bool {
        let mut test_pos = *self;
        test_pos.set_piece(King::new(*case, is_white));
        test_pos.get_pieces::<Piece>(!is_white) 
            .into_iter()
            .flat_map(|p| p.candidate_captures(&test_pos))
            .any(|(_, target)| target.case == *case)
    }

    pub fn is_case_occupied_or_attacked(&self, case: &Case, is_white: bool) -> bool {
        self.is_case_occupied(case) || self.is_case_attacked(case, is_white)
    }

    pub fn get_pieces<T: TryFrom<Piece>>(&self, is_white: bool) -> Vec<T> {
        self.pieces
            .into_iter()
            .flatten()
            .filter(|piece| piece.is_white == is_white)
            .filter_map(|piece| T::try_from(piece).ok())
            .collect()
    }

    pub fn is_white_turn(&self) -> bool {
        self.last_moved_piece.map_or(true, |p| !p.is_white)
    }

    pub fn is_turn(&self, is_white: bool) -> bool {
        self.is_white_turn() == is_white
    }

    fn can_castle(&self, rank: u8, rook_file: u8, is_white: bool) -> bool {
        matches!(self.get_piece(&self.echiquier.get_case(rank, 4)), Some(Piece::King(k)) if k.is_white == is_white && k.last_move.is_none())
            && matches!(self.get_piece(&self.echiquier.get_case(rank, rook_file)), Some(Piece::Rook(r)) if r.is_white == is_white && r.last_move.is_none())
    }

    pub fn get_fen(&self) -> String {
        let mut ranks = Vec::with_capacity(8);
        for rank in (0..8).rev() {
            let mut rank_str = String::new();
            let mut empty_count = 0u8;
            for file in 0..8 {
                match self.pieces[rank * 8 + file] {
                    Some(piece) => {
                        if empty_count > 0 {
                            rank_str.push((b'0' + empty_count) as char);
                            empty_count = 0;
                        }
                        let ch = match piece {
                            Piece::Pawn(_) => 'p',
                            Piece::Knight(_) => 'n',
                            Piece::Bishop(_) => 'b',
                            Piece::Rook(_) => 'r',
                            Piece::Queen(_) => 'q',
                            Piece::King(_) => 'k',
                        };
                        rank_str.push(if piece.is_white { ch.to_ascii_uppercase() } else { ch });
                    }
                    None => empty_count += 1,
                }
            }
            if empty_count > 0 {
                rank_str.push((b'0' + empty_count) as char);
            }
            ranks.push(rank_str);
        }
        let placement = ranks.join("/");
        let active_color = if self.is_white_turn() { "w" } else { "b" };
        let mut castling = String::new();
        for (rank, file, is_white, symbol) in [(0, 7, true, 'K'), (0, 0, true, 'Q'), (7, 7, false, 'k'), (7, 0, false, 'q')] {
            if self.can_castle(rank, file, is_white) {
                castling.push(symbol);
            }
        }
        if castling.is_empty() {
            castling.push('-');
        }
        let en_passant = match self.last_moved_piece {
            Some(Piece::Pawn(p)) if p.last_move.is_some_and(|m| m.up.abs() == 2) => {
                let m = p.last_move.unwrap();
                Case { rank: (p.case.rank as i8 - m.up / 2) as u8, file: p.case.file }.algebraic()
            }
            _ => String::from("-"),
        };
        format!("{placement} {active_color} {castling} {en_passant} {} {}", self.halfmove_clock, self.fullmove_number)
    }
}

const EMPTY_SQUARE_SYMBOL: char = '·';
const FILES: [&str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];

impl<'a> Display for Position<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            let mut squares = Vec::new();
            for file in 0..8 {
                let square = match self.pieces[rank * 8 + file] {
                    Some(piece) => piece.to_string(),
                    None => EMPTY_SQUARE_SYMBOL.to_string(),
                };
                squares.push(square);
            }
            writeln!(f, "{} {}", rank + 1, squares.join(" "))?;
        }
        write!(f, "  {}", FILES.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::bishop::Bishop;
    use crate::pieces::king::King;
    use crate::pieces::knight::Knight;
    use crate::pieces::pawn::Pawn;
    use crate::pieces::queen::Queen;
    use crate::pieces::rook::Rook;

    #[test]
    fn print_position_with_pawn() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e2 = position.echiquier.get_case(1, 4);
        let pawn = Pawn::new(e2, true);
        position.set_piece(pawn);

        assert_eq!(
            "\
8 · · · · · · · ·
7 · · · · · · · ·
6 · · · · · · · ·
5 · · · · · · · ·
4 · · · · · · · ·
3 · · · · · · · ·
2 · · · · ♟ · · ·
1 · · · · · · · ·
  A B C D E F G H",
            position.to_string()
        );
    }

    #[test]
    fn print_position_with_all_piece_types() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let a1 = position.echiquier.get_case(0, 0);
        let b1 = position.echiquier.get_case(0, 1);
        let c1 = position.echiquier.get_case(0, 2);
        let d1 = position.echiquier.get_case(0, 3);
        let e1 = position.echiquier.get_case(0, 4);
        let f2 = position.echiquier.get_case(1, 5);
        let e8 = position.echiquier.get_case(7, 4);

        position.set_piece(Rook::new(a1, true));
        position.set_piece(Knight::new(b1, true));
        position.set_piece(Bishop::new(c1, true));
        position.set_piece(Queen::new(d1, true));
        position.set_piece(King::new(e1, true));
        position.set_piece(Pawn::new(f2, true));
        position.set_piece(King::new(e8, false));

        assert_eq!(
            "\
8 · · · · ♔ · · ·
7 · · · · · · · ·
6 · · · · · · · ·
5 · · · · · · · ·
4 · · · · · · · ·
3 · · · · · · · ·
2 · · · · · ♟ · ·
1 ♜ ♞ ♝ ♛ ♚ · · ·
  A B C D E F G H",
            position.to_string()
        );
    }

    #[test]
    fn get_pieces_by_color_and_optional_kind() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let a1 = position.echiquier.get_case(0, 0);
        let b1 = position.echiquier.get_case(0, 1);
        let c1 = position.echiquier.get_case(0, 2);
        let e8 = position.echiquier.get_case(7, 4);

        let white_rook = Rook::new(a1, true);
        let white_knight = Knight::new(b1, true);
        let white_bishop = Bishop::new(c1, true);
        let black_king = King::new(e8, false);

        position.set_piece(white_rook);
        position.set_piece(white_knight);
        position.set_piece(white_bishop);
        position.set_piece(black_king);

        assert_eq!(
            "\
8 · · · · ♔ · · ·
7 · · · · · · · ·
6 · · · · · · · ·
5 · · · · · · · ·
4 · · · · · · · ·
3 · · · · · · · ·
2 · · · · · · · ·
1 ♜ ♞ ♝ · · · · ·
  A B C D E F G H",
            position.to_string()
        );

        // All white pieces (type is Piece)
        let white_pieces = position.get_pieces::<Piece>(true);
        assert_eq!(3, white_pieces.len());
        assert!(white_pieces.contains(&Piece::from(white_rook)));
        assert!(white_pieces.contains(&Piece::from(white_knight)));
        assert!(white_pieces.contains(&Piece::from(white_bishop)));

        // Specific white piece by type
        let white_rooks = position.get_pieces::<Rook>(true);
        assert_eq!(vec![white_rook], white_rooks);

        // Specific black piece by type
        let black_kings = position.get_pieces::<King>(false);
        assert_eq!(vec![black_king], black_kings);

        // All black pieces (type is Piece)
        let black_pieces = position.get_pieces::<Piece>(false);
        assert_eq!(vec![Piece::from(black_king)], black_pieces);

        // No match for piece type of given color
        let black_pawns = position.get_pieces::<Pawn>(false);
        assert!(black_pawns.is_empty());

        // No match on empty board
        let empty_position = Position::new(&echiquier);
        assert!(empty_position.get_pieces::<Piece>(true).is_empty());
    }

    #[test]
    fn position_tracks_whose_turn_it_is() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let a1 = position.echiquier.get_case(0, 0);
        let e8 = position.echiquier.get_case(7, 4);
        let white_rook = Piece::from(Rook::new(a1, true));
        let black_king = Piece::from(King::new(e8, false));
        position.set_piece(white_rook);
        position.set_piece(black_king);

        assert_eq!(
            "\
8 · · · · ♔ · · ·
7 · · · · · · · ·
6 · · · · · · · ·
5 · · · · · · · ·
4 · · · · · · · ·
3 · · · · · · · ·
2 · · · · · · · ·
1 ♜ · · · · · · ·
  A B C D E F G H",
            position.to_string()
        );

        // 1. No moves made yet: White's turn by default
        assert!(position.is_white_turn());
        assert!(position.is_turn(true));
        assert!(!position.is_turn(false));

        // 2. White moved last: Black's turn
        position.last_moved_piece = Some(white_rook);
        assert!(!position.is_white_turn());
        assert!(!position.is_turn(true));
        assert!(position.is_turn(false));

        // 3. Black moved last: White's turn
        position.last_moved_piece = Some(black_king);
        assert!(position.is_white_turn());
        assert!(position.is_turn(true));
        assert!(!position.is_turn(false));
    }

    #[test]
    fn is_case_attacked_detects_attacks_for_white_and_black() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e4 = position.echiquier.get_case(3, 4);
        let e8 = position.echiquier.get_case(7, 4);
        let a4 = position.echiquier.get_case(3, 0);

        position.set_piece(Rook::new(e8, false));
        position.set_piece(Rook::new(a4, true));

        assert_eq!(
            "\
8 · · · · ♖ · · ·
7 · · · · · · · ·
6 · · · · · · · ·
5 · · · · · · · ·
4 ♜ · · · · · · ·
3 · · · · · · · ·
2 · · · · · · · ·
1 · · · · · · · ·
  A B C D E F G H",
            position.to_string()
        );

        assert!(position.is_case_attacked(&e4, true));
        assert!(position.is_case_attacked(&e4, false));

        let h1 = position.echiquier.get_case(0, 7);
        assert!(!position.is_case_attacked(&h1, true));
        assert!(!position.is_case_attacked(&h1, false));
    }

    #[test]
    fn is_case_occupied_or_attacked_checks_both_conditions() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e4 = position.echiquier.get_case(3, 4);
        let e8 = position.echiquier.get_case(7, 4);
        let h1 = position.echiquier.get_case(0, 7);
        let b2 = position.echiquier.get_case(1, 1);

        // e8 has an enemy rook attacking e4 along the file
        position.set_piece(Rook::new(e8, false));
        // b2 has a friendly pawn (not attacked by black)
        position.set_piece(Pawn::new(b2, true));

        assert_eq!(
            "\
8 · · · · ♖ · · ·
7 · · · · · · · ·
6 · · · · · · · ·
5 · · · · · · · ·
4 · · · · · · · ·
3 · · · · · · · ·
2 · ♟ · · · · · ·
1 · · · · · · · ·
  A B C D E F G H",
            position.to_string()
        );

        // 1. Occupied and not attacked: b2
        assert!(position.is_case_occupied_or_attacked(&b2, true));

        // 2. Not occupied but attacked: e4
        assert!(position.is_case_occupied_or_attacked(&e4, true));

        // 3. Occupied and attacked: place a piece on e4
        let white_knight = Knight::new(e4, true);
        position.set_piece(white_knight);

        assert_eq!(
            "\
8 · · · · ♖ · · ·
7 · · · · · · · ·
6 · · · · · · · ·
5 · · · · · · · ·
4 · · · · ♞ · · ·
3 · · · · · · · ·
2 · ♟ · · · · · ·
1 · · · · · · · ·
  A B C D E F G H",
            position.to_string()
        );

        assert!(position.is_case_occupied_or_attacked(&e4, true));

        // 4. Neither occupied nor attacked: h1
        assert!(!position.is_case_occupied_or_attacked(&h1, true));
    }

    #[test]
    fn get_fen_follows_pgn_spec_chapter_16_1() {
        use crate::game::game::INITIAL_POSITION;
        use crate::pieces::movable::Movable;

        // 16.1.4 Example 1: Starting position
        let pos0 = INITIAL_POSITION;
        assert_eq!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            pos0.get_fen()
        );

        // 16.1.4 Example 2: After 1. e4
        let e2 = pos0.echiquier.get_case(1, 4);
        let e4 = pos0.echiquier.get_case(3, 4);
        let mut white_pawn_e2 = Pawn::try_from(pos0.get_piece(&e2).unwrap()).unwrap();
        let pos1 = white_pawn_e2.move_piece(&pos0, PieceMove { right: 0, up: 2 }, e4, None, None);
        assert_eq!(
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            pos1.get_fen()
        );

        // 16.1.4 Example 3: After 1... c5
        let c7 = pos1.echiquier.get_case(6, 2);
        let c5 = pos1.echiquier.get_case(4, 2);
        let mut black_pawn_c7 = Pawn::try_from(pos1.get_piece(&c7).unwrap()).unwrap();
        let pos2 = black_pawn_c7.move_piece(&pos1, PieceMove { right: 0, up: -2 }, c5, None, None);
        assert_eq!(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
            pos2.get_fen()
        );

        // 16.1.4 Example 4: After 2. Nf3
        let g1 = pos2.echiquier.get_case(0, 6);
        let f3 = pos2.echiquier.get_case(2, 5);
        let mut white_knight_g1 = Knight::try_from(pos2.get_piece(&g1).unwrap()).unwrap();
        let pos3 =
            white_knight_g1.move_piece(&pos2, PieceMove { right: -1, up: 2 }, f3, None, None);
        assert_eq!(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
            pos3.get_fen()
        );

        // Empty board has '-' for castling and en-passant
        let echiquier = Echiquier::new();
        let empty = Position::new(&echiquier);
        assert_eq!("8/8/8/8/8/8/8/8 w - - 0 1", empty.get_fen());

        // Endgame position: 4k3/8/8/8/8/8/4P3/4K3 w - - 5 39
        let mut endgame = Position::new(&echiquier);
        endgame.set_piece(King::new(echiquier.get_case(7, 4), false));
        endgame.set_piece(Pawn::new(echiquier.get_case(1, 4), true));
        endgame.set_piece(King::new(echiquier.get_case(0, 4), true));
        endgame.halfmove_clock = 5;
        endgame.fullmove_number = 39;
        assert_eq!(
            "4k3/8/8/8/8/8/4P3/4K3 w - - 5 39",
            endgame.get_fen()
        );
    }
}
