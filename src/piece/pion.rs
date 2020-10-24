use crate::echiquier::case::Case;
use crate::echiquier::deplacement::Deplacement;
use crate::echiquier::echiquier::Echiquier;

use super::piece::Piece;

pub struct Pion {
    case: Case,
    echiquier: Echiquier,
}

impl Piece for Pion {
    fn new(case: Case, echiquier: Echiquier) -> Self {
        return Pion {
            case: case,
            echiquier: echiquier,
        };
    }

    fn allowed_moves(&mut self) -> Vec<Case> {
        let mut result = Vec::new();
        let one_up: Deplacement = Deplacement { right: 0, up: 1 };
        if self.echiquier.exists_relative_case(&self.case, one_up) {
            result.push(self.echiquier.get_relative_case(&self.case, one_up));
        }
        let two_up: Deplacement = Deplacement { right: 0, up: 2 };
        if self.echiquier.exists_relative_case(&self.case, two_up) {
            result.push(self.echiquier.get_relative_case(&self.case, two_up));
        }
        return result;
    }

    fn move_piece(&mut self, case: Case) {
        if !self.is_move_allowed(&case) {
            panic!("forbidden move!");
        }
        self.case = case;
    }
}
