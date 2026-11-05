use crate::echiquier::case::Case;
use crate::echiquier::piece_move::PieceMove;
use crate::pieces::piece_kind::PieceKind;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub case: Case,
    pub(crate) kind: PieceKind,
    pub(crate) is_white: bool,
    pub last_move: Option<PieceMove>,
}

impl Piece {
    pub fn new(case: Case, kind: PieceKind, is_white: bool) -> Self {
        Piece {
            case,
            kind,
            is_white,
            last_move: None,
        }
    }
}