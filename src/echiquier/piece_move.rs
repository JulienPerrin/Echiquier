use std::fmt::Debug;
use std::cmp::PartialEq;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PieceMove {
    pub right: i8,
    pub up: i8,
}
