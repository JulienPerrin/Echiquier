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
    const DIRECTIONS: [PieceMove; 8] = [
        PieceMove { right: -1, up: -1 },
        PieceMove { right: 0, up: -1 },
        PieceMove { right: 1, up: -1 },
        PieceMove { right: -1, up: 0 },
        PieceMove { right: 1, up: 0 },
        PieceMove { right: -1, up: 1 },
        PieceMove { right: 0, up: 1 },
        PieceMove { right: 1, up: 1 },
    ];

    pub const fn new(case: Case, is_white: bool) -> Self {
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
        Self::DIRECTIONS.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::echiquier::echiquier::Echiquier;
    use crate::pieces::pawn::Pawn;

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
        let e5 = position.echiquier.get_case(4, 4);
        let next_position = king.move_piece(&position, one_up, e5, None);
        assert_eq!(e5, king.case);
        assert_eq!(Some(*king), next_position.pieces[(e5.rank * 8 + e5.file) as usize]);
        assert_eq!("K", king.to_string());
    }

    #[test]
    fn king_captures_adjacent_opponent_piece() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let a1 = position.echiquier.get_case(0, 0);
        let a2 = position.echiquier.get_case(1, 0);
        let b2 = position.echiquier.get_case(1, 1);
        let mut white_king = King::new(a1, true);
        let black_pawn_a2 = Pawn::new(a2, false);
        let friendly_pawn_b2 = Pawn::new(b2, true);
        position.set_piece(*white_king);
        position.set_piece(*black_pawn_a2);
        position.set_piece(*friendly_pawn_b2);

        let capture_move = PieceMove { right: 0, up: 1 };
        assert_eq!(
            vec![(capture_move, *black_pawn_a2)],
            white_king.allowed_captures(&position)
        );
        assert!(white_king.is_capture_allowed(&position, capture_move, *black_pawn_a2));

        let next_position = white_king.move_piece(&position, capture_move, a2, Some(*black_pawn_a2));
        assert_eq!(a2, white_king.case);
        assert_eq!(Some(*white_king), next_position.get_piece(&a2));
        assert_eq!(None, next_position.get_piece(&a1));
    }
}
