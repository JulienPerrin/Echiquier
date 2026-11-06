use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

use crate::echiquier::case::Case;
use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::pieces::movable::Movable;
use crate::pieces::piece::Piece;
use crate::pieces::piece_kind::PieceKind;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Knight {
    pub piece: Piece,
}

impl Knight {
    const DIRECTIONS: [PieceMove; 8] = [
        PieceMove { right: -2, up: -1 },
        PieceMove { right: -2, up: 1 },
        PieceMove { right: -1, up: -2 },
        PieceMove { right: -1, up: 2 },
        PieceMove { right: 1, up: -2 },
        PieceMove { right: 1, up: 2 },
        PieceMove { right: 2, up: -1 },
        PieceMove { right: 2, up: 1 },
    ];

    pub const fn new(case: Case, is_white: bool) -> Self {
        Knight {
            piece: Piece::new(case, PieceKind::KNIGHT, is_white),
        }
    }
}

impl Deref for Knight {
    type Target = Piece;

    fn deref(&self) -> &Self::Target {
        &self.piece
    }
}

impl DerefMut for Knight {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.piece
    }
}

impl Display for Knight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self.is_white { '♞' } else { '♘' })
    }
}

impl Movable for Knight {
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
    fn knight_moves_l_shape_and_jumps_over_pieces() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e4 = position.echiquier.get_case(3, 4);
        let mut knight = Knight::new(e4, true);
        position.set_piece(*knight);

        // Surround knight with friendly pawns on all adjacent squares to test jumping
        for dr in -1..=1 {
            for du in -1..=1 {
                if dr != 0 || du != 0 {
                    if let Some(adjacent) = position.echiquier.get_relative_case(&e4, PieceMove { right: dr, up: du }) {
                        position.set_piece(*Pawn::new(adjacent, true));
                    }
                }
            }
        }

        let allowed = knight.allowed_moves(&position);
        assert_eq!(8, allowed.len());

        // Verify none of the allowed moves are on row (up == 0), column (right == 0), or diagonal (|right| == |up|)
        for m in &allowed {
            assert_ne!(0, m.right);
            assert_ne!(0, m.up);
            assert_ne!(m.right.abs(), m.up.abs());
            assert!(
                (m.right.abs() == 1 && m.up.abs() == 2) || (m.right.abs() == 2 && m.up.abs() == 1)
            );
        }

        let move_f6 = PieceMove { right: 1, up: 2 };
        let f6 = position.echiquier.get_case(5, 5);
        let next_position = knight.move_piece(&position, move_f6, f6, None);
        assert_eq!(f6, knight.case);
        assert_eq!(Some(*knight), next_position.get_piece(&f6));
        assert_eq!(None, next_position.get_piece(&e4));
        assert_eq!("♞", knight.to_string());
        let black_knight = Knight::new(e4, false);
        assert_eq!("♘", black_knight.to_string());
    }

    #[test]
    fn knight_captures_opponent_piece() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let b1 = position.echiquier.get_case(0, 1);
        let c3 = position.echiquier.get_case(2, 2);
        let mut white_knight = Knight::new(b1, true);
        let black_pawn_c3 = Pawn::new(c3, false);
        position.set_piece(*white_knight);
        position.set_piece(*black_pawn_c3);

        let capture_move = PieceMove { right: 1, up: 2 };
        assert_eq!(
            vec![(capture_move, *black_pawn_c3)],
            white_knight.allowed_captures(&position)
        );
        assert!(white_knight.is_capture_allowed(&position, capture_move, *black_pawn_c3));

        let next_position = white_knight.move_piece(&position, capture_move, c3, Some(*black_pawn_c3));
        assert_eq!(c3, white_knight.case);
        assert_eq!(Some(*white_knight), next_position.get_piece(&c3));
        assert_eq!(None, next_position.get_piece(&b1));
    }
}
