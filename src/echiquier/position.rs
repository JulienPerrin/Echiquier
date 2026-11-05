use std::clone::Clone;
use std::cmp::PartialEq;
use std::fmt::{self, Debug, Display};
use std::marker::Copy;

use crate::echiquier::echiquier::Echiquier;
use crate::pieces::king::King;
use crate::pieces::pawn::Pawn;
use crate::pieces::piece::Piece;
use crate::pieces::piece_kind::PieceKind;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position<'a> {
    pub echiquier: &'a Echiquier,
    pub pieces: [Option<Piece>; 64],
}

impl<'a> Position<'a> {
    pub fn new(echiquier: &'a Echiquier) -> Position<'a> {
        Position {
            echiquier,
            pieces: [None; 64],
        }
    }

    pub fn set_piece(&mut self, piece: Piece) {
        let index = (piece.case.rank * 8 + piece.case.file) as usize;
        self.pieces[index] = Some(piece);
    }

    fn display_square(&self, index: usize) -> String {
        match self.pieces[index] {
            Some(piece) => match piece.kind {
                PieceKind::PAWN => Pawn { piece }.to_string(),
                PieceKind::KING => King { piece }.to_string(),
                _ => todo!(),
            },
            None => String::from("x"),
        }
    }
}

impl<'a> Display for Position<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        for (rank, row_label) in rows.iter().enumerate().rev() {
            write!(f, "{}", row_label)?;
            for file in 0..8 {
                let index = rank * 8 + file;
                write!(f, " {}", self.display_square(index))?;
            }
            writeln!(f)?;
        }
        write!(f, "  1 2 3 4 5 6 7 8")
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
            "H x x x x x x x x
G x x x x x x x x
F x x x x x x x x
E x x x x x x x x
D x x x x x x x x
C x x x x x x x x
B x x x x P x x x
A x x x x x x x x
  1 2 3 4 5 6 7 8",
            position.to_string()
        );
    }
}