use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

use super::movable::Movable;
use crate::echiquier::case::Case;
use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::pieces::piece::{Piece, PieceData};

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

    pub fn is_in_check(&self, position: &Position) -> bool {
        if !self.is_turn(position) {
            panic!("a king can not be in check it is not his turn");
        }
        position
            .get_pieces::<Piece>(!self.is_white)
            .into_iter()
            .flat_map(|p| p.candidate_captures(position))
            .any(|(_, target)| target.case == self.case)
    }

    pub fn has_moves(&self, position: &Position) -> bool {
        position
            .get_pieces::<Piece>(self.is_white)
            .into_iter()
            .any(|p| !p.allowed_moves(position).is_empty())
    }

    pub fn is_in_checkmate(&self, position: &Position) -> bool {
        if !self.is_turn(position) {
            panic!("a king can not be in checkmate it is not his turn");
        }
        self.is_in_check(position) && !self.has_moves(position)
    }

    pub fn is_in_stalemate(&self, position: &Position) -> bool {
        self.is_turn(position) && !self.is_in_check(position) && !self.has_moves(position)
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
        let next_position = king.move_piece(&position, one_up, e5, None, None);
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
            white_king.move_piece(&position, capture_move, a2, Some(black_pawn_a2.into()), None);
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
        position.last_moved_piece = Some(Piece::from(white_pawn_f7));
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

    #[test]
    fn king_is_in_checkmate_in_back_rank_mate() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let g1 = position.echiquier.get_case(0, 6);
        let f2 = position.echiquier.get_case(1, 5);
        let g2 = position.echiquier.get_case(1, 6);
        let h2 = position.echiquier.get_case(1, 7);
        let e1 = position.echiquier.get_case(0, 4);

        let white_king = King::new(g1, true);
        let black_rook = Rook::new(e1, false);
        position.set_piece(white_king);
        position.set_piece(Pawn::new(f2, true));
        position.set_piece(Pawn::new(g2, true));
        position.set_piece(Pawn::new(h2, true));
        position.set_piece(black_rook);

        assert!(white_king.is_in_check(&position));
        assert!(!white_king.has_moves(&position));
        assert!(white_king.is_in_checkmate(&position));
        assert!(!white_king.is_in_stalemate(&position));
    }

    #[test]
    fn king_is_in_stalemate_in_corner() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let a8 = position.echiquier.get_case(7, 0);
        let a6 = position.echiquier.get_case(5, 0);
        let c7 = position.echiquier.get_case(6, 2);

        let black_king = King::new(a8, false);
        let white_king = King::new(a6, true);
        let white_queen = Queen::new(c7, true);
        position.set_piece(black_king);
        position.set_piece(white_king);
        position.set_piece(white_queen);

        // 1. White just moved -> Black's turn: stalemate
        position.last_moved_piece = Some(Piece::from(white_queen));
        assert!(black_king.is_turn(&position));
        assert!(!black_king.is_in_check(&position));
        assert!(!black_king.has_moves(&position));
        assert!(black_king.is_in_stalemate(&position));
        assert!(!black_king.is_in_checkmate(&position));

        // 2. Black just moved -> White's turn: not stalemate for Black
        position.last_moved_piece = Some(Piece::from(black_king));
        assert!(!black_king.is_turn(&position));
        assert!(!black_king.is_in_stalemate(&position));
    }

    #[test]
    fn white_king_is_in_stalemate_when_white_turn() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let a1 = position.echiquier.get_case(0, 0);
        let a3 = position.echiquier.get_case(2, 0);
        let c2 = position.echiquier.get_case(1, 2);

        let white_king = King::new(a1, true);
        let black_king = King::new(a3, false);
        let black_queen = Queen::new(c2, false);
        position.set_piece(white_king);
        position.set_piece(black_king);
        position.set_piece(black_queen);

        // 1. Black moved last -> White's turn: stalemate
        position.last_moved_piece = Some(Piece::from(black_queen));
        assert!(white_king.is_turn(&position));
        assert!(!white_king.is_in_check(&position));
        assert!(!white_king.has_moves(&position));
        assert!(white_king.is_in_stalemate(&position));

        // 2. White moved last -> Black's turn: not stalemate for White
        position.last_moved_piece = Some(Piece::from(white_king));
        assert!(!white_king.is_turn(&position));
        assert!(!white_king.is_in_stalemate(&position));

        // 3. Initial position with no moves made yet -> White's turn by default
        position.last_moved_piece = None;
        assert!(white_king.is_turn(&position));
        assert!(white_king.is_in_stalemate(&position));
    }

    #[test]
    fn king_not_in_checkmate_or_stalemate_when_moves_available() {
        let echiquier = Echiquier::new();

        // 1. In check, but king can escape
        let mut position = Position::new(&echiquier);
        let e1 = position.echiquier.get_case(0, 4);
        let e8 = position.echiquier.get_case(7, 4);
        let white_king = King::new(e1, true);
        let black_rook = Rook::new(e8, false);
        position.set_piece(white_king);
        position.set_piece(black_rook);
        assert!(white_king.is_in_check(&position));
        assert!(white_king.has_moves(&position));
        assert!(!white_king.is_in_checkmate(&position));
        assert!(!white_king.is_in_stalemate(&position));

        // 2. In check, but friendly piece can block
        let mut position2 = Position::new(&echiquier);
        let b4 = position2.echiquier.get_case(3, 1);
        let bishop = Bishop::new(b4, true);
        position2.set_piece(white_king);
        position2.set_piece(black_rook);
        position2.set_piece(bishop);
        assert!(white_king.is_in_check(&position2));
        assert!(white_king.has_moves(&position2));
        assert!(!white_king.is_in_checkmate(&position2));
        assert!(!white_king.is_in_stalemate(&position2));

        // 3. Not in check and has moves
        let mut position3 = Position::new(&echiquier);
        position3.set_piece(white_king);
        assert!(!white_king.is_in_check(&position3));
        assert!(white_king.has_moves(&position3));
        assert!(!white_king.is_in_checkmate(&position3));
        assert!(!white_king.is_in_stalemate(&position3));

        // 4. King has no moves, but friendly pawn can move
        let mut position4 = Position::new(&echiquier);
        let a8 = position4.echiquier.get_case(7, 0);
        let a6 = position4.echiquier.get_case(5, 0);
        let c7 = position4.echiquier.get_case(6, 2);
        let h7 = position4.echiquier.get_case(6, 7);
        let black_king = King::new(a8, false);
        let black_pawn = Pawn::new(h7, false);
        let white_queen = Queen::new(c7, true);
        position4.set_piece(black_king);
        position4.set_piece(King::new(a6, true));
        position4.set_piece(white_queen);
        position4.set_piece(black_pawn);
        position4.last_moved_piece = Some(Piece::from(white_queen));
        assert!(!black_king.is_in_check(&position4));
        assert!(black_king.has_moves(&position4));
        assert!(!black_king.is_in_stalemate(&position4));
        assert!(!black_king.is_in_checkmate(&position4));
    }

    #[test]
    #[should_panic(expected = "a king can not be in check it is not his turn")]
    fn is_in_check_panics_if_not_his_turn() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e1 = position.echiquier.get_case(0, 4);
        let white_king = King::new(e1, true);
        position.set_piece(white_king);
        // White moved last, so it is Black's turn
        position.last_moved_piece = Some(Piece::from(white_king));
        white_king.is_in_check(&position);
    }

    #[test]
    #[should_panic(expected = "a king can not be in checkmate it is not his turn")]
    fn is_in_checkmate_panics_if_not_his_turn() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e1 = position.echiquier.get_case(0, 4);
        let white_king = King::new(e1, true);
        position.set_piece(white_king);
        // White moved last, so it is Black's turn
        position.last_moved_piece = Some(Piece::from(white_king));
        white_king.is_in_checkmate(&position);
    }
}
