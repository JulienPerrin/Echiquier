use std::clone::Clone;
use std::cmp::PartialEq;
use std::fmt::{self, Debug, Display};
use std::marker::Copy;

use crate::echiquier::case::Case;
use crate::echiquier::echiquier::Echiquier;
use crate::echiquier::piece_move::PieceMove;
use crate::pieces::bishop::Bishop;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::pawn::Pawn;
use crate::pieces::piece::Piece;
use crate::pieces::piece_kind::PieceKind;
use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;

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

    pub fn set_piece(&mut self, piece: Piece) {
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

    pub fn remove_piece(&mut self, piece: &Piece) {
        let index = (piece.case.rank * 8 + piece.case.file) as usize;
        self.pieces[index] = None;
    }

    pub fn is_case_occupied(&self, case: &Case) -> bool {
        self.get_piece(case).is_some()
    }
}

const EMPTY_SQUARE_SYMBOL: char = '·';
const FILES: [&str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];

fn piece_symbol(piece: Piece) -> String {
    match piece.kind {
        PieceKind::PAWN => Pawn { piece }.to_string(),
        PieceKind::KNIGHT => Knight { piece }.to_string(),
        PieceKind::BISHOP => Bishop { piece }.to_string(),
        PieceKind::ROOK => Rook { piece }.to_string(),
        PieceKind::QUEEN => Queen { piece }.to_string(),
        PieceKind::KING => King { piece }.to_string(),
    }
}

impl<'a> Display for Position<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            let mut squares = Vec::new();
            for file in 0..8 {
                let square = match self.pieces[rank * 8 + file] {
                    Some(piece) => piece_symbol(piece),
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
    use crate::pieces::pawn::Pawn;

    #[test]
    fn print_position_with_pawn() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e2 = position.echiquier.get_case(1, 4);
        let pawn = Pawn::new(e2, true);
        position.set_piece(*pawn);

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

        position.set_piece(*Rook::new(a1, true));
        position.set_piece(*Knight::new(b1, true));
        position.set_piece(*Bishop::new(c1, true));
        position.set_piece(*Queen::new(d1, true));
        position.set_piece(*King::new(e1, true));
        position.set_piece(*Pawn::new(f2, true));
        position.set_piece(*King::new(e8, false));

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
}
