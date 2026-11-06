mod echiquier;
mod game;
mod pieces;

use echiquier::case::Case;
use echiquier::piece_move::PieceMove;
use game::game::{Game, INITIAL_POSITION};
use pieces::pawn::Pawn;

fn main() {
    build_chessboard();
}

fn build_chessboard() {
    let mut game = Game::default_game();
    let e2: Case = INITIAL_POSITION.echiquier.get_case(1, 4);
    let mut white_pawn = Pawn::new(e2, true);
    game.play_move(&mut white_pawn, PieceMove { right: 0, up: 2 });

    let d7: Case = INITIAL_POSITION.echiquier.get_case(6, 3);
    let mut black_pawn = Pawn::new(d7, false);
    game.play_move(&mut black_pawn, PieceMove { right: 0, up: -2 });

    game.play_move(&mut white_pawn, PieceMove { right: -1, up: 1 });

    println!("Before:\n{}", game.initial_position);
    println!("\nAfter:\n{}", game.current_position());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_main() {
        main();
    }
}
