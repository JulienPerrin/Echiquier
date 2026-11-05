use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::pieces::movable::Movable;

#[derive(Clone, Debug, PartialEq)]
pub struct Game<'a> {
    pub initial_position: Position<'a>,
    pub positions: Vec<Position<'a>>,
}

impl<'a> Game<'a> {
    pub fn new(initial_position: Position<'a>) -> Game<'a> {
        Game {
            initial_position,
            positions: vec![initial_position],
        }
    }

    pub fn current_position(&self) -> &Position<'a> {
        self.positions.last().unwrap_or(&self.initial_position)
    }

    pub fn play_move<T: Movable>(&mut self, piece: &mut T, piece_move: PieceMove) -> &Position<'a> {
        let current = *self.current_position();
        let target_case = current.echiquier.get_relative_case(&piece.case, piece_move);
        let next_position = piece.move_piece(&current, piece_move, target_case);
        self.positions.push(next_position);
        self.current_position()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::echiquier::echiquier::Echiquier;
    use crate::pieces::pawn::Pawn;

    #[test]
    fn game_tracks_initial_and_played_positions() {
        let echiquier = Echiquier::new();
        let mut initial_position = Position::new(&echiquier);
        let e2 = initial_position.echiquier.get_case(1, 4);
        let mut pawn = Pawn::new(e2, true);
        initial_position.set_piece(*pawn);

        let mut game = Game::new(initial_position);
        assert_eq!(1, game.positions.len());
        assert_eq!(initial_position, *game.current_position());

        let one_up = PieceMove { right: 0, up: 1 };
        let next_position = *game.play_move(&mut pawn, one_up);

        assert_eq!(2, game.positions.len());
        assert_eq!(initial_position, game.initial_position);
        assert_eq!(initial_position, game.positions[0]);
        assert_eq!(next_position, game.positions[1]);
        assert_eq!(next_position, *game.current_position());
    }
}
