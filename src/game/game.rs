use std::fmt::{self, Display};

use crate::echiquier::case::Case;
use crate::echiquier::echiquier::Echiquier;
use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::pieces::bishop::Bishop;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::movable::Movable;
use crate::pieces::pawn::Pawn;
use crate::pieces::piece::Piece;
use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;

pub const ECHIQUIER: Echiquier = Echiquier::new();

pub const INITIAL_POSITION: Position<'static> = Position {
    echiquier: &ECHIQUIER,
    pieces: [
        Some(Piece::Rook(Rook::new(Case { rank: 0, file: 0 }, true))),
        Some(Piece::Knight(Knight::new(Case { rank: 0, file: 1 }, true))),
        Some(Piece::Bishop(Bishop::new(Case { rank: 0, file: 2 }, true))),
        Some(Piece::Queen(Queen::new(Case { rank: 0, file: 3 }, true))),
        Some(Piece::King(King::new(Case { rank: 0, file: 4 }, true))),
        Some(Piece::Bishop(Bishop::new(Case { rank: 0, file: 5 }, true))),
        Some(Piece::Knight(Knight::new(Case { rank: 0, file: 6 }, true))),
        Some(Piece::Rook(Rook::new(Case { rank: 0, file: 7 }, true))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 1, file: 0 }, true))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 1, file: 1 }, true))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 1, file: 2 }, true))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 1, file: 3 }, true))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 1, file: 4 }, true))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 1, file: 5 }, true))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 1, file: 6 }, true))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 1, file: 7 }, true))),
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
        Some(Piece::Pawn(Pawn::new(Case { rank: 6, file: 0 }, false))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 6, file: 1 }, false))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 6, file: 2 }, false))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 6, file: 3 }, false))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 6, file: 4 }, false))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 6, file: 5 }, false))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 6, file: 6 }, false))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 6, file: 7 }, false))),
        Some(Piece::Rook(Rook::new(Case { rank: 7, file: 0 }, false))),
        Some(Piece::Knight(Knight::new(Case { rank: 7, file: 1 }, false))),
        Some(Piece::Bishop(Bishop::new(Case { rank: 7, file: 2 }, false))),
        Some(Piece::Queen(Queen::new(Case { rank: 7, file: 3 }, false))),
        Some(Piece::King(King::new(Case { rank: 7, file: 4 }, false))),
        Some(Piece::Bishop(Bishop::new(Case { rank: 7, file: 5 }, false))),
        Some(Piece::Knight(Knight::new(Case { rank: 7, file: 6 }, false))),
        Some(Piece::Rook(Rook::new(Case { rank: 7, file: 7 }, false))),
    ],
    last_moved_piece: None,
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

    pub fn play_move<T>(&mut self, piece: &mut T, piece_move: PieceMove) -> &Position<'a>
    where
        T: Movable + Copy,
        Piece: From<T>,
    {
        let current = *self.current_position();
        let target_case = current.echiquier.get_relative_case(&piece.case, piece_move).expect("target case out of bounds!");
        let piece_taken = current.get_piece(&target_case).or_else(|| {
            piece
                .allowed_captures(&current)
                .into_iter()
                .find(|&(m, _)| m == piece_move)
                .map(|(_, taken)| taken)
        });
        let next_position = piece.move_piece(&current, piece_move, target_case, piece_taken);
        self.positions.push(next_position);
        self.current_position()
    }
}

impl<'a> Display for Game<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.current_position())
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
        assert_eq!(Some(Piece::from(white_pawn)), after_capture.get_piece(&d5));
    }

    #[test]
    #[should_panic(expected = "target case out of bounds!")]
    fn play_move_panics_on_out_of_bounds() {
        let mut game = Game::default_game();
        let e2 = INITIAL_POSITION.echiquier.get_case(1, 4);
        let mut white_pawn = Pawn::new(e2, true);
        game.play_move(&mut white_pawn, PieceMove { right: 0, up: 10 });
    }

    #[test]
    fn game_display_matches_current_position() {
        let game = Game::default_game();
        assert_eq!(game.current_position().to_string(), game.to_string());
    }

    #[test]
    fn game_play_move_handles_en_passant_capture() {
        let mut game = Game::default_game();
        let e2 = INITIAL_POSITION.echiquier.get_case(1, 4);
        let e5 = INITIAL_POSITION.echiquier.get_case(4, 4);
        let d7 = INITIAL_POSITION.echiquier.get_case(6, 3);
        let d5 = INITIAL_POSITION.echiquier.get_case(4, 3);
        let d6 = INITIAL_POSITION.echiquier.get_case(5, 3);
        let a7 = INITIAL_POSITION.echiquier.get_case(6, 0);

        let mut white_pawn = Pawn::new(e2, true);
        let mut black_pawn_a = Pawn::new(a7, false);
        let mut black_pawn_d = Pawn::new(d7, false);

        game.play_move(&mut white_pawn, PieceMove { right: 0, up: 2 });
        game.play_move(&mut black_pawn_a, PieceMove { right: 0, up: -1 });
        game.play_move(&mut white_pawn, PieceMove { right: 0, up: 1 });
        game.play_move(&mut black_pawn_d, PieceMove { right: 0, up: -2 });
        assert_eq!(e5, white_pawn.case);
        assert_eq!(d5, black_pawn_d.case);

        let after_en_passant = *game.play_move(&mut white_pawn, PieceMove { right: -1, up: 1 });
        assert_eq!(d6, white_pawn.case);
        assert_eq!(Some(Piece::from(white_pawn)), after_en_passant.get_piece(&d6));
        assert_eq!(None, after_en_passant.get_piece(&d5));
        assert_eq!(None, after_en_passant.get_piece(&e5));
    }

    #[test]
    fn white_pieces_are_at_the_bottom_and_black_at_the_top() {
        // Ranks 0 and 1 (bottom rows: Rank 1 and Rank 2) must be White pieces
        for rank in 0..=1 {
            for file in 0..8 {
                let case = INITIAL_POSITION.echiquier.get_case(rank, file);
                let piece = INITIAL_POSITION
                    .get_piece(&case)
                    .expect("piece must exist at initial row");
                assert!(
                    piece.is_white,
                    "piece at rank {} file {} must be white",
                    rank, file
                );
            }
        }

        // Ranks 2..=5 (middle rows: Ranks 3 to 6) must be empty
        for rank in 2..=5 {
            for file in 0..8 {
                let case = INITIAL_POSITION.echiquier.get_case(rank, file);
                assert!(
                    INITIAL_POSITION.get_piece(&case).is_none(),
                    "square at rank {} file {} must be empty",
                    rank,
                    file
                );
            }
        }

        // Ranks 6 and 7 (top rows: Rank 7 and Rank 8) must be Black pieces
        for rank in 6..=7 {
            for file in 0..8 {
                let case = INITIAL_POSITION.echiquier.get_case(rank, file);
                let piece = INITIAL_POSITION
                    .get_piece(&case)
                    .expect("piece must exist at initial row");
                assert!(
                    !piece.is_white,
                    "piece at rank {} file {} must be black",
                    rank, file
                );
            }
        }
    }

    #[test]
    fn game_display_renders_full_board_with_white_at_bottom() {
        let game = Game::default_game();
        assert_eq!(
            "\
8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
6 · · · · · · · ·
5 · · · · · · · ·
4 · · · · · · · ·
3 · · · · · · · ·
2 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
  A B C D E F G H",
            game.current_position().to_string()
        );
    }
}
