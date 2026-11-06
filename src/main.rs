mod echiquier;
mod game;
mod pieces;

use echiquier::case::Case;
use echiquier::piece_move::PieceMove;
use game::game::{Game, INITIAL_POSITION};
use pieces::knight::Knight;
use pieces::pawn::Pawn;
use pieces::queen::Queen;

fn main() {
    build_chessboard();
}

fn build_chessboard() {
    let mut game = Game::default_game();

    // 1. f4 e5
    let f2: Case = INITIAL_POSITION.echiquier.get_case(1, 5);
    let mut white_pawn_f = Pawn::new(f2, true);
    game.play_move(&mut white_pawn_f, PieceMove { right: 0, up: 2 });

    let e7: Case = INITIAL_POSITION.echiquier.get_case(6, 4);
    let mut black_pawn_e = Pawn::new(e7, false);
    game.play_move(&mut black_pawn_e, PieceMove { right: 0, up: -2 });

    // 2. fxe5 d5
    game.play_move(&mut white_pawn_f, PieceMove { right: -1, up: 1 });

    let d7: Case = INITIAL_POSITION.echiquier.get_case(6, 3);
    let mut black_pawn_d = Pawn::new(d7, false);
    game.play_move(&mut black_pawn_d, PieceMove { right: 0, up: -2 });

    // 3. fxd5 (en passant: e5 captures d5 pawn, landing on d6) Cc6
    game.play_move(&mut white_pawn_f, PieceMove { right: -1, up: 1 });

    let b8: Case = INITIAL_POSITION.echiquier.get_case(7, 1);
    let mut black_knight = Knight::new(b8, false);
    game.play_move(&mut black_knight, PieceMove { right: 1, up: -2 });

    // 4. g4 Qh4#
    let g2: Case = INITIAL_POSITION.echiquier.get_case(1, 6);
    let mut white_pawn_g = Pawn::new(g2, true);
    game.play_move(&mut white_pawn_g, PieceMove { right: 0, up: 2 });

    let d8: Case = INITIAL_POSITION.echiquier.get_case(7, 3);
    let mut black_queen = Queen::new(d8, false);
    game.play_move(&mut black_queen, PieceMove { right: 4, up: -4 });

    println!("Before:\n{}", game.initial_position);
    println!("\nAfter:\n{}", game.current_position());
    if game.is_game_finished() {
        if let Some(result) = game.game_result() {
            println!("\nGame result: {}", result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_main() {
        main();
    }
}
