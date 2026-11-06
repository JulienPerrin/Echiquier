use std::fmt::{self, Display};
use std::ops::Deref;

use crate::echiquier::case::Case;
use crate::echiquier::piece_move::PieceMove;
use crate::pieces::bishop::Bishop;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::movable::Movable;
use crate::pieces::pawn::Pawn;
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

#[cfg(test)]
impl Piece {
    pub const PAWN: Piece = Piece::Pawn(Pawn::new(Case { rank: 0, file: 0 }, true));
    pub const KNIGHT: Piece = Piece::Knight(Knight::new(Case { rank: 0, file: 0 }, true));
    pub const BISHOP: Piece = Piece::Bishop(Bishop::new(Case { rank: 0, file: 0 }, true));
    pub const ROOK: Piece = Piece::Rook(Rook::new(Case { rank: 0, file: 0 }, true));
    pub const QUEEN: Piece = Piece::Queen(Queen::new(Case { rank: 0, file: 0 }, true));
    pub const KING: Piece = Piece::King(King::new(Case { rank: 0, file: 0 }, true));
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
