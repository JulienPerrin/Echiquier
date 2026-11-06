use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

use super::movable::Movable;
use crate::echiquier::case::Case;
use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::pieces::bishop::Bishop;
use crate::pieces::knight::Knight;
use crate::pieces::piece::{Piece, PieceData};
use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PromotionPiece {
    Queen,
    Rook,
    Bishop,
    Knight,
}

impl Display for PromotionPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PromotionPiece::Queen => write!(f, "Queen"),
            PromotionPiece::Rook => write!(f, "Rook"),
            PromotionPiece::Bishop => write!(f, "Bishop"),
            PromotionPiece::Knight => write!(f, "Knight"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pawn {
    pub piece: PieceData,
}

impl Pawn {
    pub const fn new(case: Case, is_white: bool) -> Self {
        Pawn {
            piece: PieceData::new(case, is_white),
        }
    }

    pub fn is_promotion_rank(&self, case: &Case) -> bool {
        if self.is_white {
            case.rank == 7
        } else {
            case.rank == 0
        }
    }

    pub fn promote(&self, case: Case, piece_move: PieceMove, promotion: PromotionPiece) -> Piece {
        let mut data = PieceData::new(case, self.is_white);
        data.last_move = Some(piece_move);
        match promotion {
            PromotionPiece::Queen => Piece::Queen(Queen { piece: data }),
            PromotionPiece::Rook => Piece::Rook(Rook { piece: data }),
            PromotionPiece::Bishop => Piece::Bishop(Bishop { piece: data }),
            PromotionPiece::Knight => Piece::Knight(Knight { piece: data }),
        }
    }
}

impl Deref for Pawn {
    type Target = PieceData;

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
        write!(f, "{}", if self.is_white { '♟' } else { '♙' })
    }
}

impl Movable for Pawn {
    fn candidate_moves(&self, position: &Position) -> Vec<PieceMove> {
        let direction = if self.is_white { 1 } else { -1 };
        let one_step = PieceMove {
            right: 0,
            up: direction,
        };

        let Some(one_step_case) = position.echiquier.get_relative_case(&self.case, one_step) else {
            return Vec::new();
        };
        if position.is_case_occupied(&one_step_case) {
            return Vec::new();
        }

        let mut moves = vec![one_step];

        if self.has_never_moved() {
            let two_steps = PieceMove {
                right: 0,
                up: direction * 2,
            };
            if let Some(two_steps_case) =
                position.echiquier.get_relative_case(&self.case, two_steps)
                && !position.is_case_occupied(&two_steps_case)
            {
                moves.push(two_steps);
            }
        }

        moves
    }

    fn candidate_captures(&self, position: &Position) -> Vec<(PieceMove, Piece)> {
        let direction = if self.is_white { 1 } else { -1 };
        let mut captures = Vec::new();

        for right_value in [-1, 1] {
            let diagonal_move = PieceMove {
                right: right_value,
                up: direction,
            };
            if let Some(target_piece) = position.get_relative_piece(&self.case, diagonal_move)
                && target_piece.is_white != self.is_white
            {
                captures.push((diagonal_move, target_piece));
            } else if let Some(adjacent_piece) = position.get_relative_piece(
                &self.case,
                PieceMove {
                    right: right_value,
                    up: 0,
                },
            ) && matches!(adjacent_piece, Piece::Pawn(_))
                && adjacent_piece.is_white != self.is_white
                && position.last_moved_piece == Some(adjacent_piece)
                && adjacent_piece.last_move
                    == Some(PieceMove {
                        right: 0,
                        up: -direction * 2,
                    })
            {
                captures.push((diagonal_move, adjacent_piece));
            }
        }

        captures
    }

