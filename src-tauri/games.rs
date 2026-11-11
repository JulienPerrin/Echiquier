pub const IMMORTAL_GAME: &str = include_str!("../assets/immortal.pgn");
pub const EVERGREEN_GAME: &str = include_str!("../assets/evergreen.pgn");
pub const OPERA_GAME_1858: &str = include_str!("../assets/opera_game_1858.pgn");
pub const LASKER_BAUER_1889: &str = include_str!("../assets/lasker_bauer_1889.pgn");
pub const STEINITZ_BARDELEBEN_1895: &str = include_str!("../assets/steinitz_bardeleben_1895.pgn");
pub const ROTLEWI_RUBINSTEIN_1907: &str = include_str!("../assets/rotlewi_rubinstein_1907.pgn");
pub const RETI_TARTAKOWER_1910: &str = include_str!("../assets/reti_tartakower_1910.pgn");
pub const MARSHALL_LEVITSKY_1912: &str = include_str!("../assets/marshall_levitsky_1912.pgn");
pub const FISCHER_PETROSIAN_1971_GAME1: &str =
    include_str!("../assets/fischer_petrosian_1971_game1.pgn");
pub const BOTVINNIK_CAPABLANCA_1938: &str = include_str!("../assets/botvinnik_capablanca_1938.pgn");
pub const GAME_OF_THE_CENTURY_1956: &str = include_str!("../assets/game_of_the_century_1956.pgn");
pub const TAL_SMYSLOV_1959: &str = include_str!("../assets/tal_smyslov_1959.pgn");
pub const BYRNE_FISCHER_1963: &str = include_str!("../assets/byrne_fischer_1963.pgn");
pub const SPASSKY_PETROSIAN_1969_GAME19: &str =
    include_str!("../assets/spassky_petrosian_1969_game19.pgn");
pub const FISCHER_LARSEN_1971_GAME1: &str = include_str!("../assets/fischer_larsen_1971_game1.pgn");
pub const FISCHER_SPASSKY_1972_GAME6: &str =
    include_str!("../assets/fischer_spassky_1972_game6.pgn");
pub const KARPOV_KASPAROV_1985_GAME16: &str =
    include_str!("../assets/karpov_kasparov_1985_game16.pgn");
pub const SHORT_TIMMAN_1991: &str = include_str!("../assets/short_timman_1991.pgn");
pub const DEEP_BLUE_KASPAROV_1997_GAME6: &str =
    include_str!("../assets/deep_blue_kasparov_1997_game6.pgn");
pub const KASPAROV_TOPALOV_1999: &str = include_str!("../assets/kasparov_topalov_1999.pgn");

pub const ALL_GAMES: &[(&str, &str)] = &[
    ("immortal", IMMORTAL_GAME),
    ("evergreen", EVERGREEN_GAME),
    ("opera_game_1858", OPERA_GAME_1858),
    ("lasker_bauer_1889", LASKER_BAUER_1889),
    ("steinitz_bardeleben_1895", STEINITZ_BARDELEBEN_1895),
    ("rotlewi_rubinstein_1907", ROTLEWI_RUBINSTEIN_1907),
    ("reti_tartakower_1910", RETI_TARTAKOWER_1910),
    ("marshall_levitsky_1912", MARSHALL_LEVITSKY_1912),
    ("fischer_petrosian_1971_game1", FISCHER_PETROSIAN_1971_GAME1),
    ("botvinnik_capablanca_1938", BOTVINNIK_CAPABLANCA_1938),
    ("game_of_the_century_1956", GAME_OF_THE_CENTURY_1956),
    ("tal_smyslov_1959", TAL_SMYSLOV_1959),
    ("byrne_fischer_1963", BYRNE_FISCHER_1963),
    (
        "spassky_petrosian_1969_game19",
        SPASSKY_PETROSIAN_1969_GAME19,
    ),
    ("fischer_larsen_1971_game1", FISCHER_LARSEN_1971_GAME1),
    ("fischer_spassky_1972_game6", FISCHER_SPASSKY_1972_GAME6),
    ("karpov_kasparov_1985_game16", KARPOV_KASPAROV_1985_GAME16),
    ("short_timman_1991", SHORT_TIMMAN_1991),
    (
        "deep_blue_kasparov_1997_game6",
        DEEP_BLUE_KASPAROV_1997_GAME6,
    ),
    ("kasparov_topalov_1999", KASPAROV_TOPALOV_1999),
];

