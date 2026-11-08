use std::fmt::{self, Display};

use crate::echiquier::case::Case;
use crate::echiquier::echiquier::Echiquier;
#[cfg(test)]
use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::game::position_change::PositionChange;
use crate::game::result::Result;
use crate::pieces::bishop::Bishop;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
#[cfg(test)]
use crate::pieces::movable::Movable;
use crate::pieces::pawn::Pawn;
#[cfg(test)]
use crate::pieces::pawn::PromotionPiece;
use crate::pieces::piece::Piece;
use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;

pub const ECHIQUIER: Echiquier = Echiquier::new();

pub const INITIAL_POSITION: Position<'static> = Position {
    echiquier: &ECHIQUIER,
    pieces: [
        Some(Piece::Rook(Rook::new(Case { rank: 0, file: 0 }, true))),
        Some(Piece::Knight(Knight::new(Case { rank: 0, file: 1 }, true))),
        Some(Piece::Bishop(Bishop::new(Case { rank: 0, file: 2 }, true))),
        Some(Piece::Queen(Queen::new(Case { rank: 0, file: 3 }, true))),
        Some(Piece::King(King::new(Case { rank: 0, file: 4 }, true))),
        Some(Piece::Bishop(Bishop::new(Case { rank: 0, file: 5 }, true))),
        Some(Piece::Knight(Knight::new(Case { rank: 0, file: 6 }, true))),
        Some(Piece::Rook(Rook::new(Case { rank: 0, file: 7 }, true))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 1, file: 0 }, true))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 1, file: 1 }, true))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 1, file: 2 }, true))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 1, file: 3 }, true))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 1, file: 4 }, true))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 1, file: 5 }, true))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 1, file: 6 }, true))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 1, file: 7 }, true))),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(Piece::Pawn(Pawn::new(Case { rank: 6, file: 0 }, false))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 6, file: 1 }, false))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 6, file: 2 }, false))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 6, file: 3 }, false))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 6, file: 4 }, false))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 6, file: 5 }, false))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 6, file: 6 }, false))),
        Some(Piece::Pawn(Pawn::new(Case { rank: 6, file: 7 }, false))),
        Some(Piece::Rook(Rook::new(Case { rank: 7, file: 0 }, false))),
        Some(Piece::Knight(Knight::new(Case { rank: 7, file: 1 }, false))),
        Some(Piece::Bishop(Bishop::new(Case { rank: 7, file: 2 }, false))),
        Some(Piece::Queen(Queen::new(Case { rank: 7, file: 3 }, false))),
        Some(Piece::King(King::new(Case { rank: 7, file: 4 }, false))),
        Some(Piece::Bishop(Bishop::new(Case { rank: 7, file: 5 }, false))),
        Some(Piece::Knight(Knight::new(Case { rank: 7, file: 6 }, false))),
        Some(Piece::Rook(Rook::new(Case { rank: 7, file: 7 }, false))),
    ],
    last_moved_piece: None,
    halfmove_clock: 0,
    fullmove_number: 1,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Game<'a> {
    pub initial_position: Position<'a>,
    pub positions: Vec<Position<'a>>,
    pub title: Option<String>,
    pub event: Option<String>,
    pub site: Option<String>,
    pub date: Option<String>,
    pub round: Option<String>,
    pub white: Option<String>,
    pub black: Option<String>,
    pub headers: Vec<(String, String)>,
    pub result: Option<Result>,
}

struct ParsedHeaders {
    headers: Vec<(String, String)>,
    title: Option<String>,
    event: Option<String>,
    site: Option<String>,
    date: Option<String>,
    round: Option<String>,
    white: Option<String>,
    black: Option<String>,
    result: Option<Result>,
    moves_text: String,
}

fn parse_pgn_headers(text: &str) -> ParsedHeaders {
    let mut headers = Vec::new();
    let mut title = None;
    let mut event = None;
    let mut site = None;
    let mut date = None;
    let mut round = None;
    let mut white = None;
    let mut black = None;
    let mut result = None;
    let mut moves_lines = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let inner = trimmed[1..trimmed.len() - 1].trim();
            if let Some(space_pos) = inner.find(char::is_whitespace) {
                let key = inner[..space_pos].trim();
                let val = inner[space_pos..].trim().trim_matches('"');
                if key.eq_ignore_ascii_case("title") {
                    title = Some(val.to_string());
                } else if key.eq_ignore_ascii_case("event") {
                    event = Some(val.to_string());
                } else if key.eq_ignore_ascii_case("site") {
                    site = Some(val.to_string());
                } else if key.eq_ignore_ascii_case("date") {
                    date = Some(val.to_string());
                } else if key.eq_ignore_ascii_case("round") {
                    round = Some(val.to_string());
                } else if key.eq_ignore_ascii_case("white") {
                    white = Some(val.to_string());
                } else if key.eq_ignore_ascii_case("black") {
                    black = Some(val.to_string());
                } else if key.eq_ignore_ascii_case("result") {
                    if val == "1-0" {
                        result = Some(Result::WhiteWin);
                    } else if val == "0-1" {
                        result = Some(Result::BlackWin);
                    } else if val == "1/2-1/2" || val == "½-½" || val == "0.5-0.5" {
                        result = Some(Result::Draw);
                    }
                }
                headers.push((key.to_string(), val.to_string()));
            }
        } else {
            moves_lines.push(line);
        }
    }

    ParsedHeaders {
        headers,
        title,
        event,
        site,
        date,
        round,
        white,
        black,
        result,
        moves_text: moves_lines.join("\n"),
    }
}

