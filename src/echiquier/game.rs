use crate::echiquier::case::Case;
use crate::echiquier::echiquier::Echiquier;
use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::pieces::king::King;
use crate::pieces::movable::Movable;
use crate::pieces::pawn::Pawn;

pub const ECHIQUIER: Echiquier = Echiquier::new();

pub const INITIAL_POSITION: Position<'static> = Position {
    echiquier: &ECHIQUIER,
    pieces: [
        None,
        None,
        None,
        None,
        Some(King::new(Case { rank: 0, file: 4 }, true).piece),
        None,
        None,
        None,
        Some(Pawn::new(Case { rank: 1, file: 0 }, true).piece),
        Some(Pawn::new(Case { rank: 1, file: 1 }, true).piece),
        Some(Pawn::new(Case { rank: 1, file: 2 }, true).piece),
        Some(Pawn::new(Case { rank: 1, file: 3 }, true).piece),
        Some(Pawn::new(Case { rank: 1, file: 4 }, true).piece),
        Some(Pawn::new(Case { rank: 1, file: 5 }, true).piece),
        Some(Pawn::new(Case { rank: 1, file: 6 }, true).piece),
        Some(Pawn::new(Case { rank: 1, file: 7 }, true).piece),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(Pawn::new(Case { rank: 6, file: 0 }, false).piece),
        Some(Pawn::new(Case { rank: 6, file: 1 }, false).piece),
        Some(Pawn::new(Case { rank: 6, file: 2 }, false).piece),
        Some(Pawn::new(Case { rank: 6, file: 3 }, false).piece),
        Some(Pawn::new(Case { rank: 6, file: 4 }, false).piece),
        Some(Pawn::new(Case { rank: 6, file: 5 }, false).piece),
        Some(Pawn::new(Case { rank: 6, file: 6 }, false).piece),
        Some(Pawn::new(Case { rank: 6, file: 7 }, false).piece),
        None,
        None,
        None,
        None,
        Some(King::new(Case { rank: 7, file: 4 }, false).piece),
        None,
        None,
        None,
    ],
};

#[derive(Clone, Debug, PartialEq)]
pub struct Game<'a> {
    pub initial_position: Position<'a>,
    pub positions: Vec<Position<'a>>,
}

impl Game<'static> {
    pub fn default_game() -> Game<'static> {
        Game::new(INITIAL_POSITION)
    }
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
        let target_case = current
            .echiquier
            .get_relative_case(&piece.case, piece_move)
            .expect("target case out of bounds!");
        let piece_taken = current.get_piece(&target_case);
        let next_position = piece.move_piece(&current, piece_move, target_case, piece_taken);
        self.positions.push(next_position);
        self.current_position()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_tracks_initial_and_played_positions() {
        let mut game = Game::default_game();
        assert_eq!(1, game.positions.len());
        assert_eq!(INITIAL_POSITION, *game.current_position());

        let e2 = INITIAL_POSITION.echiquier.get_case(1, 4);
        let mut white_pawn = Pawn::new(e2, true);
        let two_up = PieceMove { right: 0, up: 2 };
        let next_position = *game.play_move(&mut white_pawn, two_up);

        assert_eq!(2, game.positions.len());
        assert_eq!(INITIAL_POSITION, game.initial_position);
        assert_eq!(INITIAL_POSITION, game.positions[0]);
        assert_eq!(next_position, game.positions[1]);
        assert_eq!(next_position, *game.current_position());

        let d7 = INITIAL_POSITION.echiquier.get_case(6, 3);
        let mut black_pawn = Pawn::new(d7, false);
        let two_down = PieceMove { right: 0, up: -2 };
        game.play_move(&mut black_pawn, two_down);

        let after_capture = *game.play_move(&mut white_pawn, PieceMove { right: -1, up: 1 });
        assert_eq!(4, game.positions.len());
        assert_eq!(after_capture, *game.current_position());
        let d5 = INITIAL_POSITION.echiquier.get_case(4, 3);
        assert_eq!(Some(*white_pawn), after_capture.get_piece(&d5));
    }
}