pub fn get_game(name: &str) -> Option<&'static str> {
    ALL_GAMES
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
        .map(|(_, g)| *g)
}

pub fn get_game_or_default(name: Option<&str>) -> &'static str {
    name.and_then(get_game).unwrap_or(IMMORTAL_GAME)
}

pub use crate::game::game::compute_title;

/// Identifier and display title of a built-in game, serialized for the frontend.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct GameInfo {
    pub id: String,
    pub title: String,
}

pub fn list_games() -> Vec<GameInfo> {
    ALL_GAMES
        .iter()
        .map(|(id, pgn)| GameInfo {
            id: (*id).to_string(),
            title: compute_title(pgn),
        })
        .collect()
}

pub fn list_games_json() -> String {
    serde_json::to_string(&list_games()).expect("game list is serializable")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::game::Game;

    #[test]
    fn test_all_20_games_parse_and_play() {
        assert_eq!(20, ALL_GAMES.len());
        for (name, pgn) in ALL_GAMES {
            let game = Game::from_default_text(pgn)
                .unwrap_or_else(|| panic!("game {name} failed to parse and play"));
            assert!(!game.positions.is_empty(), "game {name} has no positions");
        }
    }

    #[test]
    fn test_immortal_game_loads_and_plays() {
        let game =
            Game::from_default_text(IMMORTAL_GAME).expect("Immortal game should parse and play");
        assert_eq!(Some("The Immortal Game"), game.title.as_deref());
        assert_eq!(Some("Adolf Anderssen"), game.white.as_deref());
        assert_eq!(Some("Lionel Kieseritzky"), game.black.as_deref());
        assert_eq!(45, game.positions.len() - 1);
    }

    #[test]
    fn test_fischer_spassky_1972_game6_loads_and_has_no_title() {
        let game = Game::from_default_text(FISCHER_SPASSKY_1972_GAME6)
            .expect("Fischer vs Spassky should parse and play");
        assert_eq!(None, game.title);
        assert!(!game.is_header_defined("Title"));
        assert_eq!(Some("Bobby Fischer"), game.white.as_deref());
        assert_eq!(Some("Boris Spassky"), game.black.as_deref());
        assert_eq!(Some("Reykjavik"), game.site.as_deref());
        assert_eq!(Some("1972.07.23"), game.date.as_deref());
        assert_eq!(81, game.positions.len() - 1);
    }

    #[test]
    fn test_all_games_and_lookup() {
        assert_eq!(20, ALL_GAMES.len());
        assert_eq!(Some(IMMORTAL_GAME), get_game("immortal"));
        assert_eq!(Some(EVERGREEN_GAME), get_game("evergreen"));
        assert_eq!(Some(OPERA_GAME_1858), get_game("opera_game_1858"));
        assert_eq!(
            Some(KASPAROV_TOPALOV_1999),
            get_game("kasparov_topalov_1999")
        );
        assert_eq!(
            Some(FISCHER_SPASSKY_1972_GAME6),
            get_game("fischer_spassky_1972_game6")
        );
        assert_eq!(None, get_game("unknown"));
    }

    #[test]
    fn test_get_game_or_default() {
        assert_eq!(IMMORTAL_GAME, get_game_or_default(None));
        assert_eq!(IMMORTAL_GAME, get_game_or_default(Some("immortal")));
        assert_eq!(EVERGREEN_GAME, get_game_or_default(Some("evergreen")));
        assert_eq!(
            IMMORTAL_GAME,
            get_game_or_default(Some("non_existent_game"))
        );
    }

    #[test]
    fn test_compute_title_and_list_games_json() {
        assert_eq!("The Immortal Game", compute_title(IMMORTAL_GAME));
        assert_eq!(
            "Bobby Fischer - Boris Spassky, Reykjavik, 1972",
            compute_title(FISCHER_SPASSKY_1972_GAME6)
        );
        assert_eq!("The Immortal Game", compute_title("invalid pgn"));
        assert_eq!("The Immortal Game", compute_title("1. e4 e5"));

        let list_json = list_games_json();
        assert!(list_json.starts_with('['));
        assert!(list_json.ends_with(']'));
        assert!(list_json.contains("{\"id\":\"immortal\",\"title\":\"The Immortal Game\"}"));
        assert!(list_json.contains("{\"id\":\"evergreen\",\"title\":\"The Evergreen Game\"}"));
        assert!(list_json.contains("\"id\":\"fischer_spassky_1972_game6\""));
    }
}
