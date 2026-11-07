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
        position.is_case_attacked(&self.case, self.is_white)
    }

    fn has_moves(&self, position: &Position) -> bool {
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

    fn allowed_castling_moves(&self, position: &Position) -> Vec<PieceMove> {
        if !self.is_turn(position) || !self.has_never_moved() || self.is_in_check(position) {
            return Vec::new();
        }

        let rank = if self.is_white { 0 } else { 7 };
        if self.case != position.echiquier.get_case(rank, 4) {
            return Vec::new();
        }

        let mut moves = Vec::new();

        // Kingside castling (O-O): King moves right 2 to (rank, 6)
        let rook_h = position.echiquier.get_case(rank, 7);
        if let Some(Piece::Rook(rook)) = position.get_piece(&rook_h) {
            if rook.is_white == self.is_white && rook.has_never_moved() {
                let f_case = position.echiquier.get_case(rank, 5);
                let g_case = position.echiquier.get_case(rank, 6);
                if !position.is_case_occupied_or_attacked(&f_case, self.is_white)
                    && !position.is_case_occupied_or_attacked(&g_case, self.is_white)
                {
                    moves.push(PieceMove { right: 2, up: 0 });
                }
            }
        }

        // Queenside castling (O-O-O): King moves left 2 to (rank, 2)
        let rook_a = position.echiquier.get_case(rank, 0);
        if let Some(Piece::Rook(rook)) = position.get_piece(&rook_a) {
            if rook.is_white == self.is_white && rook.has_never_moved() {
                let b_case = position.echiquier.get_case(rank, 1);
                let c_case = position.echiquier.get_case(rank, 2);
                let d_case = position.echiquier.get_case(rank, 3);
                if !position.is_case_occupied(&b_case)
                    && !position.is_case_occupied_or_attacked(&c_case, self.is_white)
                    && !position.is_case_occupied_or_attacked(&d_case, self.is_white)
                {
                    moves.push(PieceMove { right: -2, up: 0 });
                }
            }
        }

        moves
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

    fn castling_moves(&self, position: &Position) -> Vec<PieceMove> {
        self.allowed_castling_moves(position)
    }

    fn after_move<'a>(&self, position: &mut Position<'a>, piece_move: PieceMove) {
        let rank = self.case.rank;
        if piece_move == (PieceMove { right: 2, up: 0 }) {
            let rook_h = position.echiquier.get_case(rank, 7);
            let rook_f = position.echiquier.get_case(rank, 5);
            if let Some(Piece::Rook(mut rook)) = position.get_piece(&rook_h) {
                position.remove_piece(&rook);
                rook.case = rook_f;
                rook.last_move = Some(PieceMove { right: -2, up: 0 });
                position.set_piece(rook);
            }
        } else if piece_move == (PieceMove { right: -2, up: 0 }) {
            let rook_a = position.echiquier.get_case(rank, 0);
            let rook_d = position.echiquier.get_case(rank, 3);
            if let Some(Piece::Rook(mut rook)) = position.get_piece(&rook_a) {
                position.remove_piece(&rook);
                rook.case = rook_d;
                rook.last_move = Some(PieceMove { right: 3, up: 0 });
                position.set_piece(rook);
            }
        }
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

        let next_position = white_king.move_piece(
            &position,
            capture_move,
            a2,
            Some(black_pawn_a2.into()),
            None,
        );
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
    fn has_moves_returns_true_when_only_captures_are_available() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let a8 = position.echiquier.get_case(7, 0);
        let b8 = position.echiquier.get_case(7, 1);
        let h7 = position.echiquier.get_case(6, 7);

        let black_king = King::new(a8, false);
        let white_rook_b8 = Rook::new(b8, true);
        // White Queen on h7 controls the 7th rank (a7 and b7), so king has no quiet moves
        let white_queen_h7 = Queen::new(h7, true);

        position.set_piece(black_king);
        position.set_piece(white_rook_b8);
        position.set_piece(white_queen_h7);
        position.last_moved_piece = Some(Piece::from(white_queen_h7));

        // Black king has no quiet moves, but allowed_moves includes the allowed capture on b8!
        assert_eq!(
            vec![PieceMove { right: 1, up: 0 }],
            black_king.allowed_moves(&position)
        );
        assert!(!black_king.allowed_captures(&position).is_empty());
        // Therefore has_moves must be true!
        assert!(black_king.has_moves(&position));
        assert!(!black_king.is_in_checkmate(&position));
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

    #[test]
    fn king_has_never_moved_helper() {
        let echiquier = Echiquier::new();
        let e1 = echiquier.get_case(0, 4);
        let mut king = King::new(e1, true);
        assert!(king.has_never_moved());
        king.last_move = Some(PieceMove { right: 0, up: 1 });
        assert!(!king.has_never_moved());
    }

    #[test]
    fn white_and_black_king_can_castle_both_sides() {
        let echiquier = Echiquier::new();

        // 1. White King on e1, Rooks on a1 and h1
        let mut white_pos = Position::new(&echiquier);
        let e1 = echiquier.get_case(0, 4);
        let a1 = echiquier.get_case(0, 0);
        let h1 = echiquier.get_case(0, 7);
        let white_king = King::new(e1, true);
        let rook_a1 = Rook::new(a1, true);
        let rook_h1 = Rook::new(h1, true);
        white_pos.set_piece(white_king);
        white_pos.set_piece(rook_a1);
        white_pos.set_piece(rook_h1);

        let castling = white_king.allowed_castling_moves(&white_pos);
        assert!(castling.contains(&PieceMove { right: 2, up: 0 }));
        assert!(castling.contains(&PieceMove { right: -2, up: 0 }));
        let allowed = white_king.allowed_moves(&white_pos);
        assert!(allowed.contains(&PieceMove { right: 2, up: 0 }));
        assert!(allowed.contains(&PieceMove { right: -2, up: 0 }));

        // 2. Black King on e8, Rooks on a8 and h8
        let mut black_pos = Position::new(&echiquier);
        let e8 = echiquier.get_case(7, 4);
        let a8 = echiquier.get_case(7, 0);
        let h8 = echiquier.get_case(7, 7);
        let black_king = King::new(e8, false);
        let rook_a8 = Rook::new(a8, false);
        let rook_h8 = Rook::new(h8, false);
        black_pos.set_piece(black_king);
        black_pos.set_piece(rook_a8);
        black_pos.set_piece(rook_h8);
        black_pos.last_moved_piece = Some(Piece::from(white_king));

        let black_castling = black_king.allowed_castling_moves(&black_pos);
        assert!(black_castling.contains(&PieceMove { right: 2, up: 0 }));
        assert!(black_castling.contains(&PieceMove { right: -2, up: 0 }));
    }

    #[test]
    fn king_cannot_castle_when_disqualified() {
        let echiquier = Echiquier::new();
        let e1 = echiquier.get_case(0, 4);
        let a1 = echiquier.get_case(0, 0);
        let h1 = echiquier.get_case(0, 7);

        // 1. King already moved
        let mut pos1 = Position::new(&echiquier);
        let mut moved_king = King::new(e1, true);
        moved_king.last_move = Some(PieceMove { right: 1, up: 0 });
        pos1.set_piece(moved_king);
        pos1.set_piece(Rook::new(a1, true));
        pos1.set_piece(Rook::new(h1, true));
        assert!(moved_king.allowed_castling_moves(&pos1).is_empty());

        // 2. Not king's turn
        let mut pos2 = Position::new(&echiquier);
        let white_king = King::new(e1, true);
        pos2.set_piece(white_king);
        pos2.set_piece(Rook::new(a1, true));
        pos2.set_piece(Rook::new(h1, true));
        pos2.last_moved_piece = Some(Piece::from(white_king));
        assert!(white_king.allowed_castling_moves(&pos2).is_empty());

        // 3. King not on initial square (e.g. at e2)
        let mut pos3 = Position::new(&echiquier);
        let e2 = echiquier.get_case(1, 4);
        let king_e2 = King::new(e2, true);
        pos3.set_piece(king_e2);
        pos3.set_piece(Rook::new(a1, true));
        pos3.set_piece(Rook::new(h1, true));
        assert!(king_e2.allowed_castling_moves(&pos3).is_empty());

        // 4. King in check
        let mut pos4 = Position::new(&echiquier);
        let e8 = echiquier.get_case(7, 4);
        let enemy_rook = Rook::new(e8, false);
        pos4.set_piece(white_king);
        pos4.set_piece(Rook::new(a1, true));
        pos4.set_piece(Rook::new(h1, true));
        pos4.set_piece(enemy_rook);
        assert!(white_king.is_in_check(&pos4));
        assert!(white_king.allowed_castling_moves(&pos4).is_empty());

        // 5. Intermediate squares occupied
        let mut pos5 = Position::new(&echiquier);
        let f1 = echiquier.get_case(0, 5);
        let b1 = echiquier.get_case(0, 1);
        pos5.set_piece(white_king);
        pos5.set_piece(Rook::new(a1, true));
        pos5.set_piece(Rook::new(h1, true));
        pos5.set_piece(Bishop::new(f1, true));
        pos5.set_piece(Knight::new(b1, true));
        assert!(white_king.allowed_castling_moves(&pos5).is_empty());

        // 6. Rooks have moved, missing, or wrong color
        let mut pos6 = Position::new(&echiquier);
        let mut moved_rook_a = Rook::new(a1, true);
        moved_rook_a.last_move = Some(PieceMove { right: 0, up: 1 });
        let black_rook_h = Rook::new(h1, false);
        pos6.set_piece(white_king);
        pos6.set_piece(moved_rook_a);
        pos6.set_piece(black_rook_h);
        assert!(white_king.allowed_castling_moves(&pos6).is_empty());
    }

    #[test]
    fn king_cannot_castle_through_or_into_attacked_squares() {
        let echiquier = Echiquier::new();
        let e1 = echiquier.get_case(0, 4);
        let a1 = echiquier.get_case(0, 0);
        let h1 = echiquier.get_case(0, 7);
        let white_king = King::new(e1, true);

        // 1. Passing square f1 is attacked by enemy rook on f8
        let mut pos1 = Position::new(&echiquier);
        let f8 = echiquier.get_case(7, 5);
        pos1.set_piece(white_king);
        pos1.set_piece(Rook::new(a1, true));
        pos1.set_piece(Rook::new(h1, true));
        pos1.set_piece(Rook::new(f8, false));
        let castling1 = white_king.allowed_castling_moves(&pos1);
        assert!(!castling1.contains(&PieceMove { right: 2, up: 0 }));
        assert!(castling1.contains(&PieceMove { right: -2, up: 0 }));

        // 2. Destination square g1 is attacked by enemy bishop on a7
        let mut pos2 = Position::new(&echiquier);
        let a7 = echiquier.get_case(6, 0);
        pos2.set_piece(white_king);
        pos2.set_piece(Rook::new(a1, true));
        pos2.set_piece(Rook::new(h1, true));
        pos2.set_piece(Bishop::new(a7, false));
        let castling2 = white_king.allowed_castling_moves(&pos2);
        assert!(!castling2.contains(&PieceMove { right: 2, up: 0 }));

        // 3. Passing square d1 is attacked by enemy rook on d8
        let mut pos3 = Position::new(&echiquier);
        let d8 = echiquier.get_case(7, 3);
        pos3.set_piece(white_king);
        pos3.set_piece(Rook::new(a1, true));
        pos3.set_piece(Rook::new(h1, true));
        pos3.set_piece(Rook::new(d8, false));
        let castling3 = white_king.allowed_castling_moves(&pos3);
        assert!(!castling3.contains(&PieceMove { right: -2, up: 0 }));
        assert!(castling3.contains(&PieceMove { right: 2, up: 0 }));

        // 4. Destination square c1 is attacked by enemy knight on b3
        let mut pos4 = Position::new(&echiquier);
        let b3 = echiquier.get_case(2, 1);
        pos4.set_piece(white_king);
        pos4.set_piece(Rook::new(a1, true));
        pos4.set_piece(Rook::new(h1, true));
        pos4.set_piece(Knight::new(b3, false));
        let castling4 = white_king.allowed_castling_moves(&pos4);
        assert!(!castling4.contains(&PieceMove { right: -2, up: 0 }));

        // 5. Square attacked by opponent pawn diagonally
        let mut pos5 = Position::new(&echiquier);
        let e2 = echiquier.get_case(1, 4);
        pos5.set_piece(white_king);
        pos5.set_piece(Rook::new(a1, true));
        pos5.set_piece(Rook::new(h1, true));
        pos5.set_piece(Pawn::new(e2, false));
        let castling5 = white_king.allowed_castling_moves(&pos5);
        assert!(!castling5.contains(&PieceMove { right: 2, up: 0 }));
        assert!(!castling5.contains(&PieceMove { right: -2, up: 0 }));
    }

    #[test]
    fn king_castling_moves_the_rook_as_well() {
        let echiquier = Echiquier::new();

        // 1. White Kingside castling
        let mut pos1 = Position::new(&echiquier);
        let e1 = echiquier.get_case(0, 4);
        let g1 = echiquier.get_case(0, 6);
        let h1 = echiquier.get_case(0, 7);
        let f1 = echiquier.get_case(0, 5);
        let mut white_king = King::new(e1, true);
        let rook_h1 = Rook::new(h1, true);
        pos1.set_piece(white_king);
        pos1.set_piece(rook_h1);

        let next_pos1 = white_king.move_piece(&pos1, PieceMove { right: 2, up: 0 }, g1, None, None);
        assert_eq!(None, next_pos1.get_piece(&e1));
        assert_eq!(None, next_pos1.get_piece(&h1));
        assert_eq!(Some(Piece::from(white_king)), next_pos1.get_piece(&g1));
        let moved_rook_f1 = next_pos1.get_piece(&f1).unwrap();
        assert_eq!(f1, moved_rook_f1.case);
        assert_eq!(
            Some(PieceMove { right: -2, up: 0 }),
            moved_rook_f1.last_move
        );

        // 2. White Queenside castling
        let mut pos2 = Position::new(&echiquier);
        let c1 = echiquier.get_case(0, 2);
        let a1 = echiquier.get_case(0, 0);
        let d1 = echiquier.get_case(0, 3);
        let mut white_king2 = King::new(e1, true);
        let rook_a1 = Rook::new(a1, true);
        pos2.set_piece(white_king2);
        pos2.set_piece(rook_a1);

        let next_pos2 =
            white_king2.move_piece(&pos2, PieceMove { right: -2, up: 0 }, c1, None, None);
        assert_eq!(None, next_pos2.get_piece(&e1));
        assert_eq!(None, next_pos2.get_piece(&a1));
        assert_eq!(Some(Piece::from(white_king2)), next_pos2.get_piece(&c1));
        let moved_rook_d1 = next_pos2.get_piece(&d1).unwrap();
        assert_eq!(d1, moved_rook_d1.case);
        assert_eq!(Some(PieceMove { right: 3, up: 0 }), moved_rook_d1.last_move);

        // 3. Black Kingside castling
        let mut pos3 = Position::new(&echiquier);
        let e8 = echiquier.get_case(7, 4);
        let g8 = echiquier.get_case(7, 6);
        let h8 = echiquier.get_case(7, 7);
        let f8 = echiquier.get_case(7, 5);
        let mut black_king = King::new(e8, false);
        let rook_h8 = Rook::new(h8, false);
        pos3.set_piece(black_king);
        pos3.set_piece(rook_h8);
        pos3.last_moved_piece = Some(Piece::from(white_king));

        let next_pos3 = black_king.move_piece(&pos3, PieceMove { right: 2, up: 0 }, g8, None, None);
        assert_eq!(None, next_pos3.get_piece(&e8));
        assert_eq!(None, next_pos3.get_piece(&h8));
        assert_eq!(Some(Piece::from(black_king)), next_pos3.get_piece(&g8));
        let moved_rook_f8 = next_pos3.get_piece(&f8).unwrap();
        assert_eq!(f8, moved_rook_f8.case);
        assert_eq!(
            Some(PieceMove { right: -2, up: 0 }),
            moved_rook_f8.last_move
        );

        // 4. Black Queenside castling
        let mut pos4 = Position::new(&echiquier);
        let c8 = echiquier.get_case(7, 2);
        let a8 = echiquier.get_case(7, 0);
        let d8 = echiquier.get_case(7, 3);
        let mut black_king2 = King::new(e8, false);
        let rook_a8 = Rook::new(a8, false);
        pos4.set_piece(black_king2);
        pos4.set_piece(rook_a8);
        pos4.last_moved_piece = Some(Piece::from(white_king));

        let next_pos4 =
            black_king2.move_piece(&pos4, PieceMove { right: -2, up: 0 }, c8, None, None);
        assert_eq!(None, next_pos4.get_piece(&e8));
        assert_eq!(None, next_pos4.get_piece(&a8));
        assert_eq!(Some(Piece::from(black_king2)), next_pos4.get_piece(&c8));
        let moved_rook_d8 = next_pos4.get_piece(&d8).unwrap();
        assert_eq!(d8, moved_rook_d8.case);
        assert_eq!(Some(PieceMove { right: 3, up: 0 }), moved_rook_d8.last_move);
    }
}