fn extract_trailing_result(text: &str) -> (String, Option<Result>, bool) {
    let mut trimmed = text.trim();
    let mut trailing_result = None;
    let mut is_resignation = false;

    let lower = trimmed.to_lowercase();
    if lower.ends_with("white resigns") {
        trailing_result = Some(Result::BlackWin);
        trimmed = trimmed[..trimmed.len() - "white resigns".len()].trim_end();
    } else if lower.ends_with("blancs abandonnent") {
        trailing_result = Some(Result::BlackWin);
        trimmed = trimmed[..trimmed.len() - "blancs abandonnent".len()].trim_end();
    } else if lower.ends_with("black resigns") {
        trailing_result = Some(Result::WhiteWin);
        trimmed = trimmed[..trimmed.len() - "black resigns".len()].trim_end();
    } else if lower.ends_with("noirs abandonnent") {
        trailing_result = Some(Result::WhiteWin);
        trimmed = trimmed[..trimmed.len() - "noirs abandonnent".len()].trim_end();
    } else if lower.ends_with("resigns") {
        is_resignation = true;
        trimmed = trimmed[..trimmed.len() - "resigns".len()].trim_end();
    } else if lower.ends_with("abandonne") {
        is_resignation = true;
        trimmed = trimmed[..trimmed.len() - "abandonne".len()].trim_end();
    } else if lower.ends_with("abandon") {
        is_resignation = true;
        trimmed = trimmed[..trimmed.len() - "abandon".len()].trim_end();
    } else if trimmed.ends_with("1-0") {
        trailing_result = Some(Result::WhiteWin);
        trimmed = trimmed[..trimmed.len() - "1-0".len()].trim_end();
    } else if trimmed.ends_with("0-1") {
        trailing_result = Some(Result::BlackWin);
        trimmed = trimmed[..trimmed.len() - "0-1".len()].trim_end();
    } else if trimmed.ends_with("1/2-1/2") {
        trailing_result = Some(Result::Draw);
        trimmed = trimmed[..trimmed.len() - "1/2-1/2".len()].trim_end();
    } else if trimmed.ends_with("½-½") {
        trailing_result = Some(Result::Draw);
        trimmed = trimmed[..trimmed.len() - "½-½".len()].trim_end();
    } else if trimmed.ends_with("0.5-0.5") {
        trailing_result = Some(Result::Draw);
        trimmed = trimmed[..trimmed.len() - "0.5-0.5".len()].trim_end();
    } else if trimmed.ends_with('=') {
        trailing_result = Some(Result::Draw);
        trimmed = trimmed[..trimmed.len() - '='.len_utf8()].trim_end();
    }

    (trimmed.to_string(), trailing_result, is_resignation)
}

fn clean_moves_text(text: &str) -> String {
    let sanitized = text
        .replace("1-0", " ")
        .replace("0-1", " ")
        .replace("1/2-1/2", " ")
        .replace("mat", " ")
        .replace("Mat", " ")
        .replace('*', " ");
    let mut result = String::with_capacity(sanitized.len());
    let chars: Vec<char> = sanitized.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_ascii_digit() {
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            if i < chars.len() && chars[i] == '.' {
                while i < chars.len() && chars[i] == '.' {
                    i += 1;
                }
                result.push(' ');
            } else {
                for c in &chars[start..i] {
                    result.push(*c);
                }
            }
        } else if chars[i] == '.' {
            while i < chars.len() && chars[i] == '.' {
                i += 1;
            }
            result.push(' ');
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

impl Game<'static> {
    #[cfg(test)]
    fn default_game() -> Game<'static> {
        Game::new(INITIAL_POSITION)
    }

    pub fn from_default_text(text: &str) -> Option<Game<'static>> {
        Game::from_text(INITIAL_POSITION, text)
    }
}

impl<'a> Game<'a> {
    fn new(initial_position: Position<'a>) -> Game<'a> {
        let mut game = Game {
            initial_position,
            positions: vec![initial_position],
            title: None,
            event: None,
            site: None,
            date: None,
            round: None,
            white: None,
            black: None,
            headers: Vec::new(),
            result: None,
        };
        game.result = game.board_result();
        game
    }

    #[cfg(test)]
    pub fn header(&self, key: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v.as_str())
    }

    #[cfg(test)]
    pub fn is_header_defined(&self, key: &str) -> bool {
        self.header(key).is_some_and(|v| {
            let trimmed = v.trim();
            !trimmed.is_empty()
                && trimmed != "*"
                && !trimmed
                    .chars()
                    .all(|c| matches!(c, '.' | '?' | '/' | '-' | '—' | '–' | ' ' | '\t'))
        })
    }

    pub fn compute_title(&self) -> String {
        if let Some(title) = &self.title {
            let t = title.trim();
            if !t.is_empty() && t != "*" {
                return t.to_string();
            }
        }
        let white = self.white.as_deref().unwrap_or("").trim();
        let black = self.black.as_deref().unwrap_or("").trim();
        let site_opt = self.site.as_deref().or(self.event.as_deref());
        let site = site_opt.unwrap_or("").trim();
        let date = self.date.as_deref().unwrap_or("").trim();

        let mut parts = Vec::new();
        if !white.is_empty() && !black.is_empty() {
            parts.push(format!("{white} - {black}"));
        }
        if !site.is_empty() && site != "?" {
            parts.push(site.to_string());
        }
        if !date.is_empty() {
            let year = date.split('.').next().unwrap_or("");
            if !year.is_empty() && year != "?" {
                parts.push(year.to_string());
            }
        }
        if parts.is_empty() {
            String::from("The Immortal Game")
        } else {
            parts.join(", ")
        }
    }

    fn from_text(initial_position: Position<'a>, text: &str) -> Option<Self> {
        let parsed = parse_pgn_headers(text);
        let mut game = Game::new(initial_position);
        game.headers = parsed.headers;
        game.title = parsed.title;
        game.event = parsed.event;
        game.site = parsed.site;
        game.date = parsed.date;
        game.round = parsed.round;
        game.white = parsed.white;
        game.black = parsed.black;
        game.result = parsed.result;
        game.play_text_moves(&parsed.moves_text)?;
        Some(game)
    }

    fn play_text_moves(&mut self, text: &str) -> Option<&Position<'a>> {
        let parsed = parse_pgn_headers(text);
        if self.title.is_none() {
            self.title = parsed.title;
        }
        if self.event.is_none() {
            self.event = parsed.event;
        }
        if self.site.is_none() {
            self.site = parsed.site;
        }
        if self.date.is_none() {
            self.date = parsed.date;
        }
        if self.round.is_none() {
            self.round = parsed.round;
        }
        if self.white.is_none() {
            self.white = parsed.white;
        }
        if self.black.is_none() {
            self.black = parsed.black;
        }
        if self.result.is_none() {
            self.result = parsed.result;
        }
        for (k, v) in parsed.headers {
            if !self.headers.iter().any(|(existing_k, _)| existing_k == &k) {
                self.headers.push((k, v));
            }
        }

        let (moves, trailing_result, is_resignation) = extract_trailing_result(&parsed.moves_text);

