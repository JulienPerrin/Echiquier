use std::fmt::{self, Display};
use std::ops::Deref;

use crate::echiquier::case::Case;
use crate::echiquier::piece_move::PieceMove;
use crate::echiquier::position::Position;
use crate::pieces::bishop::Bishop;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::movable::Movable;
use crate::pieces::pawn::{Pawn, PromotionPiece};
use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PieceData {
    pub case: Case,
    pub(crate) is_white: bool,
    pub last_move: Option<PieceMove>,
}

impl PieceData {
    pub const fn new(case: Case, is_white: bool) -> Self {
        PieceData {
            case,
            is_white,
            last_move: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    Pawn(Pawn),
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

impl Deref for Piece {
    type Target = dyn Movable;

    fn deref(&self) -> &Self::Target {
        match self {
            Piece::Pawn(p) => p,
            Piece::Knight(p) => p,
            Piece::Bishop(p) => p,
            Piece::Rook(p) => p,
            Piece::Queen(p) => p,
            Piece::King(p) => p,
        }
    }
}

impl Piece {
    #[allow(dead_code)]
    pub fn symbol(&self) -> char {
        let (white, black) = match self {
            Piece::Pawn(_) => ('♟', '♙'),
            Piece::Knight(_) => ('♞', '♘'),
            Piece::Bishop(_) => ('♝', '♗'),
            Piece::Rook(_) => ('♜', '♖'),
            Piece::Queen(_) => ('♛', '♕'),
            Piece::King(_) => ('♚', '♔'),
        };
        if self.is_white { white } else { black }
    }

    pub fn matches_specifier(&self, c: char) -> bool {
        match self {
            Piece::King(_) => matches!(c, '♚' | '♔' | 'K' | 'k' | 'R' | 'r'),
            Piece::Queen(_) => matches!(c, '♛' | '♕' | 'Q' | 'q' | 'D'),
            Piece::Rook(_) => matches!(c, '♜' | '♖' | 'R' | 'r' | 'T' | 't'),
            Piece::Bishop(_) => matches!(c, '♝' | '♗' | 'B' | 'F'),
            Piece::Knight(_) => matches!(c, '♞' | '♘' | 'N' | 'n' | 'C' | 'K' | 'k'),
            Piece::Pawn(_) => matches!(c, '♟' | '♙' | 'P' | 'p'),
        }
    }

    pub fn move_piece<'a>(
        &mut self,
        position: &Position<'a>,
        piece_move: PieceMove,
        case: Case,
        piece_taken: Option<Piece>,
        promotion: Option<PromotionPiece>,
    ) -> Position<'a> {
        let next = match self {
            Piece::Pawn(p) => p.move_piece(position, piece_move, case, piece_taken, promotion),
            Piece::Knight(p) => p.move_piece(position, piece_move, case, piece_taken, promotion),
            Piece::Bishop(p) => p.move_piece(position, piece_move, case, piece_taken, promotion),
            Piece::Rook(p) => p.move_piece(position, piece_move, case, piece_taken, promotion),
            Piece::Queen(p) => p.move_piece(position, piece_move, case, piece_taken, promotion),
            Piece::King(p) => p.move_piece(position, piece_move, case, piece_taken, promotion),
        };
        *self = next.get_piece(&case).expect("piece must be at target case");
        next
    }

    pub fn with_case(self, case: Case) -> Self {
        match self {
            Piece::Pawn(mut p) => {
                p.case = case;
                Piece::Pawn(p)
            }
            Piece::Knight(mut p) => {
                p.case = case;
                Piece::Knight(p)
            }
            Piece::Bishop(mut p) => {
                p.case = case;
                Piece::Bishop(p)
            }
            Piece::Rook(mut p) => {
                p.case = case;
                Piece::Rook(p)
            }
            Piece::Queen(mut p) => {
                p.case = case;
                Piece::Queen(p)
            }
            Piece::King(mut p) => {
                p.case = case;
                Piece::King(p)
            }
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&**self, f)
    }
}

impl From<Pawn> for Piece {
    fn from(piece: Pawn) -> Self {
        Piece::Pawn(piece)
    }
}

impl From<Knight> for Piece {
    fn from(piece: Knight) -> Self {
        Piece::Knight(piece)
    }
}

impl From<Bishop> for Piece {
    fn from(piece: Bishop) -> Self {
        Piece::Bishop(piece)
    }
}

impl From<Rook> for Piece {
    fn from(piece: Rook) -> Self {
        Piece::Rook(piece)
    }
}

impl From<Queen> for Piece {
    fn from(piece: Queen) -> Self {
        Piece::Queen(piece)
    }
}

impl From<King> for Piece {
    fn from(piece: King) -> Self {
        Piece::King(piece)
    }
}

impl TryFrom<Piece> for Pawn {
    type Error = ();

