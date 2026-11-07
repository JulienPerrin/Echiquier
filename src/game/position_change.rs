use std::fmt::{self, Display};

use crate::echiquier::case::Case;
use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::pieces::king::King;
use crate::pieces::piece::Piece;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PositionChange<'a> {
    pub prev: Position<'a>,
    pub next: Position<'a>,
}

fn parse_departure(left: &str) -> Option<(Option<char>, &str)> {
    const GLYPHS: [char; 12] = ['♚', '♔', '♛', '♕', '♜', '♖', '♝', '♗', '♞', '♘', '♟', '♙'];
    if left.is_empty() {
        return Some((None, ""));
    }
    let first = left.chars().next()?;
    if GLYPHS.contains(&first) {
        Some((Some(first), &left[first.len_utf8()..]))
    } else if left.len() == 2
        && matches!(left.as_bytes()[0], b'a'..=b'h')
        && matches!(left.as_bytes()[1], b'1'..=b'8')
    {
        Some((None, left))
    } else if left.len() == 1 && matches!(left.as_bytes()[0], b'a'..=b'h') {
        Some((None, left))
    } else {
        Some((Some(first), &left[first.len_utf8()..]))
    }
}

impl<'a> PositionChange<'a> {
    #[allow(dead_code)]
    pub const fn new(prev: Position<'a>, next: Position<'a>) -> Self {
        PositionChange { prev, next }
    }

    #[allow(dead_code)]
    pub fn format(&self) -> String {
        let Some(moved_piece) = self.next.last_moved_piece.or_else(|| {
            let changed: Vec<Piece> = (0..64)
                .filter_map(|i| self.next.pieces[i])
                .filter(|&p| self.prev.pieces[(p.case.rank * 8 + p.case.file) as usize] != Some(p))
                .collect();
            changed
                .iter()
                .find(|p| matches!(p, Piece::King(_)))
                .copied()
                .or_else(|| changed.into_iter().next())
        }) else {
            return String::new();
        };

        let dest = moved_piece.case.algebraic();
        let origin_case = match moved_piece.last_move {
            Some(m) => {
                let rev = PieceMove { right: -m.right, up: -m.up };
                self.prev.echiquier.get_relative_case(&moved_piece.case, rev)
            }
            None => (0..64).find_map(|i| match self.prev.pieces[i] {
                Some(p)
                    if p.is_white == moved_piece.is_white
                        && std::mem::discriminant(&p) == std::mem::discriminant(&moved_piece)
                        && self.next.pieces[(p.case.rank * 8 + p.case.file) as usize]
                            != Some(p) =>
                {
                    Some(p.case)
                }
                _ => None,
            }),
        };

        let disambiguation = match origin_case {
            Some(origin) => {
                let mut candidates = Vec::new();
                for piece in self.prev.pieces.into_iter().flatten() {
                    if piece.case != origin
                        && piece.is_white == moved_piece.is_white
                        && std::mem::discriminant(&piece) == std::mem::discriminant(&moved_piece)
                        && piece.allowed_moves(&self.prev).iter().any(|&m| {
                            self.prev.echiquier.get_relative_case(&piece.case, m)
                                == Some(moved_piece.case)
                        })
                    {
                        candidates.push(piece);
                    }
                }

                if candidates.is_empty() {
                    String::new()
                } else if candidates.iter().any(|c| c.case.file == origin.file)
                    && candidates.iter().any(|c| c.case.rank == origin.rank)
                {
                    origin.algebraic()
                } else if candidates.iter().any(|c| c.case.file == origin.file) {
                    format!("{}", (b'1' + origin.rank) as char)
                } else {
                    format!("{}", (b'a' + origin.file) as char)
                }
            }
            None => String::new(),
        };

        let captured_piece = (0..64).filter_map(|i| self.prev.pieces[i]).find(|&p| {
            p.is_white != moved_piece.is_white
                && self.next.pieces[(p.case.rank * 8 + p.case.file) as usize] != Some(p)
        });

        let capture_str = match captured_piece {
            None => String::new(),
            Some(Piece::Pawn(_)) => "x".to_string(),
            Some(piece) => format!("x{piece}"),
        };

        let base = match moved_piece {
            Piece::Pawn(_) if captured_piece.is_some() => match origin_case {
                Some(origin) => format!("{}{capture_str}{dest}", (b'a' + origin.file) as char),
                None => format!("{disambiguation}{capture_str}{dest}"),
            },
            Piece::Pawn(_) => format!("{disambiguation}{dest}"),
            _ => format!("{moved_piece}{disambiguation}{capture_str}{dest}"),
        };

        let kings = self.next.get_pieces::<King>(!moved_piece.is_white);
        let opponent_king = kings.into_iter().next();
        let suffix = if let Some(king) = opponent_king {
            if king.is_in_checkmate(&self.next) {
                "#"
            } else if king.is_in_stalemate(&self.next) {
                "="
            } else if king.is_in_check(&self.next) {
                "+"
            } else {
                ""
            }
        } else {
            ""
        };

        format!("{base}{suffix}")
    }