        let initial_len = self.positions.len();
        let cleaned = clean_moves_text(&moves);
        // TODO: Improve resilience for difficult cases with spaces inside moves like say "K x K   e4 x K"
        // For now, whitespace separates individual moves.
        for token in cleaned.split_whitespace() {
            if let Some(change) = PositionChange::from_text(self.current_position(), token) {
                self.positions.push(change.next);
            } else {
                self.positions.truncate(initial_len);
                return None;
            }
        }

        if is_resignation {
            if self.current_position().is_white_turn() {
                self.result = Some(Result::BlackWin);
            } else {
                self.result = Some(Result::WhiteWin);
            }
        } else if trailing_result.is_some() {
            self.result = trailing_result;
        } else if self.result.is_none() {
            self.result = self.board_result();
        }

        Some(self.current_position())
    }

    pub fn format_moves(&self) -> Vec<String> {
        (0..self.positions.len().saturating_sub(1))
            .map(|i| PositionChange::new(self.positions[i], self.positions[i + 1]).format())
            .collect()
    }

    #[cfg(test)]
    fn format_pgn(&self) -> String {
        let mut result = String::new();
        for (k, v) in &self.headers {
            result.push_str(&format!("[{k} \"{v}\"]\n"));
        }
        if !self.headers.is_empty() {
            result.push('\n');
        }
        let moves = self.format_moves();
        for (i, m) in moves.iter().enumerate() {
            if i % 2 == 0 {
                if i > 0 {
                    result.push(' ');
                }
                result.push_str(&format!("{}. ", i / 2 + 1));
            } else {
                result.push(' ');
            }
            result.push_str(m);
        }
        if let Some(res) = self.game_result() {
            let res_str = match res {
                Result::WhiteWin => "1-0",
                Result::BlackWin => "0-1",
                Result::Draw => "1/2-1/2",
            };
            if moves.is_empty() {
                result.push_str(res_str);
            } else {
                result.push(' ');
                result.push_str(res_str);
            }
        }
        result
    }

    fn current_position(&self) -> &Position<'a> {
        self.positions.last().unwrap_or(&self.initial_position)
    }

    #[cfg(test)]
    fn play_move<T>(&mut self, piece: &mut T, piece_move: PieceMove) -> &Position<'a>
    where
        T: Movable + Copy,
        Piece: From<T>,
    {
        self.play_move_with_promotion(piece, piece_move, None)
    }

    #[cfg(test)]
    fn play_move_with_promotion<T>(
        &mut self,
        piece: &mut T,
        piece_move: PieceMove,
        promotion: Option<PromotionPiece>,
    ) -> &Position<'a>
    where
        T: Movable + Copy,
        Piece: From<T>,
    {
        let current = *self.current_position();
        let echiquier = current.echiquier;
        let target_case = echiquier
            .get_relative_case(&piece.case, piece_move)
            .expect("target case out of bounds!");
        let piece_taken = current.get_piece(&target_case).or_else(|| {
            piece
                .allowed_captures(&current)
                .into_iter()
                .find(|&(m, _)| m == piece_move)
                .map(|(_, taken)| taken)
        });
        let next_pos = piece.move_piece(&current, piece_move, target_case, piece_taken, promotion);
        self.positions.push(next_pos);
        if self.result.is_none() {
            self.result = self.board_result();
        }
        self.current_position()
    }

    pub fn is_game_finished(&self) -> bool {
        self.game_result().is_some()
    }

    fn board_result(&self) -> Option<Result> {
        let current = self.current_position();
        let is_white_turn = current.is_white_turn();
        let king = current
            .get_pieces::<King>(is_white_turn)
            .into_iter()
            .next()?;
        if king.is_in_checkmate(current) {
            if is_white_turn {
                Some(Result::BlackWin)
            } else {
                Some(Result::WhiteWin)
            }
        } else if king.is_in_stalemate(current) {
            Some(Result::Draw)
        } else {
            None
        }
    }

    pub fn game_result(&self) -> Option<Result> {
        self.result.or_else(|| self.board_result())
    }
}

impl<'a> Display for Game<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let moves = self.format_moves();
        for (i, chunk) in moves.chunks(2).enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            let turn = i + 1;
            if chunk.len() == 2 {
                write!(f, "{turn}. {} {}", chunk[0], chunk[1])?;
            } else {
                write!(f, "{turn}. {}", chunk[0])?;
            }
        }
        if let Some(res) = self.game_result() {
            let res_str = match res {
                Result::WhiteWin => "1-0",
                Result::BlackWin => "0-1",
                Result::Draw => "1/2-1/2",
            };
            if moves.is_empty() {
                write!(f, "{res_str}")?;
            } else {
                write!(f, " {res_str}")?;
            }
        }
        Ok(())
    }
}

