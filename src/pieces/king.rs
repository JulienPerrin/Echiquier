use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

use super::movable::Movable;
use crate::echiquier::case::Case;
use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::pieces::piece::PieceData;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct King {
    pub piece: PieceData,
}

impl King {
    pub const fn new(case: Case, is_white: bool) -> Self {
        King {
            piece: PieceData::new(case, is_white),
        }
    }

    #[allow(dead_code)]
    pub fn is_in_check(&self, position: &Position) -> bool {
        position
            .get_pieces(!self.is_white, None)
            .into_iter()
            .flat_map(|p| p.candidate_captures(position))
            .any(|(_, target)| target.case == self.case)
    }
}

impl Deref for King {
    type Target = PieceData;

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
        write!(f, "{}", if self.is_white { '♚' } else { '♔' })
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
    use crate::pieces::bishop::Bishop;
    use crate::pieces::knight::Knight;
    use crate::pieces::pawn::Pawn;
    use crate::pieces::piece::Piece;
    use crate::pieces::queen::Queen;
    use crate::pieces::rook::Rook;

    #[test]
    fn king_moves_one_square_in_any_direction() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e4 = position.echiquier.get_case(3, 4);
        let mut king = King::new(e4, true);
        position.set_piece(king);

        let allowed = king.allowed_moves(&position);
        assert_eq!(8, allowed.len());

        let one_up = PieceMove { right: 0, up: 1 };
        let e5 = position.echiquier.get_case(4, 4);
        let next_position = king.move_piece(&position, one_up, e5, None);
        assert_eq!(e5, king.case);
        assert_eq!(
            Some(Piece::from(king)),
            next_position.pieces[(e5.rank * 8 + e5.file) as usize]
        );
        assert_eq!("♚", king.to_string());
        let black_king = King::new(e5, false);
        assert_eq!("♔", black_king.to_string());
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
        position.set_piece(white_king);
        position.set_piece(black_pawn_a2);
        position.set_piece(friendly_pawn_b2);

        let capture_move = PieceMove { right: 0, up: 1 };
        assert_eq!(
            vec![(capture_move, Piece::from(black_pawn_a2))],
            white_king.allowed_captures(&position)
        );
        assert!(white_king.is_capture_allowed(&position, capture_move, black_pawn_a2.into()));

        let next_position =
            white_king.move_piece(&position, capture_move, a2, Some(black_pawn_a2.into()));
        assert_eq!(a2, white_king.case);
        assert_eq!(Some(Piece::from(white_king)), next_position.get_piece(&a2));
        assert_eq!(None, next_position.get_piece(&a1));
    }

    #[test]
    fn king_is_in_check_by_various_opponent_pieces() {
        let echiquier = Echiquier::new();

        // 1. Check from Rook
        let mut position = Position::new(&echiquier);
        let e1 = position.echiquier.get_case(0, 4);
        let e8 = position.echiquier.get_case(7, 4);
        let white_king = King::new(e1, true);
        let black_rook = Rook::new(e8, false);
        position.set_piece(white_king);
        position.set_piece(black_rook);
        assert!(white_king.is_in_check(&position));

        // 2. Check from Bishop
        let mut position = Position::new(&echiquier);
        let c3 = position.echiquier.get_case(2, 2);
        let black_bishop = Bishop::new(c3, false);
        position.set_piece(white_king);
        position.set_piece(black_bishop);
        assert!(white_king.is_in_check(&position));

        // 3. Check from Knight
        let mut position = Position::new(&echiquier);
        let d3 = position.echiquier.get_case(2, 3);
        let black_knight = Knight::new(d3, false);
        position.set_piece(white_king);
        position.set_piece(black_knight);
        assert!(white_king.is_in_check(&position));

        // 4. Check from Queen
        let mut position = Position::new(&echiquier);
        let h4 = position.echiquier.get_case(3, 7);
        let black_queen = Queen::new(h4, false);
        position.set_piece(white_king);
        position.set_piece(black_queen);
        assert!(white_king.is_in_check(&position));

        // 5. Check from Pawn (diagonally)
        let mut position = Position::new(&echiquier);
        let d2 = position.echiquier.get_case(1, 3);
        let black_pawn = Pawn::new(d2, false);
        position.set_piece(white_king);
        position.set_piece(black_pawn);
        assert!(white_king.is_in_check(&position));

        // 6. Check from adjacent enemy King
        let mut position = Position::new(&echiquier);
        let e2 = position.echiquier.get_case(1, 4);
        let black_king = King::new(e2, false);
        position.set_piece(white_king);
        position.set_piece(black_king);
        assert!(white_king.is_in_check(&position));

        // 7. Black King in check from White Pawn
        let mut position = Position::new(&echiquier);
        let e8 = position.echiquier.get_case(7, 4);
        let f7 = position.echiquier.get_case(6, 5);
        let black_king_e8 = King::new(e8, false);
        let white_pawn_f7 = Pawn::new(f7, true);
        position.set_piece(black_king_e8);
        position.set_piece(white_pawn_f7);
        assert!(black_king_e8.is_in_check(&position));
    }

    #[test]
    fn king_is_not_in_check_when_blocked_or_unreachable() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e1 = position.echiquier.get_case(0, 4);
        let e2 = position.echiquier.get_case(1, 4);
        let e8 = position.echiquier.get_case(7, 4);
        let white_king = King::new(e1, true);

        // Empty board: not in check
        position.set_piece(white_king);
        assert!(!white_king.is_in_check(&position));

        // Blocked by friendly piece: not in check
        let friendly_pawn = Pawn::new(e2, true);
        let black_rook = Rook::new(e8, false);
        position.set_piece(friendly_pawn);
        position.set_piece(black_rook);
        assert!(!white_king.is_in_check(&position));

        // Blocked by opponent piece: not in check
        let mut position2 = Position::new(&echiquier);
        let opponent_pawn = Pawn::new(e2, false);
        position2.set_piece(white_king);
        position2.set_piece(opponent_pawn);
        position2.set_piece(black_rook);
        assert!(!white_king.is_in_check(&position2));

        // Pawn directly in front does not check
        let mut position3 = Position::new(&echiquier);
        position3.set_piece(white_king);
        position3.set_piece(opponent_pawn);
        assert!(!white_king.is_in_check(&position3));

        // Friendly pieces only: not in check
        let mut position4 = Position::new(&echiquier);
        let a1 = position4.echiquier.get_case(0, 0);
        let friendly_rook = Rook::new(a1, true);
        position4.set_piece(white_king);
        position4.set_piece(friendly_rook);
        assert!(!white_king.is_in_check(&position4));
    }
}
