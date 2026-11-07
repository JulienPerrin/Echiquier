use std::cmp::PartialEq;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PieceMove {
    pub right: i8,
    pub up: i8,
}

impl PieceMove {
    #[allow(dead_code)]
    const fn new(right: i8, up: i8) -> Self {
        PieceMove { right, up }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_piece_move() {
        let m = PieceMove::new(1, 2);
        assert_eq!(1, m.right);
        assert_eq!(2, m.up);
    }
}
