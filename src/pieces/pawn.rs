use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

use crate::echiquier::case::Case;
use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::pieces::piece::Piece;
use crate::pieces::piece_kind::PieceKind;
use super::movable::Movable;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pawn {
    pub piece: Piece,
}

impl Pawn {
    pub fn new(case: Case, is_white: bool) -> Self {
        Pawn {
            piece: Piece::new(case, PieceKind::PAWN, is_white),
        }
    }
}

impl Deref for Pawn {
    type Target = Piece;

    fn deref(&self) -> &Self::Target {
        &self.piece
    }
}

impl DerefMut for Pawn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.piece
    }
}

impl Display for Pawn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P")
    }
}

impl Movable for Pawn {
    fn candidate_moves(&self, _position: &Position) -> Vec<PieceMove> {
        let forward_move = if self.is_white {
            PieceMove { right: 0, up: 1 }
        } else {
            PieceMove { right: 0, up: -1 }
        };

        vec![forward_move]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::echiquier::echiquier::Echiquier;

    #[test]
    fn white_pawn_moves_one_square_up() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e2 = position.echiquier.get_case(1, 4);
        let mut pawn = Pawn::new(e2, true);
        position.set_piece(*pawn);

        let one_up = PieceMove { right: 0, up: 1 };
        let allowed = pawn.allowed_moves(&position);
        assert_eq!(vec![one_up], allowed);
        assert!(pawn.is_move_allowed(&position, one_up));

        let e3 = position.echiquier.get_relative_case(&e2, one_up);
        let next_position = pawn.move_piece(&position, one_up, e3);
        assert_eq!(e3, pawn.case);
        assert_eq!(Some(*pawn), next_position.pieces[(e3.rank * 8 + e3.file) as usize]);
        assert_eq!(None, next_position.pieces[(e2.rank * 8 + e2.file) as usize]);
    }

    #[test]
    fn black_pawn_moves_one_square_down() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e7 = position.echiquier.get_case(6, 4);
        let mut pawn = Pawn::new(e7, false);
        position.set_piece(*pawn);

        let one_down = PieceMove { right: 0, up: -1 };
        let allowed = pawn.allowed_moves(&position);
        assert_eq!(vec![one_down], allowed);
        assert!(pawn.is_move_allowed(&position, one_down));

        let e6 = position.echiquier.get_relative_case(&e7, one_down);
        let next_position = pawn.move_piece(&position, one_down, e6);
        assert_eq!(e6, pawn.case);
        assert_eq!(Some(*pawn), next_position.pieces[(e6.rank * 8 + e6.file) as usize]);
        assert_eq!(None, next_position.pieces[(e7.rank * 8 + e7.file) as usize]);
    }

    #[test]
    fn pawn_cannot_move_off_board() {
        let echiquier = Echiquier::new();
        let position = Position::new(&echiquier);
        let e8 = position.echiquier.get_case(7, 4);
        let white_pawn = Pawn::new(e8, true);
        assert!(white_pawn.allowed_moves(&position).is_empty());

        let e1 = position.echiquier.get_case(0, 4);
        let black_pawn = Pawn::new(e1, false);
        assert!(black_pawn.allowed_moves(&position).is_empty());
    }
}