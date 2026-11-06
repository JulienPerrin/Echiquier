use std::clone::Clone;
use std::cmp::PartialEq;
use std::fmt::{self, Debug, Display};
use std::marker::Copy;
use std::mem::discriminant;

use crate::echiquier::case::Case;
use crate::echiquier::echiquier::Echiquier;
use crate::echiquier::piece_move::PieceMove;
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

    #[allow(dead_code)]
    pub fn get_pieces(&self, is_white: bool, kind: Option<Piece>) -> Vec<Piece> {
        self.pieces
            .into_iter()
            .flatten()
            .filter(|piece| {
                piece.is_white == is_white
                    && kind.map_or(true, |k| discriminant(piece) == discriminant(&k))
            })
            .collect()
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

        let white_rook = Piece::from(Rook::new(a1, true));
        let white_knight = Piece::from(Knight::new(b1, true));
        let white_bishop = Piece::from(Bishop::new(c1, true));
        let black_king = Piece::from(King::new(e8, false));

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

        // All white pieces (kind is None)
        let white_pieces = position.get_pieces(true, None);
        assert_eq!(3, white_pieces.len());
        assert!(white_pieces.contains(&white_rook));
        assert!(white_pieces.contains(&white_knight));
        assert!(white_pieces.contains(&white_bishop));

        // Specific white piece by kind
        let white_rooks = position.get_pieces(true, Some(Piece::from(Rook::new(a1, true))));
        assert_eq!(vec![white_rook], white_rooks);

        // Specific black piece by kind
        let black_kings = position.get_pieces(false, Some(Piece::from(King::new(e8, false))));
        assert_eq!(vec![black_king], black_kings);

        // All black pieces (kind is None)
        let black_pieces = position.get_pieces(false, None);
        assert_eq!(vec![black_king], black_pieces);

        // No match for piece type of given color
        let black_pawns = position.get_pieces(false, Some(Piece::from(Pawn::new(a1, false))));
        assert!(black_pawns.is_empty());

        // No match on empty board
        let empty_position = Position::new(&echiquier);
        assert!(empty_position.get_pieces(true, None).is_empty());
    }
}
