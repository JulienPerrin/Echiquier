use crate::game::game::Game;

pub const IMMORTAL_GAME: &str = "\
[Event \"London\"]
[Site \"London\"]
[Date \"1851.06.21\"]
[Round \"?\"]
[White \"Adolf Anderssen\"]
[Black \"Lionel Kieseritzky\"]
[Result \"1-0\"]
[Title \"The Immortal Game\"]

1.  e4 e5
2.  f4 exf4
3.  Fc4 Dh4+
4.  Rf1 b5
5.  Fxb5 Cf6
6.  Cf3 Dh6
7.  d3 Ch5
8.  Ch4 Dg5
9.  Cf5 c6
10. g4 Cf6
11. Tg1 cxb5
12. h4 Dg6
13. h5 Dg5
14. Df3 Cg8
15. Fxf4 Df6
16. Cc3 Fc5
17. Cd5 Dxb2
18. Fd6 Fxg1
19. e5 Dxa1+
20. Re2 Ca6
21. Cxg7+ Rd8
22. Df6+ Cxf6
23. Fe7#";

#[allow(dead_code)]
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
}
