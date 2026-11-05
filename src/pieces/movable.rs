use std::fmt::Display;
use std::ops::DerefMut;

use crate::echiquier::case::Case;
use crate::echiquier::position::Position;
use crate::echiquier::piece_move::PieceMove;
use crate::pieces::piece::Piece;

pub trait Movable: DerefMut<Target = Piece> + Display {
    fn candidate_moves(&self, position: &Position) -> Vec<PieceMove>;

    fn allowed_moves(&self, position: &Position) -> Vec<PieceMove> {
        let mut moves = Vec::new();
        for piece_move in self.candidate_moves(position) {
            if position.echiquier.exists_relative_case(&self.case, piece_move) {
                moves.push(piece_move);
            }
        }
        moves
    }

    fn is_move_allowed(&self, position: &Position, piece_move: PieceMove) -> bool {
        return (&self).allowed_moves(position).contains(&piece_move);
    }

    fn move_piece<'a>(&mut self, position: &Position<'a>, piece_move: PieceMove, case: Case) -> Position<'a> {
        if !self.is_move_allowed(position, piece_move) {
            panic!("forbidden move!");
        }
        let mut next_position = *position;
        let old_index = (self.case.rank * 8 + self.case.file) as usize;
        next_position.pieces[old_index] = None;
        self.case = case;
        self.last_move = Some(piece_move);
        next_position.set_piece(**self);
        next_position
    }
}