pub fn compute_title(pgn: &str) -> String {
    match Game::from_default_text(pgn) {
        Some(g) => g.compute_title(),
        None => String::from("The Immortal Game"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_tracks_initial_and_played_positions() {
        let mut game = Game::default_game();
        assert_eq!(1, game.positions.len());
        assert_eq!(INITIAL_POSITION, *game.current_position());

        let e2 = INITIAL_POSITION.echiquier.get_case(1, 4);
        let mut white_pawn = Pawn::new(e2, true);
        let two_up = PieceMove { right: 0, up: 2 };
        let next_position = *game.play_move(&mut white_pawn, two_up);

        assert_eq!(2, game.positions.len());
        assert_eq!(INITIAL_POSITION, game.initial_position);
        assert_eq!(INITIAL_POSITION, game.positions[0]);
        assert_eq!(next_position, game.positions[1]);
        assert_eq!(next_position, *game.current_position());

        let d7 = INITIAL_POSITION.echiquier.get_case(6, 3);
        let mut black_pawn = Pawn::new(d7, false);
        let two_down = PieceMove { right: 0, up: -2 };
        game.play_move(&mut black_pawn, two_down);

        let after_capture = *game.play_move(&mut white_pawn, PieceMove { right: -1, up: 1 });
        assert_eq!(4, game.positions.len());
        assert_eq!(after_capture, *game.current_position());
        let d5 = INITIAL_POSITION.echiquier.get_case(4, 3);
        assert_eq!(Some(Piece::from(white_pawn)), after_capture.get_piece(&d5));
    }

    #[test]
    #[should_panic(expected = "target case out of bounds!")]
    fn play_move_panics_on_out_of_bounds() {
        let mut game = Game::default_game();
        let e2 = INITIAL_POSITION.echiquier.get_case(1, 4);
        let mut white_pawn = Pawn::new(e2, true);
        game.play_move(&mut white_pawn, PieceMove { right: 0, up: 10 });
    }

    #[test]
    fn game_display_prints_game_rounds() {
        let game = Game::default_game();
        assert_eq!("", game.to_string());

        let mut game1 = Game::default_game();
        game1.play_text_moves("1. e4");
        assert_eq!("1. e4", game1.to_string());

        let mut game2 = Game::default_game();
        game2.play_text_moves("1. e4 e5\n2. Nf3");
        assert_eq!("1. e4 e5\n2. ♞f3", game2.to_string());

        let parsed_game =
            Game::from_default_text("1. f4 e5 2. fxe5 d5 3. exd6 Cc6 4. g4 Qh4#").unwrap();
        assert_eq!(
            "1. f4 e5\n2. fxe5 d5\n3. exd6 ♘c6\n4. g4 ♕h4# 0-1",
            parsed_game.to_string()
        );
    }

    #[test]
    fn game_play_move_handles_en_passant_capture() {
        let mut game = Game::default_game();
        let e2 = INITIAL_POSITION.echiquier.get_case(1, 4);
        let e5 = INITIAL_POSITION.echiquier.get_case(4, 4);
        let d7 = INITIAL_POSITION.echiquier.get_case(6, 3);
        let d5 = INITIAL_POSITION.echiquier.get_case(4, 3);
        let d6 = INITIAL_POSITION.echiquier.get_case(5, 3);
        let a7 = INITIAL_POSITION.echiquier.get_case(6, 0);

        let mut white_pawn = Pawn::new(e2, true);
        let mut black_pawn_a = Pawn::new(a7, false);
        let mut black_pawn_d = Pawn::new(d7, false);

        game.play_move(&mut white_pawn, PieceMove { right: 0, up: 2 });
        game.play_move(&mut black_pawn_a, PieceMove { right: 0, up: -1 });
        game.play_move(&mut white_pawn, PieceMove { right: 0, up: 1 });
        game.play_move(&mut black_pawn_d, PieceMove { right: 0, up: -2 });
        assert_eq!(e5, white_pawn.case);
        assert_eq!(d5, black_pawn_d.case);

        let after_en_passant = *game.play_move(&mut white_pawn, PieceMove { right: -1, up: 1 });
        assert_eq!(d6, white_pawn.case);
        assert_eq!(
            "\
8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
7 · ♙ ♙ · ♙ ♙ ♙ ♙
6 ♙ · · ♟ · · · ·
5 · · · · · · · ·
4 · · · · · · · ·
3 · · · · · · · ·
2 ♟ ♟ ♟ ♟ · ♟ ♟ ♟
1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
  A B C D E F G H",
            after_en_passant.to_string()
        );
        assert_eq!(
            Some(Piece::from(white_pawn)),
            after_en_passant.get_piece(&d6)
        );
        assert_eq!(None, after_en_passant.get_piece(&d5));
        assert_eq!(None, after_en_passant.get_piece(&e5));
    }

    #[test]
    fn white_pieces_are_at_the_bottom_and_black_at_the_top() {
        // Ranks 0 and 1 (bottom rows: Rank 1 and Rank 2) must be White pieces
        for rank in 0..=1 {
            for file in 0..8 {
                let case = INITIAL_POSITION.echiquier.get_case(rank, file);
                let piece = INITIAL_POSITION
                    .get_piece(&case)
                    .expect("piece must exist at initial row");
                assert!(
                    piece.is_white,
                    "piece at rank {} file {} must be white",
                    rank, file
                );
            }
        }

        // Ranks 2..=5 (middle rows: Ranks 3 to 6) must be empty
        for rank in 2..=5 {
            for file in 0..8 {
                let case = INITIAL_POSITION.echiquier.get_case(rank, file);
                assert!(
                    INITIAL_POSITION.get_piece(&case).is_none(),
                    "square at rank {} file {} must be empty",
                    rank,
                    file
                );
            }
        }

        // Ranks 6 and 7 (top rows: Rank 7 and Rank 8) must be Black pieces
        for rank in 6..=7 {
            for file in 0..8 {
                let case = INITIAL_POSITION.echiquier.get_case(rank, file);
                let piece = INITIAL_POSITION
                    .get_piece(&case)
                    .expect("piece must exist at initial row");
                assert!(
                    !piece.is_white,
                    "piece at rank {} file {} must be black",
                    rank, file
                );
            }
        }
    }

    #[test]
    fn game_display_renders_full_board_with_white_at_bottom() {
        let game = Game::default_game();
        assert_eq!(
            "\
8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
6 · · · · · · · ·
5 · · · · · · · ·
4 · · · · · · · ·
3 · · · · · · · ·
2 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
  A B C D E F G H",
            game.current_position().to_string()
        );
    }

    #[test]
    fn game_result_returns_none_and_not_finished_when_game_ongoing() {
        let game = Game::default_game();
        assert!(!game.is_game_finished());
        assert_eq!(None, game.game_result());
    }

    #[test]
    fn game_result_returns_none_when_no_king_on_board() {
        let empty_position = Position::new(&ECHIQUIER);
        let game = Game::new(empty_position);
        assert!(!game.is_game_finished());
        assert_eq!(None, game.game_result());
    }

    #[test]
    fn game_result_black_win_when_white_in_checkmate() {
        let mut position = Position::new(&ECHIQUIER);
        let a1 = position.echiquier.get_case(0, 0);
        let a2 = position.echiquier.get_case(1, 0);
        let b2 = position.echiquier.get_case(1, 1);
        let e1 = position.echiquier.get_case(0, 4);

        let white_king = King::new(a1, true);
        let white_pawn_a = Pawn::new(a2, true);
        let white_pawn_b = Pawn::new(b2, true);
        let black_rook = Rook::new(e1, false);

        position.set_piece(white_king);
        position.set_piece(white_pawn_a);
        position.set_piece(white_pawn_b);
        position.set_piece(black_rook);
        // Black just moved -> White's turn, White is checkmated
        position.last_moved_piece = Some(Piece::from(black_rook));

        let game = Game::new(position);
        assert_eq!(
            "\
8 · · · · · · · ·
7 · · · · · · · ·
6 · · · · · · · ·
5 · · · · · · · ·
4 · · · · · · · ·
3 · · · · · · · ·
2 ♟ ♟ · · · · · ·
1 ♚ · · · ♖ · · ·
  A B C D E F G H",
            position.to_string()
        );
        assert!(game.is_game_finished());
        assert_eq!(Some(Result::BlackWin), game.game_result());
    }

    #[test]
    fn game_result_white_win_when_black_in_checkmate() {
        let mut position = Position::new(&ECHIQUIER);
        let a8 = position.echiquier.get_case(7, 0);
        let a7 = position.echiquier.get_case(6, 0);
        let b7 = position.echiquier.get_case(6, 1);
        let e8 = position.echiquier.get_case(7, 4);

        let black_king = King::new(a8, false);
        let black_pawn_a = Pawn::new(a7, false);
        let black_pawn_b = Pawn::new(b7, false);
        let white_rook = Rook::new(e8, true);

        position.set_piece(black_king);
        position.set_piece(black_pawn_a);
        position.set_piece(black_pawn_b);
        position.set_piece(white_rook);
        // White just moved -> Black's turn, Black is checkmated
        position.last_moved_piece = Some(Piece::from(white_rook));

        let game = Game::new(position);
        assert_eq!(
            "\
8 ♔ · · · ♜ · · ·
7 ♙ ♙ · · · · · ·
6 · · · · · · · ·
5 · · · · · · · ·
4 · · · · · · · ·
3 · · · · · · · ·
2 · · · · · · · ·
1 · · · · · · · ·
  A B C D E F G H",
            position.to_string()
        );
        assert!(game.is_game_finished());
        assert_eq!(Some(Result::WhiteWin), game.game_result());
        assert_eq!(Some(Result::WhiteWin), game.result);
    }

    #[test]
    fn game_result_draw_when_white_in_stalemate() {
        let mut position = Position::new(&ECHIQUIER);
        let a1 = position.echiquier.get_case(0, 0);
        let a3 = position.echiquier.get_case(2, 0);
        let c2 = position.echiquier.get_case(1, 2);

        let white_king = King::new(a1, true);
        let black_king = King::new(a3, false);
        let black_queen = Queen::new(c2, false);

        position.set_piece(white_king);
        position.set_piece(black_king);
        position.set_piece(black_queen);
        // Black just moved -> White's turn, White is stalemated
        position.last_moved_piece = Some(Piece::from(black_queen));

        let game = Game::new(position);
        assert_eq!(
            "\
8 · · · · · · · ·
7 · · · · · · · ·
6 · · · · · · · ·
5 · · · · · · · ·
4 · · · · · · · ·
3 ♔ · · · · · · ·
2 · · ♕ · · · · ·
1 ♚ · · · · · · ·
  A B C D E F G H",
            position.to_string()
        );
        assert!(game.is_game_finished());
        assert_eq!(Some(Result::Draw), game.game_result());
    }

    #[test]
    fn game_result_draw_when_black_in_stalemate() {
        let mut position = Position::new(&ECHIQUIER);
        let a8 = position.echiquier.get_case(7, 0);
        let a6 = position.echiquier.get_case(5, 0);
        let c7 = position.echiquier.get_case(6, 2);

        let black_king = King::new(a8, false);
        let white_king = King::new(a6, true);
        let white_queen = Queen::new(c7, true);

        position.set_piece(black_king);
        position.set_piece(white_king);
        position.set_piece(white_queen);
        // White just moved -> Black's turn, Black is stalemated
        position.last_moved_piece = Some(Piece::from(white_queen));

        let game = Game::new(position);
        assert_eq!(
            "\
8 ♔ · · · · · · ·
7 · · ♛ · · · · ·
6 ♚ · · · · · · ·
5 · · · · · · · ·
4 · · · · · · · ·
3 · · · · · · · ·
2 · · · · · · · ·
1 · · · · · · · ·
  A B C D E F G H",
            position.to_string()
        );
        assert!(game.is_game_finished());
        assert_eq!(Some(Result::Draw), game.game_result());
    }

    #[test]
    fn game_play_move_with_promotion_to_knight() {
        let mut position = Position::new(&ECHIQUIER);
        let e7 = position.echiquier.get_case(6, 4);
        let e8 = position.echiquier.get_case(7, 4);
        let mut white_pawn = Pawn::new(e7, true);
        position.set_piece(white_pawn);

        let mut game = Game::new(position);
        let next_position = *game.play_move_with_promotion(
            &mut white_pawn,
            PieceMove { right: 0, up: 1 },
            Some(PromotionPiece::Knight),
        );

        let piece_at_e8 = next_position.get_piece(&e8).expect("piece must be at e8");
        assert!(matches!(piece_at_e8, Piece::Knight(_)));
    }

    #[test]
    fn game_play_move_without_promotion_keeps_piece() {
        let mut position = Position::new(&ECHIQUIER);
        let e7 = position.echiquier.get_case(6, 4);
        let e8 = position.echiquier.get_case(7, 4);
        let mut white_pawn = Pawn::new(e7, true);
        position.set_piece(white_pawn);

        let mut game = Game::new(position);
        let next_position = *game.play_move(&mut white_pawn, PieceMove { right: 0, up: 1 });

        let piece_at_e8 = next_position.get_piece(&e8).expect("piece must be at e8");
        assert!(matches!(piece_at_e8, Piece::Pawn(_)));
    }

    #[test]
    fn game_play_move_with_promotion_to_queen() {
        let mut position = Position::new(&ECHIQUIER);
        let e7 = position.echiquier.get_case(6, 4);
        let e8 = position.echiquier.get_case(7, 4);
        let mut white_pawn = Pawn::new(e7, true);
        position.set_piece(white_pawn);

        let mut game = Game::new(position);
        let next_position = *game.play_move_with_promotion(
            &mut white_pawn,
            PieceMove { right: 0, up: 1 },
            Some(PromotionPiece::Queen),
        );

        let piece_at_e8 = next_position.get_piece(&e8).expect("piece must be at e8");
        assert!(matches!(piece_at_e8, Piece::Queen(_)));
    }

    #[test]
    fn game_from_text_parses_sequential_moves_with_move_numbers() {
        let text = "1. e4 e5\n2. Nf3 Nc6\n3. Bc4 Bc5";
        let game = Game::from_default_text(text).expect("game should parse");
        assert_eq!(7, game.positions.len());

        let current = game.current_position();
        let c4 = ECHIQUIER.get_case(3, 2);
        let c5 = ECHIQUIER.get_case(4, 2);
        assert!(matches!(
            current.get_piece(&c4),
            Some(Piece::Bishop(b)) if b.is_white
        ));
        assert!(matches!(
            current.get_piece(&c5),
            Some(Piece::Bishop(b)) if !b.is_white
        ));
    }

    #[test]
    fn game_from_text_handles_variations_in_numbering_and_dots() {
        let text = "1.e4 e5 2.Nf3 ... 2... Nc6";
        let game = Game::from_text(INITIAL_POSITION, text).expect("game should parse");
        assert_eq!(5, game.positions.len());
    }

    #[test]
    fn game_from_text_fools_mate() {
        let text = "1. f3 e5 2. g4 Qh4#";
        let game = Game::from_default_text(text).expect("fool's mate should parse");
        assert_eq!(5, game.positions.len());
        assert!(game.is_game_finished());
        assert_eq!(Some(Result::BlackWin), game.game_result());
    }

    #[test]
    fn game_from_text_returns_none_on_invalid_move() {
        let text = "1. e4 e5 2. e5";
        assert_eq!(None, Game::from_default_text(text));
    }

    #[test]
    fn game_from_text_with_empty_or_whitespace_or_dots_only() {
        let text = "   1. ...   ";
        let game = Game::from_default_text(text).expect("empty should parse to initial position");
        assert_eq!(1, game.positions.len());
        assert_eq!(INITIAL_POSITION, *game.current_position());
    }

    #[test]
    fn game_play_text_moves_continues_existing_game() {
        let mut game = Game::default_game();
        let pos1 = *game.play_text_moves("1. e4 e5").expect("moves should play");
        assert_eq!(3, game.positions.len());
        assert_eq!(pos1, *game.current_position());

        let pos2 = *game
            .play_text_moves("2. Nf3 Nc6")
            .expect("moves should play");
        assert_eq!(5, game.positions.len());
        assert_eq!(pos2, *game.current_position());

        assert_eq!(None, game.play_text_moves("invalid"));
    }

    #[test]
    fn game_main_parses_and_formats_roundtrip() {
        // 1. Parse text version of the game from main.rs
        let main_game_text = "1. f4 e5 2. fxe5 d5 3. exd6 Cc6 4. g4 Qh4#";
        let game = Game::from_default_text(main_game_text).expect("game from main.rs should parse");

        // 2. Visual assertion of the final position
        assert_eq!(
            "\
8 ♖ · ♗ · ♔ ♗ ♘ ♖
7 ♙ ♙ ♙ · · ♙ ♙ ♙
6 · · ♘ ♟ · · · ·
5 · · · · · · · ·
4 · · · · · · ♟ ♕
3 · · · · · · · ·
2 ♟ ♟ ♟ ♟ ♟ · · ♟
1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
  A B C D E F G H",
            game.current_position().to_string()
        );
        assert_eq!(Some(Result::BlackWin), game.game_result());

        // 3. Format moves and verify formatting
        let expected_moves = vec![
            "f4".to_string(),
            "e5".to_string(),
            "fxe5".to_string(),
            "d5".to_string(),
            "exd6".to_string(),
            "♘c6".to_string(),
            "g4".to_string(),
            "♕h4#".to_string(),
        ];
        assert_eq!(expected_moves, game.format_moves());

        let expected_pgn = "1. f4 e5 2. fxe5 d5 3. exd6 ♘c6 4. g4 ♕h4# 0-1";
        assert_eq!(expected_pgn, game.format_pgn());
        assert_eq!(
            "1. f4 e5\n2. fxe5 d5\n3. exd6 ♘c6\n4. g4 ♕h4# 0-1",
            game.to_string()
        );

        // 4. Round-trip: parsing the formatted PGN produces the same game
        let round_trip_game =
            Game::from_default_text(&game.format_pgn()).expect("formatted PGN should parse");
        assert_eq!(game, round_trip_game);
    }

    #[test]
    fn test_immortal_game_parsing() {
        let immortal_pgn = "\
[White \"Adolf Anderssen\"]
[Black \"Lionel Kieseritzky\"]

1. e4 e5
2. f4 exf4
3. Fc4 Dh4+
4. Rf1 b5
5. Fxb5 Cf6
6. Cf3 Dh6
7. d3 Ch5
8. Ch4! Dg5
9. Cf5! c6
10. g4 Cf6?!
11. Tg1! cxb5?
12. h4 Dg6
13. h5 Dg5
14. Df3 Cg8
15. Fxf4 Df6
16. Cc3 Fc5?
17. Cd5 Dxb2
18. Fd6!? Fxg1
19. e5! Dxa1+
20. Re2 Ca6??
21. Cxg7+ Rd8
22. Df6+!! Cxf6
23. Fe7#";

        let game = Game::from_default_text(immortal_pgn).expect("immortal game should parse");
        assert_eq!(Some("Adolf Anderssen"), game.white.as_deref());
        assert_eq!(Some("Lionel Kieseritzky"), game.black.as_deref());
        assert_eq!(46, game.positions.len()); // initial position + 45 moves (23*2 - 1)
        assert!(game.is_game_finished());
        assert_eq!(Some(Result::WhiteWin), game.game_result());
        assert_eq!(Some(Result::WhiteWin), game.result);
    }

    #[test]
    fn test_game_of_the_century_parsing() {
        let byrne_fischer_pgn = "\
[White \"Donald Byrne\"]
[Black \"Robert James Fischer\"]

1. Nf3 Nf6
2. c4 g6
3. Nc3 Bg7
4. d4 O-O
5. Bf4 d5
6. Qb3 dxc4
7. Qxc4 c6
8. e4 Nbd7
9. Rd1 Nb6
10. Qc5 Bg4
11. Bg5 Na4
12. Qa3 Nxc3
13. bxc3 Nxe4
14. Bxe7 Qb6
15. Bc4 Nxc3
16. Bc5 Rfe8+
17. Kf1 Be6
18. Bxb6 Bxc4+
19. Kg1 Ne2+
20. Kf1 Nxd4+
21. Kg1 Ne2+
22. Kf1 Nc3+
23. Kg1 axb6
24. Qb4 Ra4
25. Qxb6 Nxd1
26. h3 Rxa2
27. Kh2 Nxf2
28. Re1 Rxe1
29. Qd8+ Bf8
30. Nxe1 Bd5
31. Nf3 Ne4
32. Qb8 b5
33. h4 h5
34. Ne5 Kg7
35. Kg1 Bc5+
36. Kf1 Ng3+
37. Ke1 Bb4+
38. Kd1 Bb3+
39. Kc1 Ne2+
40. Kb1 Nc3+
41. Kc1 Rc2# 0-1";

        let game =
            Game::from_default_text(byrne_fischer_pgn).expect("game of the century should parse");
        assert_eq!(Some("Donald Byrne"), game.white.as_deref());
        assert_eq!(Some("Robert James Fischer"), game.black.as_deref());
        assert_eq!(83, game.positions.len()); // initial position + 82 half-moves
        assert!(game.is_game_finished());
        assert_eq!(Some(Result::BlackWin), game.game_result());
        assert_eq!(Some(Result::BlackWin), game.result);

        let pgn = game.format_pgn();
        assert!(pgn.contains("[White \"Donald Byrne\"]"));
        assert!(pgn.contains("[Black \"Robert James Fischer\"]"));

        let mut continuing_game = Game::new(INITIAL_POSITION);
        continuing_game.play_text_moves("[White \"Player 1\"]\n[Round \"1\"]\n1. e4");
        continuing_game.play_text_moves("[Round \"2\"]\n[Black \"Player 2\"]\n1... e5");
        assert_eq!(Some("Player 1"), continuing_game.white.as_deref());
        assert_eq!(Some("Player 2"), continuing_game.black.as_deref());
        assert_eq!(3, continuing_game.headers.len());
    }

    #[test]
    fn test_game_result_stored_and_displayed() {
        let pgn_header_result =
            Game::from_default_text("[Result \"1-0\"]\n1. e4").expect("header result should parse");
        assert_eq!(Some(Result::WhiteWin), pgn_header_result.result);
        assert_eq!("1. e4 1-0", pgn_header_result.to_string());
        assert_eq!(
            "[Result \"1-0\"]\n\n1. e4 1-0",
            pgn_header_result.format_pgn()
        );

        let pgn_header_black =
            Game::from_default_text("[Result \"0-1\"]\n1. e4").expect("header 0-1");
        assert_eq!(Some(Result::BlackWin), pgn_header_black.result);

        let pgn_header_draw =
            Game::from_default_text("[Result \"1/2-1/2\"]\n1. e4").expect("header 1/2");
        assert_eq!(Some(Result::Draw), pgn_header_draw.result);

        let pgn_header_glyph =
            Game::from_default_text("[Result \"½-½\"]\n1. e4").expect("header ½-½");
        assert_eq!(Some(Result::Draw), pgn_header_glyph.result);

        let white_win_explicit = Game::from_default_text("1. e4 1-0").expect("1-0 text");
        assert_eq!(Some(Result::WhiteWin), white_win_explicit.result);
        assert_eq!("1. e4 1-0", white_win_explicit.format_pgn());

        // Ongoing game
        let ongoing = Game::from_default_text("1. e4").expect("ongoing should parse");
        assert_eq!(None, ongoing.result);
        assert_eq!("1. e4", ongoing.to_string());
        assert_eq!("1. e4", ongoing.format_pgn());

        // Finished game formats with result at the end
        let white_win_game = Game::from_default_text("1. f4 e5 2. g4 Qh4#").expect("game parses");
        assert_eq!(Some(Result::BlackWin), white_win_game.result);
        assert_eq!("1. f4 e5\n2. g4 ♕h4# 0-1", white_win_game.to_string());
        assert_eq!("1. f4 e5 2. g4 ♕h4# 0-1", white_win_game.format_pgn());
    }

    #[test]
    fn test_pgn_equal_at_end_means_draw() {
        let draw_game1 = Game::from_default_text("1. e4 e5 =").expect("draw = should parse");
        assert_eq!(Some(Result::Draw), draw_game1.result);
        assert_eq!("1. e4 e5 1/2-1/2", draw_game1.to_string());
        assert_eq!("1. e4 e5 1/2-1/2", draw_game1.format_pgn());

        let draw_game2 =
            Game::from_default_text("1. e4 e5\n  =  ").expect("draw = with spaces should parse");
        assert_eq!(Some(Result::Draw), draw_game2.result);
        assert_eq!("1. e4 e5 1/2-1/2", draw_game2.to_string());

        let draw_half = Game::from_default_text("1. e4 e5 1/2-1/2").expect("1/2-1/2 should parse");
        assert_eq!(Some(Result::Draw), draw_half.result);
        assert_eq!("1. e4 e5 1/2-1/2", draw_half.to_string());

        let draw_glyph = Game::from_default_text("1. e4 e5 ½-½").expect("½-½ should parse");
        assert_eq!(Some(Result::Draw), draw_glyph.result);
        assert_eq!("1. e4 e5 1/2-1/2", draw_glyph.to_string());

        let draw_half_float =
            Game::from_default_text("1. e4 e5 0.5-0.5").expect("0.5-0.5 should parse");
        assert_eq!(Some(Result::Draw), draw_half_float.result);
        assert_eq!("1. e4 e5 1/2-1/2", draw_half_float.to_string());

        // = not at the end should return None (not a valid move)
        assert_eq!(None, Game::from_default_text("1. e4 = e5"));
    }

    #[test]
    fn test_pgn_resignations() {
        let white_resigns = Game::from_default_text("1. e4 e5 2. f4 White resigns")
            .expect("White resigns should parse");
        assert_eq!(Some(Result::BlackWin), white_resigns.result);
        assert_eq!("1. e4 e5\n2. f4 0-1", white_resigns.to_string());

        let black_resigns = Game::from_default_text("1. e4 e5 2. f4 Black resigns")
            .expect("Black resigns should parse");
        assert_eq!(Some(Result::WhiteWin), black_resigns.result);
        assert_eq!("1. e4 e5\n2. f4 1-0", black_resigns.to_string());

        let blancs_abandonnent = Game::from_default_text("1. e4 e5 2. f4 Blancs abandonnent")
            .expect("Blancs abandonnent should parse");
        assert_eq!(Some(Result::BlackWin), blancs_abandonnent.result);

        let noirs_abandonnent = Game::from_default_text("1. e4 e5 2. f4 Noirs abandonnent")
            .expect("Noirs abandonnent should parse");
        assert_eq!(Some(Result::WhiteWin), noirs_abandonnent.result);

        // Turn-based resignation: after 2. f4 it is Black's turn, so "resigns" means Black resigns
        let turn_resigns_black =
            Game::from_default_text("1. e4 e5 2. f4 resigns").expect("resigns should parse");
        assert_eq!(Some(Result::WhiteWin), turn_resigns_black.result);

        // After 1. e4 it is Black's turn, so "abandon" means Black resigns -> WhiteWin
        let turn_abandon_black =
            Game::from_default_text("1. e4 abandon").expect("abandon should parse");
        assert_eq!(Some(Result::WhiteWin), turn_abandon_black.result);

        // Before move 1 it is White's turn, so "resigns" means White resigns -> BlackWin
        let turn_resigns_white =
            Game::from_default_text("resigns").expect("resigns on turn 1 should parse");
        assert_eq!(Some(Result::BlackWin), turn_resigns_white.result);

        let turn_abandonne_white =
            Game::from_default_text("abandonne").expect("abandonne should parse");
        assert_eq!(Some(Result::BlackWin), turn_abandonne_white.result);
        assert_eq!("0-1", turn_abandonne_white.to_string());
        assert_eq!("0-1", turn_abandonne_white.format_pgn());
    }

    #[test]
    fn test_seven_tag_roster_headers_parsing() {
        let pgn = "\
[Event \"F/S Return Match\"]
[Site \"Belgrade, Serbia JUG\"]
[Date \"1992.11.04\"]
[Round \"29\"]
[White \"Fischer, Robert J.\"]
[Black \"Spassky, Boris V.\"]
[Result \"1/2-1/2\"]

1. e4 e5";
        let game = Game::from_default_text(pgn).expect("valid game");
        assert_eq!(Some("F/S Return Match"), game.event.as_deref());
        assert_eq!(Some("Belgrade, Serbia JUG"), game.site.as_deref());
        assert_eq!(Some("1992.11.04"), game.date.as_deref());
        assert_eq!(Some("29"), game.round.as_deref());
        assert_eq!(Some("Fischer, Robert J."), game.white.as_deref());
        assert_eq!(Some("Spassky, Boris V."), game.black.as_deref());
        assert_eq!(Some(Result::Draw), game.result);
        assert_eq!(Some("F/S Return Match"), game.header("Event"));
        assert_eq!(Some("F/S Return Match"), game.header("event"));
        assert_eq!(Some("29"), game.header("ROUND"));
        assert_eq!(None, game.header("NonExistent"));

        let undefined_pgn = "\
[Event \"?\"]
[Site \"?\"]
[Date \"????.??.??\"]
[Round \"?\"]
[White \"?\"]
[Black \"?\"]
[Result \"*\"]

1. e4";
        let undefined_game =
            Game::from_default_text(undefined_pgn).expect("undefined headers game");
        assert_eq!(Some("?"), undefined_game.event.as_deref());
        assert_eq!(Some("?"), undefined_game.site.as_deref());
        assert_eq!(Some("????.??.??"), undefined_game.date.as_deref());
        assert_eq!(Some("?"), undefined_game.round.as_deref());
        assert_eq!(Some("?"), undefined_game.white.as_deref());
        assert_eq!(Some("?"), undefined_game.black.as_deref());
        assert_eq!(None, undefined_game.result);

        assert!(!undefined_game.is_header_defined("Event"));
        assert!(!undefined_game.is_header_defined("Site"));
        assert!(!undefined_game.is_header_defined("Date"));
        assert!(!undefined_game.is_header_defined("Round"));
        assert!(!undefined_game.is_header_defined("White"));
        assert!(!undefined_game.is_header_defined("Black"));
        assert!(!undefined_game.is_header_defined("Result"));
        assert!(!undefined_game.is_header_defined("NonExistent"));

        assert!(game.is_header_defined("Event"));
        assert!(game.is_header_defined("Site"));
        assert!(game.is_header_defined("Date"));
        assert!(game.is_header_defined("Round"));
        assert!(game.is_header_defined("White"));
        assert!(game.is_header_defined("Black"));
        assert!(game.is_header_defined("Result"));

        let punc_pgn = "[Round \"-\"]\n[Date \"????/??/??\"]\n[Site \"—\"]\n[Event \"..\"]\n1. e4";
        let punc_game = Game::from_default_text(punc_pgn).expect("punctuation headers game");
        assert!(!punc_game.is_header_defined("Round"));
        assert!(!punc_game.is_header_defined("Date"));
        assert!(!punc_game.is_header_defined("Site"));
        assert!(!punc_game.is_header_defined("Event"));
    }

    #[test]
    fn test_custom_title_header_parsing() {
        let pgn = "[Title \"The Immortal Game\"]\n1. e4";
        let game = Game::from_default_text(pgn).expect("valid game with title");
        assert_eq!(Some("The Immortal Game"), game.title.as_deref());
        assert_eq!(Some("The Immortal Game"), game.header("Title"));
        assert!(game.is_header_defined("Title"));

        let no_title = Game::from_default_text("1. e4").expect("game without title");
        assert_eq!(None, no_title.title);
        assert!(!no_title.is_header_defined("Title"));
    }

    #[test]
    fn test_compute_title() {
        let game_with_title =
            Game::from_default_text("[Title \"The Immortal Game\"]\n1. e4").unwrap();
        assert_eq!("The Immortal Game", game_with_title.compute_title());

        let pgn_match = "[White \"Bobby Fischer\"]\n[Black \"Boris Spassky\"]\n[Site \"Reykjavik\"]\n[Date \"1972.07.23\"]\n1. c4";
        let game_match = Game::from_default_text(pgn_match).unwrap();
        assert_eq!(
            "Bobby Fischer - Boris Spassky, Reykjavik, 1972",
            game_match.compute_title()
        );

        let event_game = Game::from_default_text(
            "[Event \"London Tournament\"]\n[White \"Anderssen\"]\n[Black \"Kieseritzky\"]\n1. e4",
        )
        .unwrap();
        assert_eq!(
            "Anderssen - Kieseritzky, London Tournament",
            event_game.compute_title()
        );

        assert_eq!("The Immortal Game", compute_title("invalid pgn"));
        assert_eq!("The Immortal Game", compute_title("1. e4 e5"));
        assert_eq!(
            "Bobby Fischer - Boris Spassky, Reykjavik, 1972",
            compute_title(pgn_match)
        );
    }
}
