use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

use crate::echiquier::case::Case;
use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::pieces::piece::Piece;
use crate::pieces::piece_kind::PieceKind;
use super::movable::Movable;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct King {
    pub piece: Piece,
}

impl King {
    pub fn new(case: Case, is_white: bool) -> Self {
        King {
            piece: Piece::new(case, PieceKind::KING, is_white),
        }
    }
}

impl Deref for King {
    type Target = Piece;

    fn deref(&self) -> &Self::Target {
        &self.piece
    }
}

impl DerefMut for King {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.piece
    }
}

impl Display for King {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "K")
    }
}

impl Movable for King {
    fn candidate_moves(&self, _position: &Position) -> Vec<PieceMove> {
        vec![
            PieceMove { right: -1, up: -1 },
            PieceMove { right: 0, up: -1 },
            PieceMove { right: 1, up: -1 },
            PieceMove { right: -1, up: 0 },
            PieceMove { right: 1, up: 0 },
            PieceMove { right: -1, up: 1 },
            PieceMove { right: 0, up: 1 },
            PieceMove { right: 1, up: 1 },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::echiquier::echiquier::Echiquier;

    #[test]
    fn king_moves_one_square_in_any_direction() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e4 = position.echiquier.get_case(3, 4);
        let mut king = King::new(e4, true);
        position.set_piece(*king);

        let allowed = king.allowed_moves(&position);
        assert_eq!(8, allowed.len());

        let one_up = PieceMove { right: 0, up: 1 };
        let e5 = position.echiquier.get_relative_case(&e4, one_up);
        let next_position = king.move_piece(&position, one_up, e5);
        assert_eq!(e5, king.case);
        assert_eq!(Some(*king), next_position.pieces[(e5.rank * 8 + e5.file) as usize]);
    }
}
