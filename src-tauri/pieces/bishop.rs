use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

use crate::echiquier::case::Case;
use crate::echiquier::direction::Direction;
use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::pieces::movable::Movable;
use crate::pieces::piece::PieceData;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bishop {
    pub piece: PieceData,
}

impl Bishop {
    const DIRECTIONS: [(i8, i8); 4] = Direction::DIAGONAL;

    pub const fn new(case: Case, is_white: bool) -> Self {
        Bishop {
            piece: PieceData::new(case, is_white),
        }
    }
}

impl Deref for Bishop {
    type Target = PieceData;

    fn deref(&self) -> &Self::Target {
        &self.piece
    }
}

impl DerefMut for Bishop {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.piece
    }
}

impl Display for Bishop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self.is_white { '♝' } else { '♗' })
    }
}

impl Movable for Bishop {
    fn candidate_moves(&self, position: &Position) -> Vec<PieceMove> {
        self.sliding_candidate_moves(position, &Self::DIRECTIONS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::echiquier::echiquier::Echiquier;
    use crate::pieces::pawn::Pawn;
    use crate::pieces::piece::Piece;

    #[test]
    fn bishop_moves_diagonally_and_cannot_jump_over_pieces() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let c1 = position.echiquier.get_case(0, 2);
        let mut bishop = Bishop::new(c1, true);
        position.set_piece(bishop);

        // Place a friendly pawn at e3 (blocking further squares along that diagonal)
        let e3 = position.echiquier.get_case(2, 4);
        let friendly_pawn = Pawn::new(e3, true);
        position.set_piece(friendly_pawn);

        let allowed = bishop.allowed_moves(&position);
        // From c1:
        // diagonal up-left: b2 (-1, 1), a3 (-2, 2)
        // diagonal up-right: d2 (1, 1), but not e3 (2, 2) [occupied by a friendly pawn]
        assert!(allowed.contains(&PieceMove { right: -1, up: 1 }));
        assert!(allowed.contains(&PieceMove { right: -2, up: 2 }));
        assert!(allowed.contains(&PieceMove { right: 1, up: 1 }));
        assert!(!allowed.contains(&PieceMove { right: 2, up: 2 }));
        // Cannot jump over e3 to f4 (3, 3) or g5 (4, 4)
        assert!(!allowed.contains(&PieceMove { right: 3, up: 3 }));
        assert!(!allowed.contains(&PieceMove { right: 4, up: 4 }));

        let d2 = position.echiquier.get_case(1, 3);
        let next_position =
            bishop.move_piece(&position, PieceMove { right: 1, up: 1 }, d2, None, None);
        assert_eq!(d2, bishop.case);
        assert_eq!(Some(Piece::from(bishop)), next_position.get_piece(&d2));
        assert_eq!(None, next_position.get_piece(&c1));
        assert_eq!("♝", bishop.to_string());
        let black_bishop = Bishop::new(c1, false);
        assert_eq!("♗", black_bishop.to_string());
    }

    #[test]
    fn bishop_captures_opponent_piece() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let c1 = position.echiquier.get_case(0, 2);
        let e3 = position.echiquier.get_case(2, 4);
        let mut white_bishop = Bishop::new(c1, true);
        let black_pawn_e3 = Pawn::new(e3, false);
        position.set_piece(white_bishop);
        position.set_piece(black_pawn_e3);

        let capture_move = PieceMove { right: 2, up: 2 };
        assert_eq!(
            vec![(capture_move, Piece::from(black_pawn_e3))],
            white_bishop.allowed_captures(&position)
        );
        assert!(white_bishop.is_capture_allowed(&position, capture_move, black_pawn_e3.into()));

        // Cannot jump past e3 to f4
        assert!(
            !white_bishop
                .allowed_moves(&position)
                .contains(&PieceMove { right: 3, up: 3 })
        );

        let next_position = white_bishop.move_piece(
            &position,
            capture_move,
            e3,
            Some(black_pawn_e3.into()),
            None,
        );
        assert_eq!(e3, white_bishop.case);
        assert_eq!(
            Some(Piece::from(white_bishop)),
            next_position.get_piece(&e3)
        );
        assert_eq!(None, next_position.get_piece(&c1));
    }
}
