use core::panic;
use std::fmt::Display;
use std::ops::DerefMut;

use crate::echiquier::case::Case;
use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::pieces::king::King;
use crate::pieces::pawn::PromotionPiece;
use crate::pieces::piece::{Piece, PieceData};

pub trait Movable: DerefMut<Target = PieceData> + Display {
    fn candidate_moves(&self, position: &Position) -> Vec<PieceMove>;

    fn sliding_candidate_moves(
        &self,
        position: &Position,
        directions: &[(i8, i8)],
    ) -> Vec<PieceMove> {
        let mut moves = Vec::new();
        let echiquier = position.echiquier;

        for &(right_step, up_step) in directions {
            let mut distance = 1;
            let mut blocked = false;
            while !blocked {
                let piece_move = PieceMove {
                    right: right_step * distance,
                    up: up_step * distance,
                };
                if let Some(target_case) = echiquier.get_relative_case(&self.case, piece_move) {
                    moves.push(piece_move);
                    blocked = position.is_case_occupied(&target_case);
                    distance += 1;
                } else {
                    blocked = true;
                }
            }
        }

        moves
    }

    fn candidate_captures(&self, position: &Position) -> Vec<(PieceMove, Piece)> {
        let mut captures = Vec::new();

        for piece_move in self.candidate_moves(position) {
            if let Some(target_piece) = position.get_relative_piece(&self.case, piece_move)
                && target_piece.is_white != self.is_white
            {
                captures.push((piece_move, target_piece));
            }
        }

        captures
    }

    fn leaves_king_in_check(
        &self,
        position: &Position,
        target_case: Case,
        piece_taken: Option<&Piece>,
    ) -> bool {
        let Some(piece) = position.get_piece(&self.case) else {
            return false;
        };
        let mut next_position = *position;
        next_position.remove_piece(self);
        if let Some(taken) = piece_taken {
            next_position.remove_piece(taken);
        }
        next_position.set_piece(piece.with_case(target_case));
        next_position
            .get_pieces::<King>(self.is_white)
            .into_iter()
            .any(|king| king.is_in_check(&next_position))
    }

    fn allowed_moves(&self, position: &Position) -> Vec<PieceMove> {
        let mut moves = Vec::new();
        for piece_move in self.candidate_moves(position) {
            if let Some(target_case) = position.echiquier.get_relative_case(&self.case, piece_move)
                && !position.is_case_occupied(&target_case)
                && !self.leaves_king_in_check(position, target_case, None)
            {
                moves.push(piece_move);
            }
        }
        moves.extend(self.castling_moves(position));
        moves.extend(self.allowed_captures(position).into_iter().map(|(m, _)| m));
        moves
    }

    fn castling_moves(&self, _position: &Position) -> Vec<PieceMove> {
        Vec::new()
    }

    fn allowed_captures(&self, position: &Position) -> Vec<(PieceMove, Piece)> {
        let mut captures = Vec::new();
        for (piece_move, piece_taken) in self.candidate_captures(position) {
            if !matches!(piece_taken, Piece::King(_))
                && let Some(target_case) =
                    position.echiquier.get_relative_case(&self.case, piece_move)
                && !self.leaves_king_in_check(position, target_case, Some(&piece_taken))
            {
                captures.push((piece_move, piece_taken));
            }
        }
        captures
    }

    fn is_move_allowed(&self, position: &Position, piece_move: PieceMove) -> bool {
        self.allowed_moves(position).contains(&piece_move)
    }

    fn is_capture_allowed(
        &self,
        position: &Position,
        piece_move: PieceMove,
        piece_taken: Piece,
    ) -> bool {
        self.allowed_captures(position)
            .contains(&(piece_move, piece_taken))
    }

    fn has_never_moved(&self) -> bool {
        self.last_move.is_none()
    }

    fn is_turn(&self, position: &Position) -> bool {
        position.is_turn(self.is_white)
    }

    fn promoted_piece(
        &self,
        _target_case: Case,
        _piece_move: PieceMove,
        _promotion: PromotionPiece,
    ) -> Option<Piece> {
        panic!("Not implemented")
    }

