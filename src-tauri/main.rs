mod echiquier;
mod game;
mod immortal;
mod pieces;

use game::game::INITIAL_POSITION;

pub fn hello_world() -> String {
    INITIAL_POSITION.get_fen()
}

fn main() {
    println!("{}", hello_world());
    immortal::build_chessboard();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world_and_main() {
        assert_eq!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            hello_world()
        );
        main();
    }
}
