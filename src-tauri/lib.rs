mod commands;
mod echiquier;
mod game;
mod games;
pub mod immortal;
mod pieces;

use std::collections::BTreeMap;

use game::game::{Game, INITIAL_POSITION};
pub use games::GameInfo;

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

pub fn game_headers(game_name: Option<&str>) -> BTreeMap<String, String> {
    let pgn = games::get_game_or_default(game_name);
    let game = Game::from_default_text(pgn).expect("valid game");
    game.headers.into_iter().collect()
}

pub fn game_headers_json(game_name: Option<&str>) -> String {
    serde_json::to_string(&game_headers(game_name)).expect("headers are serializable")
}

pub fn list_games() -> Vec<GameInfo> {
    games::list_games()
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

/// Launches the desktop application (Tauri window serving the web frontend).
#[cfg(not(tarpaulin_include))]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            commands::list_games,
            commands::game_headers,
            commands::game_moves,
            commands::game_fen,
            commands::game_pgn,
            commands::download_pgn,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_games_is_typed() {
        let list = list_games();
        assert_eq!(20, list.len());
        assert_eq!("immortal", list[0].id);
        assert_eq!("The Immortal Game", list[0].title);
        assert_eq!(list_games_json(), serde_json::to_string(&list).unwrap());
    }

    #[test]
    fn test_game_headers_is_typed() {
        let headers = game_headers(None);
        assert_eq!(Some("London"), headers.get("Event").map(String::as_str));
        assert_eq!(Some("?"), headers.get("Round").map(String::as_str));
        assert_eq!(
            Some("The Evergreen Game"),
            game_headers(Some("evergreen"))
                .get("Title")
                .map(String::as_str)
        );
        let parsed: std::collections::BTreeMap<String, String> =
            serde_json::from_str(&game_headers_json(None)).unwrap();
        assert_eq!(headers, parsed);
    }
}