    fn promoted_piece(
        &self,
        target_case: Case,
        piece_move: PieceMove,
        promotion: PromotionPiece,
    ) -> Option<Piece> {
        if self.is_promotion_rank(&target_case) {
            Some(self.promote(target_case, piece_move, promotion))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::echiquier::echiquier::Echiquier;

    #[test]
    fn white_pawn_moves_one_or_two_squares_initially_then_one() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e2 = position.echiquier.get_case(1, 4);
        let mut pawn = Pawn::new(e2, true);
        position.set_piece(pawn);

        let one_up = PieceMove { right: 0, up: 1 };
        let two_up = PieceMove { right: 0, up: 2 };
        let allowed = pawn.allowed_moves(&position);
        assert_eq!(vec![one_up, two_up], allowed);
        assert!(pawn.is_move_allowed(&position, one_up));
        assert!(pawn.is_move_allowed(&position, two_up));

        let e3 = position.echiquier.get_case(2, 4);
        let next_position = pawn.move_piece(&position, one_up, e3, None, None);
        assert_eq!(e3, pawn.case);
        assert_eq!(
            Some(Piece::from(pawn)),
            next_position.pieces[(e3.rank * 8 + e3.file) as usize]
        );
        assert_eq!(None, next_position.pieces[(e2.rank * 8 + e2.file) as usize]);

        assert_eq!(vec![one_up], pawn.allowed_moves(&next_position));
    }

    #[test]
    fn black_pawn_moves_one_or_two_squares_initially_then_one() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e7 = position.echiquier.get_case(6, 4);
        let mut pawn = Pawn::new(e7, false);
        position.set_piece(pawn);
        let a2 = position.echiquier.get_case(1, 0);
        position.last_moved_piece = Some(Piece::from(Pawn::new(a2, true)));

        let one_down = PieceMove { right: 0, up: -1 };
        let two_down = PieceMove { right: 0, up: -2 };
        let allowed = pawn.allowed_moves(&position);
        assert_eq!(vec![one_down, two_down], allowed);
        assert!(pawn.is_move_allowed(&position, one_down));
        assert!(pawn.is_move_allowed(&position, two_down));

        let e5 = position.echiquier.get_case(4, 4);
        let next_position = pawn.move_piece(&position, two_down, e5, None, None);
        assert_eq!(e5, pawn.case);
        assert_eq!(
            Some(Piece::from(pawn)),
            next_position.pieces[(e5.rank * 8 + e5.file) as usize]
        );
        assert_eq!(None, next_position.pieces[(e7.rank * 8 + e7.file) as usize]);

        assert_eq!(vec![one_down], pawn.allowed_moves(&next_position));
    }

    #[test]
    fn pawn_cannot_move_onto_or_jump_over_occupied_square() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e2 = position.echiquier.get_case(1, 4);
        let e3 = position.echiquier.get_case(2, 4);
        let e4 = position.echiquier.get_case(3, 4);
        let pawn = Pawn::new(e2, true);
        position.set_piece(pawn);

        let mut blocked_two_squares = position;
        blocked_two_squares.set_piece(Pawn::new(e4, false));
        assert_eq!(
            vec![PieceMove { right: 0, up: 1 }],
            pawn.allowed_moves(&blocked_two_squares)
        );