    fn try_from(piece: Piece) -> Result<Self, Self::Error> {
        match piece {
            Piece::Pawn(p) => Ok(p),
            _ => Err(()),
        }
    }
}

impl TryFrom<Piece> for Knight {
    type Error = ();

    fn try_from(piece: Piece) -> Result<Self, Self::Error> {
        match piece {
            Piece::Knight(p) => Ok(p),
            _ => Err(()),
        }
    }
}

impl TryFrom<Piece> for Bishop {
    type Error = ();

    fn try_from(piece: Piece) -> Result<Self, Self::Error> {
        match piece {
            Piece::Bishop(p) => Ok(p),
            _ => Err(()),
        }
    }
}

impl TryFrom<Piece> for Rook {
    type Error = ();

    fn try_from(piece: Piece) -> Result<Self, Self::Error> {
        match piece {
            Piece::Rook(p) => Ok(p),
            _ => Err(()),
        }
    }
}

impl TryFrom<Piece> for Queen {
    type Error = ();

    fn try_from(piece: Piece) -> Result<Self, Self::Error> {
        match piece {
            Piece::Queen(p) => Ok(p),
            _ => Err(()),
        }
    }
}

impl TryFrom<Piece> for King {
    type Error = ();

    fn try_from(piece: Piece) -> Result<Self, Self::Error> {
        match piece {
            Piece::King(p) => Ok(p),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::echiquier::echiquier::Echiquier;
    use crate::echiquier::piece_move::PieceMove;
    use crate::echiquier::position::Position;

    #[test]
    fn try_from_piece_success_and_failure() {
        let case = Case { rank: 0, file: 0 };
        let pawn = Pawn::new(case, true);
        let knight = Knight::new(case, true);
        let bishop = Bishop::new(case, true);
        let rook = Rook::new(case, true);
        let queen = Queen::new(case, true);
        let king = King::new(case, true);

        let piece_pawn = Piece::from(pawn);
        let piece_knight = Piece::from(knight);
        let piece_bishop = Piece::from(bishop);
        let piece_rook = Piece::from(rook);
        let piece_queen = Piece::from(queen);
        let piece_king = Piece::from(king);

        assert_eq!(Ok(pawn), Pawn::try_from(piece_pawn));
        assert_eq!(Err(()), Pawn::try_from(piece_king));

        assert_eq!(Ok(knight), Knight::try_from(piece_knight));
        assert_eq!(Err(()), Knight::try_from(piece_pawn));

        assert_eq!(Ok(bishop), Bishop::try_from(piece_bishop));
        assert_eq!(Err(()), Bishop::try_from(piece_pawn));

        assert_eq!(Ok(rook), Rook::try_from(piece_rook));
        assert_eq!(Err(()), Rook::try_from(piece_pawn));

        assert_eq!(Ok(queen), Queen::try_from(piece_queen));
        assert_eq!(Err(()), Queen::try_from(piece_pawn));

        assert_eq!(Ok(king), King::try_from(piece_king));
        assert_eq!(Err(()), King::try_from(piece_pawn));
    }

    #[test]
    fn piece_with_case_updates_case_for_all_variants() {
        let from = Case { rank: 0, file: 0 };
        let to = Case { rank: 3, file: 4 };

        let pieces = [
            Piece::from(Pawn::new(from, true)),
            Piece::from(Knight::new(from, true)),
            Piece::from(Bishop::new(from, true)),
            Piece::from(Rook::new(from, true)),
            Piece::from(Queen::new(from, true)),
            Piece::from(King::new(from, true)),
        ];

        for piece in pieces {
            let moved = piece.with_case(to);
            assert_eq!(to, moved.case);
        }
    }

    #[test]
    fn piece_symbol_returns_glyph_for_each_variant() {
        let case = Case { rank: 0, file: 0 };
        assert_eq!('♟', Piece::from(Pawn::new(case, true)).symbol());
        assert_eq!('♙', Piece::from(Pawn::new(case, false)).symbol());
        assert_eq!('♞', Piece::from(Knight::new(case, true)).symbol());
        assert_eq!('♘', Piece::from(Knight::new(case, false)).symbol());
        assert_eq!('♝', Piece::from(Bishop::new(case, true)).symbol());
        assert_eq!('♗', Piece::from(Bishop::new(case, false)).symbol());
        assert_eq!('♜', Piece::from(Rook::new(case, true)).symbol());
        assert_eq!('♖', Piece::from(Rook::new(case, false)).symbol());
        assert_eq!('♛', Piece::from(Queen::new(case, true)).symbol());
        assert_eq!('♕', Piece::from(Queen::new(case, false)).symbol());
        assert_eq!('♚', Piece::from(King::new(case, true)).symbol());
        assert_eq!('♔', Piece::from(King::new(case, false)).symbol());
    }

    #[test]
    fn piece_move_piece_delegates_to_inner_movable() {
        let echiquier = Echiquier::new();
        let mut pos = Position::new(&echiquier);
        let e2 = echiquier.get_case(1, 4);
        let e4 = echiquier.get_case(3, 4);
        let mut piece = Piece::from(Pawn::new(e2, true));
        pos.set_piece(piece);
        let next_pos = piece.move_piece(&pos, PieceMove { right: 0, up: 2 }, e4, None, None);
        assert_eq!(e4, piece.case);
        assert_eq!(Some(piece), next_pos.get_piece(&e4));
    }

    #[test]
    fn piece_matches_specifier_recognizes_letters_and_glyphs() {
        let case = Case { rank: 0, file: 0 };
        let knight = Piece::from(Knight::new(case, true));
        assert!(knight.matches_specifier('♞'));
        assert!(knight.matches_specifier('♘'));
        assert!(knight.matches_specifier('C'));
        assert!(!knight.matches_specifier('c')); // minuscules < 'h' are rejected (files)
        assert!(knight.matches_specifier('N'));
        assert!(knight.matches_specifier('n'));
        assert!(knight.matches_specifier('K'));
        assert!(knight.matches_specifier('k'));
        assert!(!knight.matches_specifier('B'));

        let bishop = Piece::from(Bishop::new(case, true));
        assert!(bishop.matches_specifier('♝'));
        assert!(bishop.matches_specifier('♗'));
        assert!(bishop.matches_specifier('B'));
        assert!(bishop.matches_specifier('F'));
        assert!(!bishop.matches_specifier('b')); // minuscules < 'h' are rejected
        assert!(!bishop.matches_specifier('f')); // minuscules < 'h' are rejected
        assert!(!bishop.matches_specifier('C'));

        let king = Piece::from(King::new(case, true));
        assert!(king.matches_specifier('♚'));
        assert!(king.matches_specifier('♔'));
        assert!(king.matches_specifier('K'));
        assert!(king.matches_specifier('k'));
        assert!(king.matches_specifier('R'));
        assert!(king.matches_specifier('r'));
        assert!(!king.matches_specifier('Q'));

        let queen = Piece::from(Queen::new(case, true));
        assert!(queen.matches_specifier('♛'));
        assert!(queen.matches_specifier('♕'));
        assert!(queen.matches_specifier('Q'));
        assert!(queen.matches_specifier('q'));
        assert!(queen.matches_specifier('D'));
        assert!(!queen.matches_specifier('d')); // minuscules < 'h' are rejected

        let rook = Piece::from(Rook::new(case, true));
        assert!(rook.matches_specifier('♜'));
        assert!(rook.matches_specifier('♖'));
        assert!(rook.matches_specifier('R'));
        assert!(rook.matches_specifier('r'));
        assert!(rook.matches_specifier('T'));
        assert!(rook.matches_specifier('t'));

        let pawn = Piece::from(Pawn::new(case, true));
        assert!(pawn.matches_specifier('♟'));
        assert!(pawn.matches_specifier('♙'));
        assert!(pawn.matches_specifier('P'));
        assert!(pawn.matches_specifier('p'));
        assert!(!pawn.matches_specifier('K'));
    }
}