    #[allow(dead_code)]
    fn print(&self) -> String {
        let formatted = self.format();
        println!("{formatted}");
        formatted
    }

    #[allow(dead_code)]
    pub fn from_text(position: &Position<'a>, text: &str) -> Option<Self> {
        let cleaned = text.trim_end_matches(['!', '?', '+', '#', '=']);
        if cleaned.is_empty() {
            return None;
        }

        let is_capture = cleaned.contains(['x', 'X', '×', ':']);
        let (piece_symbol, disambiguation, captured_spec, dest_opt) = if is_capture {
            let capture_pos = cleaned.find(['x', 'X', '×', ':'])?;
            let left = &cleaned[..capture_pos];
            let right = &cleaned[capture_pos + 1..];

            if left.is_empty() || right.is_empty() {
                return None;
            }

            let (captured_spec, dest_opt) = if right.len() >= 2
                && right.is_char_boundary(right.len() - 2)
                && let Some(dest) = Case::from_algebraic(&right[right.len() - 2..])
            {
                (&right[..right.len() - 2], Some(dest))
            } else {
                (right, None)
            };

            let (sym, dis) = parse_departure(left)?;
            (sym, dis, captured_spec, dest_opt)
        } else {
            if cleaned.len() < 2 || !cleaned.is_char_boundary(cleaned.len() - 2) {
                return None;
            }

            let (left, dest_str) = cleaned.split_at(cleaned.len() - 2);
            let dest = Case::from_algebraic(dest_str)?;
            let (sym, dis) = parse_departure(left)?;
            (sym, dis, "", Some(dest))
        };

        let is_white = position.is_white_turn();
        let mut candidates = Vec::new();

        for piece in position.pieces.into_iter().flatten() {
            let color_match = piece.is_white == is_white;
            let kind_match = match piece_symbol {
                Some(symbol) => piece.matches_specifier(symbol),
                None => {
                    if disambiguation.len() == 2 {
                        true
                    } else {
                        matches!(piece, Piece::Pawn(_))
                    }
                }
            };
            let disambig_match = disambiguation.chars().all(|ch| match ch {
                'a'..='h' => piece.case.file == (ch as u8 - b'a'),
                '1'..='8' => piece.case.rank == (ch as u8 - b'1'),
                _ => false,
            });

            if color_match && kind_match && disambig_match {
                if is_capture {
                    for (m, taken) in piece.allowed_captures(position) {
                        let target = position.echiquier.get_relative_case(&piece.case, m)?;
                        let dest_match = dest_opt.map_or(true, |d| target == d);
                        let captured_match = captured_spec
                            .chars()
                            .next()
                            .map_or(true, |c| taken.matches_specifier(c));
                        if dest_match && captured_match {
                            candidates.push((piece, m, Some(taken), target));
                        }
                    }
                } else if let Some(dest) = dest_opt {
                    for &m in &piece.allowed_moves(position) {
                        if position.echiquier.get_relative_case(&piece.case, m) == Some(dest) {
                            candidates.push((piece, m, None, dest));
                        }
                    }
                }
            }
        }

        if candidates.len() != 1 {
            return None;
        }

        let (mut piece, m, taken, target_case) = candidates.remove(0);
        let next = piece.move_piece(position, m, target_case, taken, None);
        Some(PositionChange::new(*position, next))
    }
}

impl<'a> Display for PositionChange<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[allow(dead_code)]
fn format_position_change(prev: &Position, next: &Position) -> String {
    PositionChange::new(*prev, *next).format()
}

#[allow(dead_code)]
fn print_position_change(prev: &Position, next: &Position) -> String {
    PositionChange::new(*prev, *next).print()
}