        let mut blocked_immediately = position;
        blocked_immediately.set_piece(Pawn::new(e3, false));
        assert!(pawn.allowed_moves(&blocked_immediately).is_empty());
    }

    #[test]
    fn pawn_captures_diagonally_and_removes_taken_piece() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e4 = position.echiquier.get_case(3, 4);
        let d5 = position.echiquier.get_case(4, 3);
        let f5 = position.echiquier.get_case(4, 5);
        let mut white_pawn = Pawn::new(e4, true);
        let black_pawn_d5 = Pawn::new(d5, false);
        let friendly_pawn_f5 = Pawn::new(f5, true);
        position.set_piece(white_pawn);
        position.set_piece(black_pawn_d5);
        position.set_piece(friendly_pawn_f5);

        let capture_move = PieceMove { right: -1, up: 1 };
        assert_eq!(
            vec![(capture_move, Piece::from(black_pawn_d5))],
            white_pawn.allowed_captures(&position)
        );
        assert!(white_pawn.is_capture_allowed(&position, capture_move, black_pawn_d5.into()));

        let next_position =
            white_pawn.move_piece(&position, capture_move, d5, Some(black_pawn_d5.into()), None);
        assert_eq!(d5, white_pawn.case);
        assert_eq!(Some(Piece::from(white_pawn)), next_position.get_piece(&d5));
        assert_eq!(None, next_position.get_piece(&e4));

        let mut black_pawn_for_capture = black_pawn_d5;
        let black_capture_move = PieceMove { right: 1, up: -1 };
        let taken_white_pawn = Piece::from(Pawn::new(e4, true));
        assert_eq!(
            vec![(black_capture_move, taken_white_pawn)],
            black_pawn_for_capture.allowed_captures(&position)
        );
        let mut black_turn_position = position;
        let white_pawn_piece = Piece::from(Pawn::new(e4, true));
        black_turn_position.last_moved_piece = Some(white_pawn_piece);
        let after_black_capture = black_pawn_for_capture.move_piece(
            &black_turn_position,
            black_capture_move,
            e4,
            Some(taken_white_pawn),
            None,
        );
        assert_eq!(
            Some(Piece::from(black_pawn_for_capture)),
            after_black_capture.get_piece(&e4)
        );
        assert_eq!(None, after_black_capture.get_piece(&d5));
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

    #[test]
    fn pawn_display_unicode_symbols() {
        let echiquier = Echiquier::new();
        let e2 = echiquier.get_case(1, 4);
        let e7 = echiquier.get_case(6, 4);
        assert_eq!("♟", Pawn::new(e2, true).to_string());
        assert_eq!("♙", Pawn::new(e7, false).to_string());
    }

    #[test]
    fn pawn_captures_en_passant_only_immediately_after_opponent_two_square_move() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);

        // White pawn on e5 (rank 4, file 4), Black pawn on d7 (rank 6, file 3)
        let e5 = echiquier.get_case(4, 4);
        let d7 = echiquier.get_case(6, 3);
        let d5 = echiquier.get_case(4, 3);
        let d6 = echiquier.get_case(5, 3);
        let h2 = echiquier.get_case(1, 7);
        let h3 = echiquier.get_case(2, 7);

        let mut white_pawn = Pawn::new(e5, true);
        white_pawn.last_move = Some(PieceMove { right: 0, up: 1 });
        let mut black_pawn = Pawn::new(d7, false);
        let mut other_white_pawn = Pawn::new(h2, true);
        position.set_piece(white_pawn);
        position.set_piece(black_pawn);
        position.set_piece(other_white_pawn);
        position.last_moved_piece = Some(Piece::from(white_pawn));

        // Black pawn moves 2 squares from d7 to d5
        let after_d5 =
            black_pawn.move_piece(&position, PieceMove { right: 0, up: -2 }, d5, None, None);

        let en_passant_move = PieceMove { right: -1, up: 1 };
        assert_eq!(
            vec![(en_passant_move, Piece::from(black_pawn))],
            white_pawn.allowed_captures(&after_d5)
        );
        assert!(white_pawn.is_capture_allowed(&after_d5, en_passant_move, black_pawn.into()));

        // If another move happens in between, en-passant is no longer allowed
        let after_other_move =
            other_white_pawn.move_piece(&after_d5, PieceMove { right: 0, up: 1 }, h3, None, None);
        assert!(white_pawn.allowed_captures(&after_other_move).is_empty());

        // Execute en-passant capture on `after_d5`: white pawn moves to d6 and black pawn on d5 is removed
        let after_en_passant =
            white_pawn.move_piece(&after_d5, en_passant_move, d6, Some(black_pawn.into()), None);
        assert_eq!(d6, white_pawn.case);
        assert_eq!(
            Some(Piece::from(white_pawn)),
            after_en_passant.get_piece(&d6)
        );
        assert_eq!(None, after_en_passant.get_piece(&e5));
        assert_eq!(None, after_en_passant.get_piece(&d5));
    }

    #[test]
    fn black_pawn_captures_en_passant() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);

        // Black pawn on d4 (rank 3, file 3), White pawn on e2 (rank 1, file 4)
        let d4 = echiquier.get_case(3, 3);
        let e2 = echiquier.get_case(1, 4);
        let e4 = echiquier.get_case(3, 4);
        let e3 = echiquier.get_case(2, 4);

        let mut black_pawn = Pawn::new(d4, false);
        black_pawn.last_move = Some(PieceMove { right: 0, up: -1 });
        let mut white_pawn = Pawn::new(e2, true);
        position.set_piece(black_pawn);
        position.set_piece(white_pawn);

        // White pawn moves 2 squares from e2 to e4
        let after_e4 =
            white_pawn.move_piece(&position, PieceMove { right: 0, up: 2 }, e4, None, None);

        let en_passant_move = PieceMove { right: 1, up: -1 };
        assert_eq!(
            vec![(en_passant_move, Piece::from(white_pawn))],
            black_pawn.allowed_captures(&after_e4)
        );

        let after_en_passant =
            black_pawn.move_piece(&after_e4, en_passant_move, e3, Some(white_pawn.into()), None);
        assert_eq!(e3, black_pawn.case);
        assert_eq!(
            Some(Piece::from(black_pawn)),
            after_en_passant.get_piece(&e3)
        );
        assert_eq!(None, after_en_passant.get_piece(&d4));
        assert_eq!(None, after_en_passant.get_piece(&e4));
    }

    #[test]
    fn promotion_piece_variants_and_display() {
        assert_eq!(PromotionPiece::Queen.to_string(), "Queen");
        assert_eq!(PromotionPiece::Rook.to_string(), "Rook");
        assert_eq!(PromotionPiece::Bishop.to_string(), "Bishop");
        assert_eq!(PromotionPiece::Knight.to_string(), "Knight");
    }

    #[test]
    fn white_pawn_promotion_to_all_pieces() {
        let echiquier = Echiquier::new();
        let e7 = echiquier.get_case(6, 4);
        let e8 = echiquier.get_case(7, 4);
        let one_up = PieceMove { right: 0, up: 1 };

        for (variant, expected_discriminant) in [
            (PromotionPiece::Queen, std::mem::discriminant(&Piece::Queen(Queen::new(e8, true)))),
            (PromotionPiece::Rook, std::mem::discriminant(&Piece::Rook(Rook::new(e8, true)))),
            (PromotionPiece::Bishop, std::mem::discriminant(&Piece::Bishop(Bishop::new(e8, true)))),
            (PromotionPiece::Knight, std::mem::discriminant(&Piece::Knight(Knight::new(e8, true)))),
        ] {
            let mut position = Position::new(&echiquier);
            let mut white_pawn = Pawn::new(e7, true);
            position.set_piece(white_pawn);

            let next_position = white_pawn.move_piece(
                &position,
                one_up,
                e8,
                None,
                Some(variant),
            );

            let piece_at_e8 = next_position.get_piece(&e8).expect("piece must be at e8");
            assert_eq!(std::mem::discriminant(&piece_at_e8), expected_discriminant);
            assert_eq!(Some(piece_at_e8), next_position.last_moved_piece);
            assert_eq!(None, next_position.get_piece(&e7));
        }
    }

    #[test]
    fn white_pawn_diagonal_capture_promotion() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e7 = echiquier.get_case(6, 4);
        let d8 = echiquier.get_case(7, 3);
        let mut white_pawn = Pawn::new(e7, true);
        let black_rook = Rook::new(d8, false);
        position.set_piece(white_pawn);
        position.set_piece(black_rook);

        let diag_capture = PieceMove { right: -1, up: 1 };
        let next_position = white_pawn.move_piece(
            &position,
            diag_capture,
            d8,
            Some(Piece::from(black_rook)),
            Some(PromotionPiece::Knight),
        );

        let piece_at_d8 = next_position.get_piece(&d8).expect("piece must be at d8");
        assert!(matches!(piece_at_d8, Piece::Knight(_)));
        assert_eq!(None, next_position.get_piece(&e7));
    }

    #[test]
    fn black_pawn_promotion_straight_and_capture() {
        let echiquier = Echiquier::new();
        let e2 = echiquier.get_case(1, 4);
        let e1 = echiquier.get_case(0, 4);
        let d1 = echiquier.get_case(0, 3);
        let one_down = PieceMove { right: 0, up: -1 };
        let diag_down = PieceMove { right: -1, up: -1 };

        // 1. Straight promotion to Queen
        let mut position1 = Position::new(&echiquier);
        let mut black_pawn = Pawn::new(e2, false);
        let white_dummy = Pawn::new(echiquier.get_case(1, 0), true);
        position1.set_piece(black_pawn);
        position1.set_piece(white_dummy);
        position1.last_moved_piece = Some(Piece::from(white_dummy));

        let next_position1 = black_pawn.move_piece(
            &position1,
            one_down,
            e1,
            None,
            Some(PromotionPiece::Queen),
        );
        let piece_at_e1 = next_position1.get_piece(&e1).expect("piece must be at e1");
        assert!(matches!(piece_at_e1, Piece::Queen(_)));

        // 2. Capture promotion to Bishop
        let mut position2 = Position::new(&echiquier);
        let mut black_pawn2 = Pawn::new(e2, false);
        let white_rook = Rook::new(d1, true);
        position2.set_piece(black_pawn2);
        position2.set_piece(white_rook);
        position2.last_moved_piece = Some(Piece::from(white_rook));

        let next_position2 = black_pawn2.move_piece(
            &position2,
            diag_down,
            d1,
            Some(Piece::from(white_rook)),
            Some(PromotionPiece::Bishop),
        );
        let piece_at_d1 = next_position2.get_piece(&d1).expect("piece must be at d1");
        assert!(matches!(piece_at_d1, Piece::Bishop(_)));
    }

    #[test]
    fn pawn_move_with_promotion_on_non_promotion_rank_keeps_pawn() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e2 = echiquier.get_case(1, 4);
        let e3 = echiquier.get_case(2, 4);
        let mut pawn = Pawn::new(e2, true);
        position.set_piece(pawn);

        let next_position = pawn.move_piece(
            &position,
            PieceMove { right: 0, up: 1 },
            e3,
            None,
            Some(PromotionPiece::Queen),
        );
        let piece_at_e3 = next_position.get_piece(&e3).expect("piece must be at e3");
        assert!(matches!(piece_at_e3, Piece::Pawn(_)));
    }
}
