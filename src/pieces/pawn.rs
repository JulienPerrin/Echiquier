use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

use super::movable::Movable;
use crate::echiquier::case::Case;
use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::pieces::piece::Piece;
use crate::pieces::piece_kind::PieceKind;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pawn {
    pub piece: Piece,
}

impl Pawn {
    pub const fn new(case: Case, is_white: bool) -> Self {
        Pawn {
            piece: Piece::new(case, PieceKind::PAWN, is_white),
        }
    }
}

impl Deref for Pawn {
    type Target = Piece;

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

        if self.last_move.is_none() {
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

        for right in [-1, 1] {
            let diagonal_move = PieceMove {
                right,
                up: direction,
            };
            if let Some(target_piece) = position.get_relative_piece(&self.case, diagonal_move)
                && target_piece.is_white != self.is_white
            {
                captures.push((diagonal_move, target_piece));
            }
        }

        captures
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
        position.set_piece(*pawn);

        let one_up = PieceMove { right: 0, up: 1 };
        let two_up = PieceMove { right: 0, up: 2 };
        let allowed = pawn.allowed_moves(&position);
        assert_eq!(vec![one_up, two_up], allowed);
        assert!(pawn.is_move_allowed(&position, one_up));
        assert!(pawn.is_move_allowed(&position, two_up));

        let e3 = position.echiquier.get_case(2, 4);
        let next_position = pawn.move_piece(&position, one_up, e3, None);
        assert_eq!(e3, pawn.case);
        assert_eq!(
            Some(*pawn),
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
        position.set_piece(*pawn);

        let one_down = PieceMove { right: 0, up: -1 };
        let two_down = PieceMove { right: 0, up: -2 };
        let allowed = pawn.allowed_moves(&position);
        assert_eq!(vec![one_down, two_down], allowed);
        assert!(pawn.is_move_allowed(&position, one_down));
        assert!(pawn.is_move_allowed(&position, two_down));

        let e5 = position.echiquier.get_case(4, 4);
        let next_position = pawn.move_piece(&position, two_down, e5, None);
        assert_eq!(e5, pawn.case);
        assert_eq!(
            Some(*pawn),
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
        position.set_piece(*pawn);

        let mut blocked_two_squares = position;
        blocked_two_squares.set_piece(*Pawn::new(e4, false));
        assert_eq!(
            vec![PieceMove { right: 0, up: 1 }],
            pawn.allowed_moves(&blocked_two_squares)
        );

        let mut blocked_immediately = position;
        blocked_immediately.set_piece(*Pawn::new(e3, false));
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
        position.set_piece(*white_pawn);
        position.set_piece(*black_pawn_d5);
        position.set_piece(*friendly_pawn_f5);

        let capture_move = PieceMove { right: -1, up: 1 };
        assert_eq!(
            vec![(capture_move, *black_pawn_d5)],
            white_pawn.allowed_captures(&position)
        );
        assert!(white_pawn.is_capture_allowed(&position, capture_move, *black_pawn_d5));

        let next_position =
            white_pawn.move_piece(&position, capture_move, d5, Some(*black_pawn_d5));
        assert_eq!(d5, white_pawn.case);
        assert_eq!(Some(*white_pawn), next_position.get_piece(&d5));
        assert_eq!(None, next_position.get_piece(&e4));

        let mut black_pawn_for_capture = black_pawn_d5;
        let black_capture_move = PieceMove { right: 1, up: -1 };
        let taken_white_pawn = *Pawn::new(e4, true);
        assert_eq!(
            vec![(black_capture_move, taken_white_pawn)],
            black_pawn_for_capture.allowed_captures(&position)
        );
        let after_black_capture = black_pawn_for_capture.move_piece(
            &position,
            black_capture_move,
            e4,
            Some(taken_white_pawn),
        );
        assert_eq!(
            Some(*black_pawn_for_capture),
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
}