#[allow(dead_code)]
fn parse_position_change<'a>(
    position: &Position<'a>,
    text: &str,
) -> Option<PositionChange<'a>> {
    PositionChange::from_text(position, text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::echiquier::echiquier::Echiquier;
    use crate::echiquier::piece_move::PieceMove;
    use crate::pieces::bishop::Bishop;
    use crate::pieces::king::King;
    use crate::pieces::knight::Knight;
    use crate::pieces::movable::Movable;
    use crate::pieces::pawn::Pawn;
    use crate::pieces::queen::Queen;
    use crate::pieces::rook::Rook;

    const ECHIQUIER: Echiquier = Echiquier::new();

    #[test]
    fn format_and_print_position_change_between_positions() {
        let mut position = Position::new(&ECHIQUIER);
        let c5 = position.echiquier.get_case(4, 2);
        let c6 = position.echiquier.get_case(5, 2);

        // 1. White King moves to c6 -> "♚c6"
        let mut white_king = King::new(c5, true);
        position.set_piece(white_king);
        let next_white_king =
            white_king.move_piece(&position, PieceMove { right: 0, up: 1 }, c6, None, None);
        let m1 = PositionChange::new(position, next_white_king);
        assert_eq!("♚c6", m1.format());
        assert_eq!("♚c6", m1.to_string());
        assert_eq!("♚c6", m1.print());
        assert_eq!("♚c6", format_position_change(&position, &next_white_king));
        assert_eq!("♚c6", print_position_change(&position, &next_white_king));
        let parsed1 = PositionChange::from_text(&position, "♚c6").expect("parse ♚c6");
        assert_eq!(m1, parsed1);
        assert_eq!("♚c6", parsed1.format());

        // 2. Black King moves to c6 -> "♔c6"
        let mut pos2 = Position::new(&ECHIQUIER);
        let mut black_king = King::new(c5, false);
        pos2.set_piece(black_king);
        pos2.last_moved_piece = Some(Piece::from(white_king));
        let next_black_king =
            black_king.move_piece(&pos2, PieceMove { right: 0, up: 1 }, c6, None, None);
        let m2 = PositionChange::new(pos2, next_black_king);
        assert_eq!("♔c6", m2.format());
        assert_eq!("♔c6", m2.to_string());
        let parsed2 = PositionChange::from_text(&pos2, "♔c6").expect("parse ♔c6");
        assert_eq!(m2, parsed2);
        assert_eq!("♔c6", parsed2.format());

        // 3. White Pawn moves to c6 -> "c6"
        let mut pos3 = Position::new(&ECHIQUIER);
        let mut white_pawn = Pawn::new(c5, true);
        pos3.set_piece(white_pawn);
        let next_white_pawn =
            white_pawn.move_piece(&pos3, PieceMove { right: 0, up: 1 }, c6, None, None);
        let m3 = PositionChange::new(pos3, next_white_pawn);
        assert_eq!("c6", m3.format());
        let parsed3 = PositionChange::from_text(&pos3, "c6").expect("parse c6");
        assert_eq!(m3, parsed3);
        assert_eq!("c6", parsed3.format());

        // 4. Black Pawn moves to e5 -> "e5"
        let e6 = position.echiquier.get_case(5, 4);
        let e5 = position.echiquier.get_case(4, 4);
        let mut pos4 = Position::new(&ECHIQUIER);
        let mut black_pawn = Pawn::new(e6, false);
        pos4.set_piece(black_pawn);
        pos4.last_moved_piece = Some(Piece::from(white_pawn));
        let next_black_pawn =
            black_pawn.move_piece(&pos4, PieceMove { right: 0, up: -1 }, e5, None, None);
        let m4 = PositionChange::new(pos4, next_black_pawn);
        assert_eq!("e5", m4.format());
        let parsed4 = PositionChange::from_text(&pos4, "e5").expect("parse e5");
        assert_eq!(m4, parsed4);
        assert_eq!("e5", parsed4.format());

        // 5. White Queen moves to d4 -> "♛d4"
        let d1 = position.echiquier.get_case(0, 3);
        let d4 = position.echiquier.get_case(3, 3);
        let mut pos5: Position<'_> = Position::new(&ECHIQUIER);
        let mut white_queen = Queen::new(d1, true);
        pos5.set_piece(white_queen);
        let next_white_queen =
            white_queen.move_piece(&pos5, PieceMove { right: 0, up: 3 }, d4, None, None);
        let m5 = PositionChange::new(pos5, next_white_queen);
        assert_eq!("♛d4", m5.format());
        let parsed5 = PositionChange::from_text(&pos5, "♛d4").expect("parse ♛d4");
        assert_eq!(m5, parsed5);
        assert_eq!("♛d4", parsed5.format());

        // 6. Fallback when last_moved_piece is None
        let mut pos6_prev = Position::new(&ECHIQUIER);
        let mut pos6_next = Position::new(&ECHIQUIER);
        pos6_prev.set_piece(King::new(c5, true));
        pos6_next.set_piece(King::new(c6, true));
        let m6 = PositionChange::new(pos6_prev, pos6_next);
        assert_eq!("♚c6", m6.format());

        // 7. Fallback when no move occurred
        let m7 = PositionChange::new(pos6_prev, pos6_prev);
        assert_eq!("", m7.format());

        // 8. Fallback when origin_case is None
        let pos8_prev = Position::new(&ECHIQUIER);
        let mut pos8_next = Position::new(&ECHIQUIER);
        pos8_next.set_piece(King::new(c6, true));
        let m8 = PositionChange::new(pos8_prev, pos8_next);
        assert_eq!("♚c6", m8.format());
    }

    #[test]
    fn format_disambiguates_when_multiple_pieces_can_move_to_same_case() {
        // 1. Two knights on different columns can move to d4 -> disambiguate by column
        let mut pos1 = Position::new(&ECHIQUIER);
        let b3 = pos1.echiquier.get_case(2, 1);
        let f3 = pos1.echiquier.get_case(2, 5);
        let d4 = pos1.echiquier.get_case(3, 3);
        let mut knight_b3 = Knight::new(b3, true);
        let knight_f3 = Knight::new(f3, true);
        pos1.set_piece(knight_b3);
        pos1.set_piece(knight_f3);

        let next_pos1 = knight_b3.move_piece(&pos1, PieceMove { right: 2, up: 1 }, d4, None, None);
        let m1 = PositionChange::new(pos1, next_pos1);
        assert_eq!("♞bd4", m1.format());
        let parsed1 = PositionChange::from_text(&pos1, "♞bd4").expect("parse ♞bd4");
        assert_eq!(m1, parsed1);
        assert_eq!("♞bd4", parsed1.format());

        // 2. Two rooks on the same column can move to a4 -> disambiguate by row
        let mut pos2 = Position::new(&ECHIQUIER);
        let a1 = pos2.echiquier.get_case(0, 0);
        let a8 = pos2.echiquier.get_case(7, 0);
        let a4 = pos2.echiquier.get_case(3, 0);
        let mut rook_a1 = Rook::new(a1, true);
        let rook_a8 = Rook::new(a8, true);
        pos2.set_piece(rook_a1);
        pos2.set_piece(rook_a8);

        let next_pos2 = rook_a1.move_piece(&pos2, PieceMove { right: 0, up: 3 }, a4, None, None);
        let m2 = PositionChange::new(pos2, next_pos2);
        assert_eq!("♜1a4", m2.format());
        let parsed2 = PositionChange::from_text(&pos2, "♜1a4").expect("parse ♜1a4");
        assert_eq!(m2, parsed2);
        assert_eq!("♜1a4", parsed2.format());

        // 3. Three bishops where one shares column and one shares row -> disambiguate by both column and row
        let mut pos3 = Position::new(&ECHIQUIER);
        let c1_case = pos3.echiquier.get_case(0, 2);
        let c5_case = pos3.echiquier.get_case(4, 2);
        let g1_case = pos3.echiquier.get_case(0, 6);
        let e3_case = pos3.echiquier.get_case(2, 4);
        let mut bishop_c1 = Bishop::new(c1_case, true);
        let bishop_c5 = Bishop::new(c5_case, true);
        let bishop_g1 = Bishop::new(g1_case, true);
        pos3.set_piece(bishop_c1);
        pos3.set_piece(bishop_c5);
        pos3.set_piece(bishop_g1);

        let next_pos3 =
            bishop_c1.move_piece(&pos3, PieceMove { right: 2, up: 2 }, e3_case, None, None);
        let m3 = PositionChange::new(pos3, next_pos3);
        assert_eq!("♝c1e3", m3.format());
        let parsed3 = PositionChange::from_text(&pos3, "♝c1e3").expect("parse ♝c1e3");
        assert_eq!(m3, parsed3);
        assert_eq!("♝c1e3", parsed3.format());
    }

    #[test]
    fn format_captures_of_pawn_and_piece() {
        // 1. White King captures Black Pawn -> "♚xe5"
        let mut pos1 = Position::new(&ECHIQUIER);
        let e4 = pos1.echiquier.get_case(3, 4);
        let e5 = pos1.echiquier.get_case(4, 4);
        let mut white_king = King::new(e4, true);
        let black_pawn = Pawn::new(e5, false);
        pos1.set_piece(white_king);
        pos1.set_piece(black_pawn);
        let next_pos1 = white_king.move_piece(
            &pos1,
            PieceMove { right: 0, up: 1 },
            e5,
            Some(Piece::from(black_pawn)),
            None,
        );
        let m1 = PositionChange::new(pos1, next_pos1);
        assert_eq!("♚xe5", m1.format());
        let parsed1 = PositionChange::from_text(&pos1, "♚xe5").expect("parse ♚xe5");
        assert_eq!(m1, parsed1);
        assert_eq!("♚xe5", parsed1.format());

        // 2. White King captures Black Bishop -> "♚x♗e5"
        let mut pos2 = Position::new(&ECHIQUIER);
        let mut white_king2 = King::new(e4, true);
        let black_bishop = Bishop::new(e5, false);
        pos2.set_piece(white_king2);
        pos2.set_piece(black_bishop);
        let next_pos2 = white_king2.move_piece(
            &pos2,
            PieceMove { right: 0, up: 1 },
            e5,
            Some(Piece::from(black_bishop)),
            None,
        );
        let m2 = PositionChange::new(pos2, next_pos2);
        assert_eq!("♚x♗e5", m2.format());
        let parsed2 = PositionChange::from_text(&pos2, "♚x♗e5").expect("parse ♚x♗e5");
        assert_eq!(m2, parsed2);
        assert_eq!("♚x♗e5", parsed2.format());

        // 3. White Pawn captures Black Pawn -> "dxe5"
        let mut pos3 = Position::new(&ECHIQUIER);
        let d4 = pos3.echiquier.get_case(3, 3);
        let mut white_pawn = Pawn::new(d4, true);
        let black_pawn3 = Pawn::new(e5, false);
        pos3.set_piece(white_pawn);
        pos3.set_piece(black_pawn3);
        let next_pos3 = white_pawn.move_piece(
            &pos3,
            PieceMove { right: 1, up: 1 },
            e5,
            Some(Piece::from(black_pawn3)),
            None,
        );
        let m3 = PositionChange::new(pos3, next_pos3);
        assert_eq!("dxe5", m3.format());
        let parsed3 = PositionChange::from_text(&pos3, "dxe5").expect("parse dxe5");
        assert_eq!(m3, parsed3);
        assert_eq!("dxe5", parsed3.format());

        // 4. White Pawn captures Black Bishop -> "dx♗e5"
        let mut pos4 = Position::new(&ECHIQUIER);
        let mut white_pawn4 = Pawn::new(d4, true);
        let black_bishop4 = Bishop::new(e5, false);
        pos4.set_piece(white_pawn4);
        pos4.set_piece(black_bishop4);
        let next_pos4 = white_pawn4.move_piece(
            &pos4,
            PieceMove { right: 1, up: 1 },
            e5,
            Some(Piece::from(black_bishop4)),
            None,
        );
        let m4 = PositionChange::new(pos4, next_pos4);
        assert_eq!("dx♗e5", m4.format());
        let parsed4 = PositionChange::from_text(&pos4, "dx♗e5").expect("parse dx♗e5");
        assert_eq!(m4, parsed4);
        assert_eq!("dx♗e5", parsed4.format());

        // 5. Black King captures White Bishop -> "♔x♝e4"
        let mut pos5 = Position::new(&ECHIQUIER);
        let mut black_king = King::new(e5, false);
        let white_bishop = Bishop::new(e4, true);
        pos5.set_piece(black_king);
        pos5.set_piece(white_bishop);
        pos5.last_moved_piece = Some(Piece::from(white_bishop));
        let next_pos5 = black_king.move_piece(
            &pos5,
            PieceMove { right: 0, up: -1 },
            e4,
            Some(Piece::from(white_bishop)),
            None,
        );
        let m5 = PositionChange::new(pos5, next_pos5);
        assert_eq!("♔x♝e4", m5.format());
        let parsed5 = PositionChange::from_text(&pos5, "♔x♝e4").expect("parse ♔x♝e4");
        assert_eq!(m5, parsed5);
        assert_eq!("♔x♝e4", parsed5.format());

        // 6. Black Pawn captures White Pawn -> "exd4"
        let mut pos6 = Position::new(&ECHIQUIER);
        let mut black_pawn6 = Pawn::new(e5, false);
        let white_pawn6 = Pawn::new(d4, true);
        pos6.set_piece(black_pawn6);
        pos6.set_piece(white_pawn6);
        pos6.last_moved_piece = Some(Piece::from(white_pawn6));
        let next_pos6 = black_pawn6.move_piece(
            &pos6,
            PieceMove { right: -1, up: -1 },
            d4,
            Some(Piece::from(white_pawn6)),
            None,
        );
        let m6 = PositionChange::new(pos6, next_pos6);
        assert_eq!("exd4", m6.format());
        let parsed6 = PositionChange::from_text(&pos6, "exd4").expect("parse exd4");
        assert_eq!(m6, parsed6);
        assert_eq!("exd4", parsed6.format());
    }

    #[test]
    fn format_check_checkmate_and_draw_suffixes() {
        // 1. Move giving Check -> "+" (e.g. "♖e1+")
        let mut pos1 = Position::new(&ECHIQUIER);
        let a1 = pos1.echiquier.get_case(0, 0);
        let e8 = pos1.echiquier.get_case(7, 4);
        let e1 = pos1.echiquier.get_case(0, 4);
        let white_king = King::new(a1, true);
        let mut black_rook = Rook::new(e8, false);
        pos1.set_piece(white_king);
        pos1.set_piece(black_rook);
        pos1.last_moved_piece = Some(Piece::from(white_king));
        let next_pos1 =
            black_rook.move_piece(&pos1, PieceMove { right: 0, up: -7 }, e1, None, None);
        let m1 = PositionChange::new(pos1, next_pos1);
        assert_eq!("♖e1+", m1.format());
        let parsed1 = PositionChange::from_text(&pos1, "♖e1+").expect("parse ♖e1+");
        assert_eq!(m1, parsed1);
        let parsed1_letter = PositionChange::from_text(&pos1, "Re1+").expect("parse Re1+");
        assert_eq!(m1, parsed1_letter);

        // 2. Move giving Checkmate -> "#" (e.g. "♖e1#")
        let mut pos2 = Position::new(&ECHIQUIER);
        let a2 = pos2.echiquier.get_case(1, 0);
        let b2 = pos2.echiquier.get_case(1, 1);
        let white_king2 = King::new(a1, true);
        let white_pawn_a = Pawn::new(a2, true);
        let white_pawn_b = Pawn::new(b2, true);
        let mut black_rook2 = Rook::new(e8, false);
        pos2.set_piece(white_king2);
        pos2.set_piece(white_pawn_a);
        pos2.set_piece(white_pawn_b);
        pos2.set_piece(black_rook2);
        pos2.last_moved_piece = Some(Piece::from(white_king2));
        let next_pos2 =
            black_rook2.move_piece(&pos2, PieceMove { right: 0, up: -7 }, e1, None, None);
        let m2 = PositionChange::new(pos2, next_pos2);
        assert_eq!("♖e1#", m2.format());
        let parsed2 = PositionChange::from_text(&pos2, "♖e1#").expect("parse ♖e1#");
        assert_eq!(m2, parsed2);
        let parsed2_letter = PositionChange::from_text(&pos2, "Re1#").expect("parse Re1#");
        assert_eq!(m2, parsed2_letter);

        // 3. Move causing Stalemate / Draw -> "=" (e.g. "♛c7=")
        let mut pos3 = Position::new(&ECHIQUIER);
        let a8 = pos3.echiquier.get_case(7, 0);
        let a6 = pos3.echiquier.get_case(5, 0);
        let d7 = pos3.echiquier.get_case(6, 3);
        let c7 = pos3.echiquier.get_case(6, 2);
        let black_king = King::new(a8, false);
        let white_king3 = King::new(a6, true);
        let mut white_queen = Queen::new(d7, true);
        pos3.set_piece(black_king);
        pos3.set_piece(white_king3);
        pos3.set_piece(white_queen);
        pos3.last_moved_piece = Some(Piece::from(black_king));
        let next_pos3 =
            white_queen.move_piece(&pos3, PieceMove { right: -1, up: 0 }, c7, None, None);
        let m3 = PositionChange::new(pos3, next_pos3);
        assert_eq!("♛c7=", m3.format());
        let parsed3 = PositionChange::from_text(&pos3, "♛c7=").expect("parse ♛c7=");
        assert_eq!(m3, parsed3);
        let parsed3_letter = PositionChange::from_text(&pos3, "Qc7=").expect("parse Qc7=");
        assert_eq!(m3, parsed3_letter);

        // 4. Move with opponent king present but safe -> no suffix
        let mut pos4 = Position::new(&ECHIQUIER);
        let h8 = pos4.echiquier.get_case(7, 7);
        let c5 = pos4.echiquier.get_case(4, 2);
        let c6 = pos4.echiquier.get_case(5, 2);
        let mut white_king4 = King::new(c5, true);
        let black_king4 = King::new(h8, false);
        pos4.set_piece(white_king4);
        pos4.set_piece(black_king4);
        let next_pos4 =
            white_king4.move_piece(&pos4, PieceMove { right: 0, up: 1 }, c6, None, None);
        let m4 = PositionChange::new(pos4, next_pos4);
        assert_eq!("♚c6", m4.format());
        let parsed4 = PositionChange::from_text(&pos4, "♚c6").expect("parse ♚c6");
        assert_eq!(m4, parsed4);
        let parsed4_letter = PositionChange::from_text(&pos4, "Kc6").expect("parse Kc6");
        assert_eq!(m4, parsed4_letter);

        // 5. Fallback when pawn captures without origin_case
        let mut pos5_prev = Position::new(&ECHIQUIER);
        let mut pos5_next = Position::new(&ECHIQUIER);
        let e5 = pos5_prev.echiquier.get_case(4, 4);
        pos5_prev.set_piece(Bishop::new(e5, false));
        pos5_next.set_piece(Pawn::new(e5, true));
        let m5 = PositionChange::new(pos5_prev, pos5_next);
        assert_eq!("x♗e5", m5.format());
    }

    #[test]
    fn parse_position_change_from_position_and_simple_text() {
        let mut position = Position::new(&ECHIQUIER);
        let c5 = position.echiquier.get_case(4, 2);

        // 1. White Pawn moves to c6 -> "c6"
        let white_pawn = Pawn::new(c5, true);
        position.set_piece(white_pawn);
        let m1 = PositionChange::from_text(&position, "c6").expect("move should be parsed");
        assert_eq!("c6", m1.format());

        // 2. White King moves to c6 -> "♚c6"
        let mut pos2 = Position::new(&ECHIQUIER);
        let white_king = King::new(c5, true);
        pos2.set_piece(white_king);
        let m2 = PositionChange::from_text(&pos2, "♚c6").expect("move should be parsed");
        assert_eq!("♚c6", m2.format());

        // 3. Black King moves to c6 -> "♔c6"
        let mut pos3 = Position::new(&ECHIQUIER);
        let black_king = King::new(c5, false);
        pos3.set_piece(black_king);
        pos3.last_moved_piece = Some(Piece::from(white_king)); // Black's turn
        let m3 = PositionChange::from_text(&pos3, "♔c6").expect("move should be parsed");
        assert_eq!("♔c6", m3.format());

        // 4. White Queen moves to d4 -> "♛d4"
        let mut pos4 = Position::new(&ECHIQUIER);
        let d1 = pos4.echiquier.get_case(0, 3);
        let white_queen = Queen::new(d1, true);
        pos4.set_piece(white_queen);
        let m4 = PositionChange::from_text(&pos4, "♛d4").expect("move should be parsed");
        assert_eq!("♛d4", m4.format());

        // 5. Invalid moves or text return None
        assert_eq!(None, PositionChange::from_text(&position, ""));
        assert_eq!(None, PositionChange::from_text(&position, "e"));
        assert_eq!(None, PositionChange::from_text(&position, "♚"));
        assert_eq!(None, PositionChange::from_text(&position, "c9"));
        assert_eq!(None, PositionChange::from_text(&position, "♛e8")); // No queen on board
    }

    #[test]
    fn parse_position_change_with_disambiguation() {
        // 1. Two knights on different columns can move to d4 -> disambiguate by column
        let mut pos1 = Position::new(&ECHIQUIER);
        let b3 = pos1.echiquier.get_case(2, 1);
        let f3 = pos1.echiquier.get_case(2, 5);
        let knight_b3 = Knight::new(b3, true);
        let knight_f3 = Knight::new(f3, true);
        pos1.set_piece(knight_b3);
        pos1.set_piece(knight_f3);

        // Ambiguous without disambiguation returns None
        assert_eq!(None, PositionChange::from_text(&pos1, "♞d4"));
        // Invalid disambiguation returns None
        assert_eq!(None, PositionChange::from_text(&pos1, "♞zd4"));

        // Disambiguated by file 'b'
        let m1 = PositionChange::from_text(&pos1, "♞bd4").expect("move should be parsed");
        assert_eq!("♞bd4", m1.format());
        assert_eq!(Some(m1), parse_position_change(&pos1, "♞bd4"));
        assert_eq!(Some(m1), PositionChange::from_text(&pos1, "Nbd4"));
        assert_eq!(Some(m1), PositionChange::from_text(&pos1, "Cbd4"));

        // 2. Two rooks on the same column can move to a4 -> disambiguate by row
        let mut pos2 = Position::new(&ECHIQUIER);
        let a1 = pos2.echiquier.get_case(0, 0);
        let a8 = pos2.echiquier.get_case(7, 0);
        let rook_a1 = Rook::new(a1, true);
        let rook_a8 = Rook::new(a8, true);
        pos2.set_piece(rook_a1);
        pos2.set_piece(rook_a8);

        let m2 = PositionChange::from_text(&pos2, "♜1a4").expect("move should be parsed");
        assert_eq!("♜1a4", m2.format());
        assert_eq!(Some(m2), PositionChange::from_text(&pos2, "R1a4"));

        // 3. Three bishops -> disambiguate by both column and row
        let mut pos3 = Position::new(&ECHIQUIER);
        let c1_case = pos3.echiquier.get_case(0, 2);
        let c5_case = pos3.echiquier.get_case(4, 2);
        let g1_case = pos3.echiquier.get_case(0, 6);
        let bishop_c1 = Bishop::new(c1_case, true);
        let bishop_c5 = Bishop::new(c5_case, true);
        let bishop_g1 = Bishop::new(g1_case, true);
        pos3.set_piece(bishop_c1);
        pos3.set_piece(bishop_c5);
        pos3.set_piece(bishop_g1);

        let m3 = PositionChange::from_text(&pos3, "♝c1e3").expect("move should be parsed");
        assert_eq!("♝c1e3", m3.format());
        assert_eq!(Some(m3), PositionChange::from_text(&pos3, "Bc1e3"));
        assert_eq!(Some(m3), PositionChange::from_text(&pos3, "c1e3"));
    }

    #[test]
    fn parse_position_change_with_captures_permissive() {
        // Setup: White Knight on c4, Black Knight on e5
        let mut pos = Position::new(&ECHIQUIER);
        let c4 = pos.echiquier.get_case(3, 2);
        let e5 = pos.echiquier.get_case(4, 4);
        let white_knight = Knight::new(c4, true);
        let black_knight = Knight::new(e5, false);
        pos.set_piece(white_knight);
        pos.set_piece(black_knight);

        // 1. All 4 variations from user request for Knight taking Knight on e5:
        // Kc4xe5, Kc4xCe5, c4xe5, c4xCe5
        let m1 = PositionChange::from_text(&pos, "Kc4xe5").expect("Kc4xe5 should parse");
        let m2 = PositionChange::from_text(&pos, "Kc4xCe5").expect("Kc4xCe5 should parse");
        let m3 = PositionChange::from_text(&pos, "c4xe5").expect("c4xe5 should parse");
        let m4 = PositionChange::from_text(&pos, "c4xCe5").expect("c4xCe5 should parse");

        assert_eq!(m1, m2);
        assert_eq!(m1, m3);
        assert_eq!(m1, m4);
        assert_eq!("♞x♘e5", m1.format());

        // 2. Illegal move does not panic, returns None:
        assert_eq!(None, PositionChange::from_text(&pos, "")); // Empty text
        assert_eq!(None, PositionChange::from_text(&pos, "!?+#=")); // Only annotations
        assert_eq!(None, PositionChange::from_text(&pos, "Kc3xe5")); // Knight not on c3
        assert_eq!(None, PositionChange::from_text(&pos, "c4x")); // Incomplete target
        assert_eq!(None, PositionChange::from_text(&pos, "xe5")); // Empty left side

        // 3. Permissive annotations (!, ?, +, #) ignored during parsing:
        let m_check = PositionChange::from_text(&pos, "c4xe5+").expect("c4xe5+ should parse");
        assert_eq!(m1, m_check);
        let m_mate = PositionChange::from_text(&pos, "c4xe5#").expect("c4xe5# should parse");
        assert_eq!(m1, m_mate);
        let m_excl = PositionChange::from_text(&pos, "c4xe5!").expect("c4xe5! should parse");
        assert_eq!(m1, m_excl);
        let m_question = PositionChange::from_text(&pos, "c4xe5?").expect("c4xe5? should parse");
        assert_eq!(m1, m_question);
        let m_combo = PositionChange::from_text(&pos, "c4xe5!?").expect("c4xe5!? should parse");
        assert_eq!(m1, m_combo);
        let m_combo2 = PositionChange::from_text(&pos, "c4xe5?!").expect("c4xe5?! should parse");
        assert_eq!(m1, m_combo2);
        let m_check_excl =
            PositionChange::from_text(&pos, "c4xe5+!").expect("c4xe5+! should parse");
        assert_eq!(m1, m_check_excl);

        // 4. Pawn captures piece: "dxe5" and "dxCe5"
        let mut pos_pawn = Position::new(&ECHIQUIER);
        let d4 = pos_pawn.echiquier.get_case(3, 3);
        let white_pawn = Pawn::new(d4, true);
        pos_pawn.set_piece(white_pawn);
        pos_pawn.set_piece(black_knight);
        let m_pawn = PositionChange::from_text(&pos_pawn, "dxe5").expect("dxe5 should parse");
        assert_eq!("dx♘e5", m_pawn.format());
        let m_pawn_piece =
            PositionChange::from_text(&pos_pawn, "dxCe5").expect("dxCe5 should parse");
        assert_eq!(m_pawn, m_pawn_piece);

        // 5. King captures bishop: "♚xe5" and "♚x♗e5"
        let mut pos_king = Position::new(&ECHIQUIER);
        let e4 = pos_king.echiquier.get_case(3, 4);
        let white_king = King::new(e4, true);
        let black_bishop = Bishop::new(e5, false);
        pos_king.set_piece(white_king);
        pos_king.set_piece(black_bishop);
        let m_king = PositionChange::from_text(&pos_king, "♚xe5").expect("♚xe5 should parse");
        assert_eq!("♚x♗e5", m_king.format());
        let m_king2 = PositionChange::from_text(&pos_king, "♚x♗e5").expect("♚x♗e5 should parse");
        assert_eq!(m_king, m_king2);

        // 6. Kxe5 and ♞xe5 (no departure disambiguation when only one move is possible)
        let m_k_xe5 = PositionChange::from_text(&pos, "Kxe5").expect("Kxe5 should parse");
        let m_glyph_xe5 = PositionChange::from_text(&pos, "♞xe5").expect("♞xe5 should parse");
        let m_k_xce5 = PositionChange::from_text(&pos, "KxCe5").expect("KxCe5 should parse");
        let m_k_xne5 = PositionChange::from_text(&pos, "KxNe5").expect("KxNe5 should parse");
        let m_glyph_xne5 = PositionChange::from_text(&pos, "♞x♘e5").expect("♞x♘e5 should parse");
        assert_eq!(m1, m_k_xe5);
        assert_eq!(m1, m_glyph_xe5);
        assert_eq!(m1, m_k_xce5);
        assert_eq!(m1, m_k_xne5);
        assert_eq!(m1, m_glyph_xne5);

        // 7. KxK and ♞x♘ (no destination when only one capture is possible)
        let m_kxk = PositionChange::from_text(&pos, "KxK").expect("KxK should parse");
        let m_glyph_x = PositionChange::from_text(&pos, "♞x♘").expect("♞x♘ should parse");
        assert_eq!(m1, m_kxk);
        assert_eq!(m1, m_glyph_x);

        // 8. Ambiguous KxK returns None when multiple captures match
        let d6 = pos.echiquier.get_case(5, 3);
        let black_knight2 = Knight::new(d6, false);
        let mut pos_ambiguous = pos;
        pos_ambiguous.set_piece(black_knight2); // c4 knight can take e5 or d6 (2 knight captures)
        assert_eq!(None, PositionChange::from_text(&pos_ambiguous, "KxK"));
    }
}
