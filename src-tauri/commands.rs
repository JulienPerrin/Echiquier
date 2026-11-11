//! Tauri IPC commands exposing the chess API to the desktop frontend.
//!
//! Command names and argument names mirror the `invoke` calls in `src/main.ts`
//! (camelCase on the JS side, snake_case here).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::GameInfo;

#[tauri::command]
pub fn list_games() -> Vec<GameInfo> {
    crate::list_games()
}

#[tauri::command]
pub fn game_headers(game: Option<String>) -> BTreeMap<String, String> {
    crate::game_headers(game.as_deref())
}

#[tauri::command]
pub fn game_moves(game: Option<String>) -> Vec<String> {
    crate::game_moves(game.as_deref())
}

#[tauri::command]
pub fn game_fen(move_index: usize, game: Option<String>) -> Result<String, String> {
    crate::game_fen(game.as_deref(), move_index).ok_or_else(|| {
        format!(
            "No position at move {move_index} in game {}",
            game.as_deref().unwrap_or("immortal")
        )
    })
}

#[tauri::command]
pub fn game_pgn(game: Option<String>) -> String {
    crate::game_pgn(game.as_deref())
}

pub fn pgn_file_name(game: Option<&str>) -> String {
    format!("{}.pgn", game.unwrap_or("immortal"))
}

pub fn pgn_download_path(base_dir: Option<PathBuf>, game: Option<&str>) -> PathBuf {
    let dir = base_dir.unwrap_or_else(|| {
        std::env::var_os("HOME")
            .map(|h| PathBuf::from(h).join("Downloads"))
            .unwrap_or_else(std::env::temp_dir)
    });
    dir.join(pgn_file_name(game))
}

pub fn save_pgn(path: &Path, game: Option<&str>) -> std::io::Result<()> {
    std::fs::write(path, crate::game_pgn(game))
}

#[tauri::command]
#[cfg(not(tarpaulin_include))]
pub fn download_pgn(app: tauri::AppHandle, game: Option<String>) -> Result<String, String> {
    use tauri::Manager;
    let base = app.path().download_dir().ok();
    let target = pgn_download_path(base, game.as_deref());
    save_pgn(&target, game.as_deref())
        .map(|()| target.to_string_lossy().into_owned())
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_games_command() {
        assert_eq!(crate::list_games(), list_games());
    }

    #[test]
    fn test_game_headers_command() {
        assert_eq!(crate::game_headers(None), game_headers(None));
        assert_eq!(
            crate::game_headers(Some("evergreen")),
            game_headers(Some(String::from("evergreen")))
        );
    }

    #[test]
    fn test_game_moves_command() {
        assert_eq!(crate::game_moves(None), game_moves(None));
        assert_eq!(
            crate::game_moves(Some("evergreen")),
            game_moves(Some(String::from("evergreen")))
        );
    }

    #[test]
    fn test_game_fen_command() {
        assert_eq!(
            Ok(String::from(
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
            )),
            game_fen(1, None)
        );
        assert_eq!(
            crate::game_fen(Some("evergreen"), 3),
            game_fen(3, Some(String::from("evergreen"))).ok()
        );
        assert_eq!(
            Err(String::from("No position at move 46 in game immortal")),
            game_fen(46, Some(String::from("immortal")))
        );
    }

    #[test]
    fn test_game_pgn_command() {
        assert_eq!(crate::game_pgn(None), game_pgn(None));
        assert_eq!(
            crate::game_pgn(Some("evergreen")),
            game_pgn(Some(String::from("evergreen")))
        );
    }

    #[test]
    fn test_pgn_file_name() {
        assert_eq!("immortal.pgn", pgn_file_name(None));
        assert_eq!("evergreen.pgn", pgn_file_name(Some("evergreen")));
    }

    #[test]
    fn test_save_pgn() {
        let path = std::env::temp_dir().join("echiquier_test_save_pgn.pgn");
        save_pgn(&path, Some("evergreen")).unwrap();
        assert_eq!(
            crate::game_pgn(Some("evergreen")),
            std::fs::read_to_string(&path).unwrap()
        );
        std::fs::remove_file(&path).unwrap();

        assert!(save_pgn(&std::env::temp_dir(), None).is_err());
    }

    #[test]
    fn test_pgn_download_path() {
        let custom_dir = PathBuf::from("/tmp/custom_downloads");
        let path = pgn_download_path(Some(custom_dir.clone()), Some("evergreen"));
        assert_eq!(custom_dir.join("evergreen.pgn"), path);

        let default_path = pgn_download_path(None, None);
        assert!(default_path.ends_with("immortal.pgn"));
    }
}
