use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

use crate::echiquier::case::Case;
use crate::echiquier::direction::Direction;
use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::pieces::movable::Movable;
use crate::pieces::piece::Piece;
use crate::pieces::piece_kind::PieceKind;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rook {
    pub piece: Piece,
}

impl Rook {
    const DIRECTIONS: [(i8, i8); 4] = Direction::ORTHOGONAL;

    pub const fn new(case: Case, is_white: bool) -> Self {
        Rook {
            piece: Piece::new(case, PieceKind::ROOK, is_white),
        }
    }
}

impl Deref for Rook {
    type Target = Piece;

    fn deref(&self) -> &Self::Target {
        &self.piece
    }
}

impl DerefMut for Rook {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.piece
    }
}

impl Display for Rook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self.is_white { '♜' } else { '♖' })
    }
}

impl Movable for Rook {
    fn candidate_moves(&self, position: &Position) -> Vec<PieceMove> {
        self.sliding_candidate_moves(position, &Self::DIRECTIONS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::echiquier::echiquier::Echiquier;
    use crate::pieces::pawn::Pawn;

    #[test]
    fn rook_moves_horizontally_and_vertically_and_cannot_jump_over_pieces() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let a1 = position.echiquier.get_case(0, 0);
        let mut rook = Rook::new(a1, true);
        position.set_piece(*rook);

        // Movement is not yet made (track movement for castling)
        assert!(rook.last_move.is_none());

        // Place a friendly pawn at a3 (rank 2, file 0)
        let a3 = position.echiquier.get_case(2, 0);
        let friendly_pawn = Pawn::new(a3, true);
        position.set_piece(*friendly_pawn);

        let allowed = rook.allowed_moves(&position);
        // Can move up to a2 (0, 1) but not onto the friendly pawn at a3 (0, 2), and horizontally along rank 0: b1..h1
        assert!(allowed.contains(&PieceMove { right: 0, up: 1 }));
        assert!(!allowed.contains(&PieceMove { right: 0, up: 2 }));
        assert!(allowed.contains(&PieceMove { right: 1, up: 0 }));
        assert!(allowed.contains(&PieceMove { right: 7, up: 0 }));

        // Cannot jump over a3 to a4 (0, 3) or a8 (0, 7)
        assert!(!allowed.contains(&PieceMove { right: 0, up: 3 }));
        assert!(!allowed.contains(&PieceMove { right: 0, up: 7 }));

        let b1 = position.echiquier.get_case(0, 1);
        let right_one = PieceMove { right: 1, up: 0 };
        let next_position = rook.move_piece(&position, right_one, b1, None);
        assert_eq!(b1, rook.case);
        assert_eq!(Some(right_one), rook.last_move);
        assert!(rook.last_move.is_some());
        assert_eq!(Some(*rook), next_position.get_piece(&b1));
        assert_eq!(None, next_position.get_piece(&a1));
        assert_eq!("♜", rook.to_string());
        let black_rook = Rook::new(b1, false);
        assert_eq!("♖", black_rook.to_string());
    }

    #[test]
    fn rook_captures_opponent_piece() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let a1 = position.echiquier.get_case(0, 0);
        let a3 = position.echiquier.get_case(2, 0);
        let mut white_rook = Rook::new(a1, true);
        let black_pawn_a3 = Pawn::new(a3, false);
        position.set_piece(*white_rook);
        position.set_piece(*black_pawn_a3);

        let capture_move = PieceMove { right: 0, up: 2 };
        assert_eq!(
            vec![(capture_move, *black_pawn_a3)],
            white_rook.allowed_captures(&position)
        );
        assert!(white_rook.is_capture_allowed(&position, capture_move, *black_pawn_a3));

        // Cannot jump past a3 to a4
        assert!(
            !white_rook
                .allowed_moves(&position)
                .contains(&PieceMove { right: 0, up: 3 })
        );

        let next_position =
            white_rook.move_piece(&position, capture_move, a3, Some(*black_pawn_a3));
        assert_eq!(a3, white_rook.case);
        assert_eq!(Some(*white_rook), next_position.get_piece(&a3));
        assert_eq!(None, next_position.get_piece(&a1));
    }
}