    fn after_move<'a>(&self, _position: &mut Position<'a>, _piece_move: PieceMove) {}

    fn move_piece<'a>(
        &mut self,
        position: &Position<'a>,
        piece_move: PieceMove,
        case: Case,
        piece_taken: Option<Piece>,
        promotion: Option<PromotionPiece>,
    ) -> Position<'a>
    where
        Self: Sized + Copy,
        Piece: From<Self>,
    {
        if !self.is_turn(position) {
            panic!("not your turn!");
        }
        let mut next_position = *position;
        match piece_taken {
            Some(taken) => {
                if !self.is_capture_allowed(position, piece_move, taken) {
                    panic!("forbidden capture!");
                }
                next_position.remove_piece(&taken);
            }
            None => {
                if position.is_case_occupied(&case) || !self.is_move_allowed(position, piece_move) {
                    panic!("forbidden move!");
                }
            }
        }
        next_position.remove_piece(self);
        self.case = case;
        self.last_move = Some(piece_move);
        let moved_piece = match promotion {
            Some(promo) => match self.promoted_piece(case, piece_move, promo) {
                Some(piece) => piece,
                None => Piece::from(*self),
            },
            None => Piece::from(*self),
        };
        next_position.set_piece(moved_piece);
        self.after_move(&mut next_position, piece_move);
        next_position.last_moved_piece = Some(moved_piece);
        next_position.halfmove_clock = if piece_taken.is_some() || matches!(Piece::from(*self), Piece::Pawn(_)) { 0 } else { position.halfmove_clock + 1 };
        if !self.is_white {
            next_position.fullmove_number = position.fullmove_number + 1;
        }
        next_position
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::echiquier::echiquier::Echiquier;
    use crate::pieces::bishop::Bishop;
    use crate::pieces::king::King;
    use crate::pieces::pawn::Pawn;
    use crate::pieces::rook::Rook;

    #[test]
    #[should_panic(expected = "forbidden move!")]
    fn move_piece_panics_on_forbidden_move() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e2 = position.echiquier.get_case(1, 4);
        let mut pawn = Pawn::new(e2, true);
        position.set_piece(pawn);

        let forbidden_move = PieceMove { right: 1, up: 0 };
        let f2 = position.echiquier.get_case(1, 5);
        pawn.move_piece(&position, forbidden_move, f2, None, None);
    }

    #[test]
    #[should_panic(expected = "forbidden capture!")]
    fn move_piece_panics_on_forbidden_capture() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e4 = position.echiquier.get_case(3, 4);
        let mut king = King::new(e4, true);
        position.set_piece(king);

        let e5 = position.echiquier.get_case(4, 4);
        let friendly_pawn = Pawn::new(e5, true);
        position.set_piece(friendly_pawn);

        king.move_piece(
            &position,
            PieceMove { right: 0, up: 1 },
            e5,
            Some(friendly_pawn.into()),
            None,
        );
    }

    #[test]
    #[should_panic(expected = "forbidden move!")]
    fn move_piece_panics_on_quiet_move_onto_occupied_case() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let a1 = position.echiquier.get_case(0, 0);
        let mut rook = Rook::new(a1, true);
        position.set_piece(rook);

        let a3 = position.echiquier.get_case(2, 0);
        position.set_piece(Pawn::new(a3, true));

        rook.move_piece(&position, PieceMove { right: 0, up: 2 }, a3, None, None);
    }

    #[test]
    #[should_panic(expected = "not your turn!")]
    fn move_piece_panics_if_not_his_turn() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let a1 = position.echiquier.get_case(0, 0);
        let mut rook = Rook::new(a1, true);
        position.set_piece(rook);
        // White moved last, so it is Black's turn, not White's!
        position.last_moved_piece = Some(Piece::from(rook));

        let b1 = position.echiquier.get_case(0, 1);
        rook.move_piece(&position, PieceMove { right: 1, up: 0 }, b1, None, None);
    }

    #[test]
    fn piece_is_turn_reflects_player_turn() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let a1 = position.echiquier.get_case(0, 0);
        let a8 = position.echiquier.get_case(7, 0);
        let white_rook = Rook::new(a1, true);
        let black_rook = Rook::new(a8, false);

        // Initially White's turn
        assert!(white_rook.is_turn(&position));
        assert!(!black_rook.is_turn(&position));

        // After White moves, it is Black's turn
        position.last_moved_piece = Some(Piece::from(white_rook));
        assert!(!white_rook.is_turn(&position));
        assert!(black_rook.is_turn(&position));
    }

    #[test]
    fn allowed_captures_excludes_king() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e4 = position.echiquier.get_case(3, 4);
        let d5 = position.echiquier.get_case(4, 3);
        let f5 = position.echiquier.get_case(4, 5);
        let white_pawn = Pawn::new(e4, true);
        let black_king = King::new(d5, false);
        let black_pawn = Pawn::new(f5, false);
        position.set_piece(white_pawn);
        position.set_piece(black_king);
        position.set_piece(black_pawn);

        let king_capture_move = PieceMove { right: -1, up: 1 };
        let pawn_capture_move = PieceMove { right: 1, up: 1 };
        assert_eq!(
            vec![
                (king_capture_move, Piece::from(black_king)),
                (pawn_capture_move, Piece::from(black_pawn)),
            ],
            white_pawn.candidate_captures(&position)
        );
        assert_eq!(
            vec![(pawn_capture_move, Piece::from(black_pawn))],
            white_pawn.allowed_captures(&position)
        );
        assert!(!white_pawn.is_capture_allowed(&position, king_capture_move, black_king.into()));
    }

    #[test]
    fn sliding_candidate_moves_stops_at_boundary_and_occupied_case() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let a1 = position.echiquier.get_case(0, 0);
        let king = King::new(a1, true);
        position.set_piece(king);

        let a2 = position.echiquier.get_case(1, 0);
        position.set_piece(Pawn::new(a2, false));

        // Going left (-1, 0) immediately hits boundary (line 23 break)
        // Going up (0, 1) hits occupied a2 (line 27 break)
        let moves = king.sliding_candidate_moves(&position, &[(-1, 0), (0, 1)]);
        assert_eq!(vec![PieceMove { right: 0, up: 1 }], moves);
    }

    #[test]
    fn sliding_candidate_moves_slides_multiple_squares() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let a1 = position.echiquier.get_case(0, 0);
        let rook = Rook::new(a1, true);
        position.set_piece(rook);

        let a3 = position.echiquier.get_case(2, 0);
        position.set_piece(Pawn::new(a3, true));

        // Going up: a2 (distance 1, empty) then a3 (distance 2, occupied)
        let moves = rook.sliding_candidate_moves(&position, &[(0, 1)]);
        assert_eq!(
            vec![PieceMove { right: 0, up: 1 }, PieceMove { right: 0, up: 2 },],
            moves
        );
    }

    #[test]
    fn allowed_moves_prevents_king_from_moving_into_check() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e1 = position.echiquier.get_case(0, 4);
        let d8 = position.echiquier.get_case(7, 3);
        let king = King::new(e1, true);
        let rook = Rook::new(d8, false);
        position.set_piece(king);
        position.set_piece(rook);

        let moves = king.allowed_moves(&position);
        let d1_move = PieceMove { right: -1, up: 0 };
        let d2_move = PieceMove { right: -1, up: 1 };
        assert!(!moves.contains(&d1_move));
        assert!(!moves.contains(&d2_move));
        assert!(moves.contains(&PieceMove { right: 0, up: 1 }));
        assert!(moves.contains(&PieceMove { right: 1, up: 0 }));
        assert!(moves.contains(&PieceMove { right: 1, up: 1 }));
    }

    #[test]
    fn allowed_moves_prevents_pinned_piece_from_leaving_king_in_check() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e1 = position.echiquier.get_case(0, 4);
        let e2 = position.echiquier.get_case(1, 4);
        let e8 = position.echiquier.get_case(7, 4);
        let king = King::new(e1, true);
        let bishop = Bishop::new(e2, true);
        let enemy_rook = Rook::new(e8, false);
        position.set_piece(king);
        position.set_piece(bishop);
        position.set_piece(enemy_rook);

        let moves = bishop.allowed_moves(&position);
        assert!(moves.is_empty());
    }

    #[test]
    fn allowed_moves_allows_interposing_piece_to_block_check() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e1 = position.echiquier.get_case(0, 4);
        let b4 = position.echiquier.get_case(3, 1);
        let e8 = position.echiquier.get_case(7, 4);
        let king = King::new(e1, true);
        let bishop = Bishop::new(b4, true);
        let enemy_rook = Rook::new(e8, false);
        position.set_piece(king);
        position.set_piece(bishop);
        position.set_piece(enemy_rook);

        let moves = bishop.allowed_moves(&position);
        let block_move = PieceMove { right: 3, up: 3 };
        assert_eq!(vec![block_move], moves);
    }

    #[test]
    fn allowed_moves_allows_king_moving_out_of_check_to_safe_square() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e1 = position.echiquier.get_case(0, 4);
        let e8 = position.echiquier.get_case(7, 4);
        let king = King::new(e1, true);
        let enemy_rook = Rook::new(e8, false);
        position.set_piece(king);
        position.set_piece(enemy_rook);

        let moves = king.allowed_moves(&position);
        let e2_move = PieceMove { right: 0, up: 1 };
        assert!(!moves.contains(&e2_move));
        assert!(moves.contains(&PieceMove { right: -1, up: 0 }));
        assert!(moves.contains(&PieceMove { right: 1, up: 0 }));
        assert!(moves.contains(&PieceMove { right: -1, up: 1 }));
        assert!(moves.contains(&PieceMove { right: 1, up: 1 }));
    }

    #[test]
    fn allowed_moves_when_piece_not_in_position() {
        let echiquier = Echiquier::new();
        let position = Position::new(&echiquier);
        let e2 = position.echiquier.get_case(1, 4);
        let pawn = Pawn::new(e2, true);
        let moves = pawn.allowed_moves(&position);
        assert!(!moves.is_empty());
    }

    #[test]
    fn move_piece_without_promotion_keeps_piece() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e7 = position.echiquier.get_case(6, 4);
        let e8 = position.echiquier.get_case(7, 4);
        let mut pawn = Pawn::new(e7, true);
        position.set_piece(pawn);

        let next_position = pawn.move_piece(&position, PieceMove { right: 0, up: 1 }, e8, None, None);
        let piece_at_e8 = next_position.get_piece(&e8).expect("piece must be at e8");
        assert!(matches!(piece_at_e8, Piece::Pawn(_)));
    }

    #[test]
    fn move_piece_with_promotion_promotes_piece() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e7 = position.echiquier.get_case(6, 4);
        let e8 = position.echiquier.get_case(7, 4);
        let mut pawn = Pawn::new(e7, true);
        position.set_piece(pawn);

        let next_position = pawn.move_piece(
            &position,
            PieceMove { right: 0, up: 1 },
            e8,
            None,
            Some(PromotionPiece::Queen),
        );
        let piece_at_e8 = next_position.get_piece(&e8).expect("piece must be at e8");
        assert!(matches!(piece_at_e8, Piece::Queen(_)));
    }

    #[test]
    #[should_panic(expected = "Not implemented")]
    fn move_piece_with_promotion_on_non_pawn_panics() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let a1 = position.echiquier.get_case(0, 0);
        let a2 = position.echiquier.get_case(1, 0);
        let mut rook = Rook::new(a1, true);
        position.set_piece(rook);

        rook.move_piece(
            &position,
            PieceMove { right: 0, up: 1 },
            a2,
            None,
            Some(PromotionPiece::Queen),
        );
    }

    #[test]
    fn allowed_captures_prevents_pinned_piece_from_capturing_and_leaving_king_in_check() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e1 = position.echiquier.get_case(0, 4);
        let e2 = position.echiquier.get_case(1, 4);
        let e8 = position.echiquier.get_case(7, 4);
        let d3 = position.echiquier.get_case(2, 3);
        let king = King::new(e1, true);
        let bishop = Bishop::new(e2, true);
        let enemy_rook = Rook::new(e8, false);
        let enemy_pawn = Pawn::new(d3, false);

        position.set_piece(king);
        position.set_piece(bishop);
        position.set_piece(enemy_rook);
        position.set_piece(enemy_pawn);

        // Bishop is pinned along the e-file by enemy rook at e8.
        // Capturing the pawn at d3 would leave king in check, so allowed_captures must be empty.
        let captures = bishop.allowed_captures(&position);
        assert!(captures.is_empty());
    }

    #[test]
    fn allowed_captures_allows_capturing_pinner_or_interposing_attacker() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e1 = position.echiquier.get_case(0, 4);
        let c3 = position.echiquier.get_case(2, 2);
        let a5 = position.echiquier.get_case(4, 0);
        let king = King::new(e1, true);
        let bishop = Bishop::new(c3, true);
        let enemy_bishop = Bishop::new(a5, false);

        position.set_piece(king);
        position.set_piece(bishop);
        position.set_piece(enemy_bishop);

        // Bishop pinned along diagonal a5-e1 by enemy bishop on a5.
        // Capturing the pinner on a5 resolves check, so it must be allowed!
        let captures = bishop.allowed_captures(&position);
        let capture_pinner = PieceMove { right: -2, up: 2 };
        assert_eq!(vec![(capture_pinner, Piece::from(enemy_bishop))], captures);
    }

    #[test]
    fn king_cannot_capture_defended_piece() {
        let echiquier = Echiquier::new();
        let mut position = Position::new(&echiquier);
        let e1 = position.echiquier.get_case(0, 4);
        let e2 = position.echiquier.get_case(1, 4);
        let e8 = position.echiquier.get_case(7, 4);
        let king = King::new(e1, true);
        let enemy_pawn = Pawn::new(e2, false);
        let enemy_rook = Rook::new(e8, false);

        position.set_piece(king);
        position.set_piece(enemy_pawn);
        position.set_piece(enemy_rook);

        // Pawn on e2 is defended by Rook on e8 along column e.
        // King cannot capture pawn on e2 because it leaves king in check.
        let captures = king.allowed_captures(&position);
        assert!(captures.is_empty());
    }
}
