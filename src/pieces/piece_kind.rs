#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceKind {
    PAWN = 0,
    KNIGHT = 1,
    BISHOP = 2,
    ROOK = 3,
    QUEEN = 4,
    KING = 5,
}

impl PieceKind {
    #[allow(dead_code)]
    pub const BISSHOP: PieceKind = PieceKind::BISHOP;
}
