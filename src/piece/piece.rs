use crate::echiquier::case::Case;
use crate::echiquier::echiquier::Echiquier;

pub trait Piece {
    fn new(case: Case, echiquier: Echiquier) -> Self;
    fn allowed_moves(&mut self) -> Vec<Case>;
    fn is_move_allowed(&mut self, case: &Case) -> bool {
        return self.allowed_moves().contains(case);
    }
    fn move_piece(&mut self, case: Case);
}
