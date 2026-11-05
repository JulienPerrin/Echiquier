mod echiquier;
mod pieces;

use echiquier::case::Case;
use echiquier::echiquier::Echiquier;
use echiquier::game::Game;
use echiquier::piece_move::PieceMove;
use echiquier::position::Position;
use pieces::pawn::Pawn;

fn main() {
    build_chessboard();
}

fn build_chessboard() {
    let echiquier = Echiquier::new();
    let mut initial_position = Position::new(&echiquier);
    let e2: Case = initial_position.echiquier.get_case(1, 4);
    let mut pawn = Pawn::new(e2, true);
    initial_position.set_piece(*pawn);

    let mut game = Game::new(initial_position);

    let one_up = PieceMove { right: 0, up: 1 };
    game.play_move(&mut pawn, one_up);

    println!("Before:\n{}", game.initial_position);
    println!("\nAfter:\n{}", game.current_position());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_main() {
        main();
        //assert_eq!(1, 2, "intentional test failure");
    }
}
