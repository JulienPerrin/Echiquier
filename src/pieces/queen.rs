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
pub struct Queen {
    pub piece: Piece,
}

impl Queen {
    const DIRECTIONS: [(i8, i8); 8] = Direction::ALL;

    pub const fn new(case: Case, is_white: bool) -> Self {
        Queen {
            piece: Piece::new(case, PieceKind::QUEEN, is_white),
        }
    }
}

impl Deref for Queen {
    type Target = Piece;

    fn deref(&self) -> &Self::Target {
        &self.piece
    }
}

impl DerefMut for Queen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.piece
    }
}

impl Display for Queen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self.is_white { '♛' } else { '♕' })
    }
}

impl Movable for Queen {
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
    fn queen_moves_horizontally_vertically_diagonally_and_cannot_jump_over_pieces() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let d1 = position.echiquier.get_case(0, 3);
        let mut queen = Queen::new(d1, true);
        position.set_piece(*queen);

        // Place a friendly pawn at d3 (rank 2, file 3) blocking vertical moves past d3
        let d3 = position.echiquier.get_case(2, 3);
        position.set_piece(*Pawn::new(d3, true));

        // Place a friendly pawn at b3 (rank 2, file 1) blocking diagonal moves past b3
        let b3 = position.echiquier.get_case(2, 1);
        position.set_piece(*Pawn::new(b3, true));

        let allowed = queen.allowed_moves(&position);

        // Vertical moves: d2 (0, 1), d3 (0, 2) allowed, but cannot jump over d3 to d4 (0, 3)
        assert!(allowed.contains(&PieceMove { right: 0, up: 1 }));
        assert!(allowed.contains(&PieceMove { right: 0, up: 2 }));
        assert!(!allowed.contains(&PieceMove { right: 0, up: 3 }));

        // Diagonal up-left moves: c2 (-1, 1), b3 (-2, 2) allowed, but cannot jump over b3 to a4 (-3, 3)
        assert!(allowed.contains(&PieceMove { right: -1, up: 1 }));
        assert!(allowed.contains(&PieceMove { right: -2, up: 2 }));
        assert!(!allowed.contains(&PieceMove { right: -3, up: 3 }));

        // Diagonal up-right moves: e2 (1, 1), f3 (2, 2), g4 (3, 3), h5 (4, 4) unblocked
        assert!(allowed.contains(&PieceMove { right: 1, up: 1 }));
        assert!(allowed.contains(&PieceMove { right: 4, up: 4 }));

        // Horizontal moves: unblocked along rank 0
        assert!(allowed.contains(&PieceMove { right: -3, up: 0 })); // a1
        assert!(allowed.contains(&PieceMove { right: 4, up: 0 })); // h1

        let d2 = position.echiquier.get_case(1, 3);
        let next_position = queen.move_piece(&position, PieceMove { right: 0, up: 1 }, d2, None);
        assert_eq!(d2, queen.case);
        assert_eq!(Some(*queen), next_position.get_piece(&d2));
        assert_eq!(None, next_position.get_piece(&d1));
        assert_eq!("♛", queen.to_string());
        let black_queen = Queen::new(d1, false);
        assert_eq!("♕", black_queen.to_string());
    }

    #[test]
    fn queen_captures_opponent_piece() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let d1 = position.echiquier.get_case(0, 3);
        let d4 = position.echiquier.get_case(3, 3);
        let mut white_queen = Queen::new(d1, true);
        let black_pawn_d4 = Pawn::new(d4, false);
        position.set_piece(*white_queen);
        position.set_piece(*black_pawn_d4);

        let capture_move = PieceMove { right: 0, up: 3 };
        assert_eq!(
            vec![(capture_move, *black_pawn_d4)],
            white_queen.allowed_captures(&position)
        );
        assert!(white_queen.is_capture_allowed(&position, capture_move, *black_pawn_d4));

        // Cannot jump past d4 to d5
        assert!(!white_queen.allowed_moves(&position).contains(&PieceMove { right: 0, up: 4 }));

        let next_position = white_queen.move_piece(&position, capture_move, d4, Some(*black_pawn_d4));
        assert_eq!(d4, white_queen.case);
        assert_eq!(Some(*white_queen), next_position.get_piece(&d4));
        assert_eq!(None, next_position.get_piece(&d1));
    }
}
