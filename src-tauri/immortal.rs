use crate::game::game::Game;

pub use crate::games::IMMORTAL_GAME;

#[cfg(test)]
fn main() {
    build_chessboard();
}

pub fn build_chessboard() {
    let game =
        Game::from_default_text(IMMORTAL_GAME).expect("Failed to parse and play the Immortal Game");

    if let (Some(w), Some(b)) = (game.white.as_deref(), game.black.as_deref()) {
        println!("{w} vs {b}\n");
    }

    let moves = game.format_moves();
    println!("Initial position:\n{}\n", game.initial_position);

    for (i, position) in game.positions.iter().enumerate().skip(1) {
        let move_str = &moves[i - 1];
        let turn = (i - 1) / 2 + 1;
        let label = if (i - 1) % 2 == 0 {
            format!("{turn}. {move_str}")
        } else {
            format!("{turn}... {move_str}")
        };
        println!("Position {i} ({label}):\n{position}\n");
    }

    println!("Game:\n{game}");
    if game.is_game_finished() {
        if let Some(result) = game.game_result() {
            println!("\nGame result: {result}");
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

    #[test]
    fn test_immortal_game_asset() {
        assert!(IMMORTAL_GAME.contains("[Title \"The Immortal Game\"]"));
        assert!(IMMORTAL_GAME.contains("23. Fe7#"));
    }
}
