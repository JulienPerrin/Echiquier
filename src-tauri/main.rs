mod echiquier;
mod game;
mod immortal;
mod pieces;

use game::game::{Game, INITIAL_POSITION};

pub fn hello_world() -> String {
    INITIAL_POSITION.get_fen()
}

pub fn immortal_fen(move_index: usize) -> Option<String> {
    let game = Game::from_default_text(immortal::IMMORTAL_GAME)?;
    game.positions.get(move_index).map(|pos| pos.get_fen())
}

pub fn immortal_moves() -> Vec<String> {
    let game = Game::from_default_text(immortal::IMMORTAL_GAME).expect("valid immortal game");
    game.format_moves()
}

pub fn immortal_headers_json() -> String {
    let game = Game::from_default_text(immortal::IMMORTAL_GAME).expect("valid immortal game");
    let entries: Vec<String> = game
        .headers
        .iter()
        .map(|(k, v)| format!("\"{}\":\"{}\"", k, v))
        .collect();
    format!("{{{}}}", entries.join(","))
}

pub fn handle_cli_command(cmd: &str, arg: Option<&str>) -> Option<String> {
    match cmd {
        "fen" => immortal_fen(arg?.parse().ok()?),
        "moves" => Some(immortal_moves().join("\n")),
        "headers" => Some(immortal_headers_json()),
        _ => None,
    }
}

fn run_with_args(cmd: Option<&str>, arg: Option<&str>) {
    if let Some(c) = cmd {
        if let Some(out) = handle_cli_command(c, arg) {
            println!("{out}");
        }
    } else {
        println!("{}", hello_world());
        immortal::build_chessboard();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    run_with_args(
        args.first().map(String::as_str),
        args.get(1).map(String::as_str),
    );
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

    #[test]
    fn test_immortal_fen_and_moves_api() {
        assert_eq!(
            Some(String::from(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
            )),
            immortal_fen(0)
        );
        assert_eq!(
            Some(String::from(
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
            )),
            immortal_fen(1)
        );
        assert_eq!(
            Some(String::from(
                "r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 b - - 1 23"
            )),
            immortal_fen(45)
        );
        assert_eq!(None, immortal_fen(46));

        let moves = immortal_moves();
        assert_eq!(45, moves.len());
        assert_eq!("e4", moves[0]);
        assert_eq!("e5", moves[1]);
        assert_eq!("♝e7#", moves[44]);

        assert_eq!(immortal_fen(1), handle_cli_command("fen", Some("1")));
        assert_eq!(None, handle_cli_command("fen", Some("invalid")));
        assert_eq!(None, handle_cli_command("fen", None));
        assert_eq!(Some(moves.join("\n")), handle_cli_command("moves", None));
        let headers_json = immortal_headers_json();
        assert!(headers_json.contains("\"Event\":\"London\""));
        assert!(headers_json.contains("\"Round\":\"?\""));
        assert!(headers_json.contains("\"Title\":\"The Immortal Game\""));
        assert_eq!(Some(headers_json), handle_cli_command("headers", None));
        assert_eq!(None, handle_cli_command("unknown", None));
        run_with_args(Some("moves"), None);
    }
}
