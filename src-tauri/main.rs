mod echiquier;
mod game;
mod games;
mod immortal;
mod pieces;

use game::game::{Game, INITIAL_POSITION};

pub fn hello_world() -> String {
    INITIAL_POSITION.get_fen()
}

pub fn game_fen(game_name: Option<&str>, move_index: usize) -> Option<String> {
    let pgn = games::get_game_or_default(game_name);
    let game = Game::from_default_text(pgn)?;
    game.positions.get(move_index).map(|pos| pos.get_fen())
}

pub fn game_moves(game_name: Option<&str>) -> Vec<String> {
    let pgn = games::get_game_or_default(game_name);
    let game = Game::from_default_text(pgn).expect("valid game");
    game.format_moves()
}

pub fn game_headers_json(game_name: Option<&str>) -> String {
    let pgn = games::get_game_or_default(game_name);
    let game = Game::from_default_text(pgn).expect("valid game");
    let entries: Vec<String> = game
        .headers
        .iter()
        .map(|(k, v)| format!("\"{}\":\"{}\"", k, v.replace('\"', "\\\"")))
        .collect();
    format!("{{{}}}", entries.join(","))
}

pub fn list_games_json() -> String {
    games::list_games_json()
}

pub fn immortal_fen(move_index: usize) -> Option<String> {
    game_fen(None, move_index)
}

pub fn immortal_moves() -> Vec<String> {
    game_moves(None)
}

pub fn game_pgn(game_name: Option<&str>) -> String {
    games::get_game_or_default(game_name).to_string()
}

pub fn immortal_pgn() -> String {
    game_pgn(None)
}

pub fn immortal_headers_json() -> String {
    game_headers_json(None)
}

pub fn handle_cli_command(cmd: &str, arg1: Option<&str>, arg2: Option<&str>) -> Option<String> {
    match cmd {
        "games" | "list-games" => Some(list_games_json()),
        "fen" => {
            let move_idx: usize = arg1?.parse().ok()?;
            game_fen(arg2, move_idx)
        }
        "moves" => Some(game_moves(arg1).join("\n")),
        "headers" => Some(game_headers_json(arg1)),
        "pgn" => Some(game_pgn(arg1)),
        _ => None,
    }
}

fn run_with_args(cmd: Option<&str>, arg1: Option<&str>, arg2: Option<&str>) {
    if let Some(c) = cmd {
        if let Some(out) = handle_cli_command(c, arg1, arg2) {
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
        args.get(2).map(String::as_str),
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

        assert_eq!(immortal_fen(1), handle_cli_command("fen", Some("1"), None));
        assert_eq!(None, handle_cli_command("fen", Some("invalid"), None));
        assert_eq!(None, handle_cli_command("fen", None, None));
        assert_eq!(
            Some(moves.join("\n")),
            handle_cli_command("moves", None, None)
        );
        let headers_json = immortal_headers_json();
        assert!(headers_json.contains("\"Event\":\"London\""));
        assert!(headers_json.contains("\"Round\":\"?\""));
        assert!(headers_json.contains("\"Title\":\"The Immortal Game\""));
        assert_eq!(
            Some(headers_json),
            handle_cli_command("headers", None, None)
        );
        assert_eq!(None, handle_cli_command("unknown", None, None));
        run_with_args(Some("moves"), None, None);
    }

    #[test]
    fn test_game_selection_api_and_cli() {
        assert_eq!(immortal_fen(0), game_fen(None, 0));
        assert_eq!(immortal_fen(0), game_fen(Some("immortal"), 0));
        assert_ne!(immortal_fen(3), game_fen(Some("evergreen"), 3));

        let evergreen_moves = game_moves(Some("evergreen"));
        assert!(!evergreen_moves.is_empty());
        assert_ne!(immortal_moves(), evergreen_moves);

        let evergreen_headers = game_headers_json(Some("evergreen"));
        assert!(evergreen_headers.contains("\"Title\":\"The Evergreen Game\""));

        let list_json = list_games_json();
        assert!(list_json.contains("\"id\":\"immortal\""));
        assert!(list_json.contains("\"id\":\"evergreen\""));

        assert_eq!(
            Some(list_json.clone()),
            handle_cli_command("games", None, None)
        );
        assert_eq!(
            Some(list_json),
            handle_cli_command("list-games", None, None)
        );
        assert_eq!(
            game_fen(Some("evergreen"), 1),
            handle_cli_command("fen", Some("1"), Some("evergreen"))
        );
        assert_eq!(
            Some(evergreen_moves.join("\n")),
            handle_cli_command("moves", Some("evergreen"), None)
        );
        assert_eq!(
            Some(evergreen_headers),
            handle_cli_command("headers", Some("evergreen"), None)
        );

        run_with_args(Some("games"), None, None);
        run_with_args(Some("fen"), Some("1"), Some("evergreen"));
        run_with_args(Some("headers"), Some("evergreen"), None);

        assert_eq!(immortal_pgn(), game_pgn(None));
        assert_eq!(immortal_pgn(), game_pgn(Some("immortal")));
        assert!(immortal_pgn().contains("The Immortal Game"));
        assert!(game_pgn(Some("evergreen")).contains("The Evergreen Game"));
        assert_eq!(Some(immortal_pgn()), handle_cli_command("pgn", None, None));
        assert_eq!(
            Some(game_pgn(Some("evergreen"))),
            handle_cli_command("pgn", Some("evergreen"), None)
        );
        run_with_args(Some("pgn"), Some("evergreen"), None);
    }
}
