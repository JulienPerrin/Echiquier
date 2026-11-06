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
}

impl<'a> Position<'a> {
    #[allow(dead_code)]
    pub const fn new(echiquier: &'a Echiquier) -> Position<'a> {
        Position {
            echiquier,
            pieces: [None; 64],
            last_moved_piece: None,
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

        // 1. Occupied and not attacked: b2
        assert!(position.is_case_occupied_or_attacked(&b2, true));

        // 2. Not occupied but attacked: e4
        assert!(position.is_case_occupied_or_attacked(&e4, true));

        // 3. Occupied and attacked: place a piece on e4
        let white_knight = Knight::new(e4, true);
        position.set_piece(white_knight);
        assert!(position.is_case_occupied_or_attacked(&e4, true));

        // 4. Neither occupied nor attacked: h1
        assert!(!position.is_case_occupied_or_attacked(&h1, true));
    }
}
