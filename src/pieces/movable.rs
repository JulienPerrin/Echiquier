use std::fmt::Display;
use std::ops::DerefMut;

use crate::echiquier::case::Case;
use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::pieces::piece::{Piece, PieceData};

pub trait Movable: DerefMut<Target = PieceData> + Display {
    fn candidate_moves(&self, position: &Position) -> Vec<PieceMove>;

    fn sliding_candidate_moves(
        &self,
        position: &Position,
        directions: &[(i8, i8)],
    ) -> Vec<PieceMove> {
        let mut moves = Vec::new();

        for &(right_step, up_step) in directions {
            let mut distance = 1;
            let mut blocked = false;
            while !blocked {
                let piece_move = PieceMove {
                    right: right_step * distance,
                    up: up_step * distance,
                };
                if let Some(target_case) =
                    position.echiquier.get_relative_case(&self.case, piece_move)
                {
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

    fn allowed_moves(&self, position: &Position) -> Vec<PieceMove> {
        let mut moves = Vec::new();
        for piece_move in self.candidate_moves(position) {
            if let Some(target_case) = position.echiquier.get_relative_case(&self.case, piece_move)
                && !position.is_case_occupied(&target_case)
            {
                moves.push(piece_move);
            }
        }
        moves
    }

    fn allowed_captures(&self, position: &Position) -> Vec<(PieceMove, Piece)> {
        let mut captures = Vec::new();
        for (piece_move, piece_taken) in self.candidate_captures(position) {
            if !matches!(piece_taken, Piece::King(_))
                && position
                    .echiquier
                    .get_relative_case(&self.case, piece_move)
                    .is_some()
            {
                captures.push((piece_move, piece_taken));
            }
        }
        captures
    }

    fn is_move_allowed(&self, position: &Position, piece_move: PieceMove) -> bool {
        return (&self).allowed_moves(position).contains(&piece_move);
    }

    fn is_capture_allowed(
        &self,
        position: &Position,
        piece_move: PieceMove,
        piece_taken: Piece,
    ) -> bool {
        return (&self)
            .allowed_captures(position)
            .contains(&(piece_move, piece_taken));
    }

    fn move_piece<'a>(
        &mut self,
        position: &Position<'a>,
        piece_move: PieceMove,
        case: Case,
        piece_taken: Option<Piece>,
    ) -> Position<'a>
    where
        Self: Sized + Copy,
        Piece: From<Self>,
    {
        let mut next_position = *position;
        match piece_taken {
            Some(taken) => {
                if !self.is_capture_allowed(position, piece_move, taken) {
                    panic!("forbidden capture!");
                }
                next_position.remove_piece(&taken);
            }
            None => {
                if !self.is_move_allowed(position, piece_move) {
                    panic!("forbidden move!");
                }
            }
        }
        next_position.remove_piece(self);
        self.case = case;
        self.last_move = Some(piece_move);
        let moved_piece = Piece::from(*self);
        next_position.set_piece(moved_piece);
        next_position.last_moved_piece = Some(moved_piece);
        next_position
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::echiquier::echiquier::Echiquier;
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
        pawn.move_piece(&position, forbidden_move, f2, None);
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

        rook.move_piece(&position, PieceMove { right: 0, up: 2 }, a3, None);
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
}
