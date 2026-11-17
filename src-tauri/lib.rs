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

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct PositionState {
    pub fen: String,
    pub turn: String,
    pub san_moves: Vec<String>,
    pub allowed_moves: BTreeMap<String, Vec<String>>,
    pub result: Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct ImportedPgn {
    pub headers: BTreeMap<String, String>,
    pub coord_moves: Vec<String>,
    pub state: PositionState,
}

fn position_state_from_game(game: &Game<'_>) -> PositionState {
    let result = game.game_result().map(|r| match r {
        game::result::Result::WhiteWin => String::from("1-0"),
        game::result::Result::BlackWin => String::from("0-1"),
        game::result::Result::Draw => String::from("1/2-1/2"),
    });
    PositionState {
        fen: game.current_fen(),
        turn: if game.is_white_turn() {
            String::from("white")
        } else {
            String::from("black")
        },
        san_moves: game.format_moves(),
        allowed_moves: game.allowed_moves_map(),
        result,
    }
}

fn extract_coord_moves(game: &Game<'_>) -> Vec<String> {
    use crate::echiquier::case::Case;
    use crate::pieces::piece::Piece;

    let mut coords = Vec::with_capacity(game.positions.len().saturating_sub(1));
    for i in 0..game.positions.len().saturating_sub(1) {
        let prev = game.positions[i];
        let next = game.positions[i + 1];
        if let Some(moved) = next.last_moved_piece
            && let Some(m) = moved.last_move
        {
            let to = moved.case;
            let from = Case {
                rank: (to.rank as i8 - m.up) as u8,
                file: (to.file as i8 - m.right) as u8,
            };
            let mut s = format!("{}{}", from.algebraic(), to.algebraic());
            if matches!(prev.get_piece(&from), Some(Piece::Pawn(_)))
                && (to.rank == 7 || to.rank == 0)
            {
                let promo = match moved {
                    Piece::Rook(_) => Some('r'),
                    Piece::Bishop(_) => Some('b'),
                    Piece::Knight(_) => Some('n'),
                    Piece::Queen(_) => Some('q'),
                    _ => None,
                };
                if let Some(ch) = promo {
                    s.push(ch);
                }
            }
            coords.push(s);
        }
    }
    coords
}

pub fn position_state(moves: &[String]) -> Result<PositionState, String> {
    let mut game = Game::new_standard();
    for m in moves {
        game.play_coordinate_move(m)?;
    }
    Ok(position_state_from_game(&game))
}

pub fn position_state_json(moves: &[String]) -> Result<String, String> {
    let state = position_state(moves)?;
    serde_json::to_string(&state).map_err(|e| e.to_string())
}

pub fn import_pgn(pgn: &str) -> Result<ImportedPgn, String> {
    let trimmed = pgn.trim();
    if trimmed.is_empty() {
        return Err(String::from("Empty PGN"));
    }
    let game = Game::from_default_text(trimmed).ok_or_else(|| String::from("Invalid PGN"))?;
    let headers: BTreeMap<String, String> = game.headers.iter().cloned().collect();
    let coord_moves = extract_coord_moves(&game);
    let state = position_state_from_game(&game);
    Ok(ImportedPgn {
        headers,
        coord_moves,
        state,
    })
}

pub fn import_pgn_json(pgn: &str) -> Result<String, String> {
    let imported = import_pgn(pgn)?;
    serde_json::to_string(&imported).map_err(|e| e.to_string())
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
            commands::position_state,
            commands::import_pgn,
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

    #[test]
    fn test_position_state_start_and_moves() {
        let start = position_state(&[]).unwrap();
        assert_eq!(INITIAL_POSITION.get_fen(), start.fen);
        assert_eq!("white", start.turn);
        assert!(start.san_moves.is_empty());
        assert_eq!(
            Some(&vec![String::from("e3"), String::from("e4")]),
            start.allowed_moves.get("e2")
        );
        assert!(!start.allowed_moves.contains_key("e7"));

        let after_e4 = position_state(&[String::from("e2e4")]).unwrap();
        assert_eq!("black", after_e4.turn);
        assert_eq!(vec![String::from("e4")], after_e4.san_moves);
        assert_eq!(
            Some(&vec![String::from("e6"), String::from("e5")]),
            after_e4.allowed_moves.get("e7")
        );

        assert!(position_state(&[String::from("e2e5")]).is_err());
        assert!(position_state(&[String::from("bad")]).is_err());
        assert!(position_state(&[String::from("e7e5")]).is_err());
        assert!(position_state(&[String::from("e3e4")]).is_err());

        let json = position_state_json(&[String::from("e2e4")]).unwrap();
        assert!(json.contains("\"turn\":\"black\""));

        let fools_mate = position_state(&[
            String::from("f2f3"),
            String::from("e7e5"),
            String::from("g2g4"),
            String::from("d8h4"),
        ])
        .unwrap();
        assert_eq!(Some(String::from("0-1")), fools_mate.result);
        assert_eq!(None, start.result);
    }

    #[test]
    fn test_import_pgn_valid_and_invalid() {
        let pgn = "[Event \"Club\"]\n[White \"Alice\"]\n[Black \"Bob\"]\n[Result \"0-1\"]\n\n1. f3 e5 2. g4 Qh4# 0-1";
        let imported = import_pgn(pgn).unwrap();
        assert_eq!(Some(&String::from("Club")), imported.headers.get("Event"));
        assert_eq!(
            vec!["f2f3", "e7e5", "g2g4", "d8h4"],
            imported.coord_moves
        );
        assert_eq!(Some(String::from("0-1")), imported.state.result);
        assert_eq!(vec!["f3", "e5", "g4", "♕h4#"], imported.state.san_moves);

        assert!(import_pgn("").is_err());
        assert!(import_pgn("1. Zz9").is_err());
    }
}
