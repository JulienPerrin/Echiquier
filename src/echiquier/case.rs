use std::clone::Clone;
use std::cmp::PartialEq;
use std::fmt::{self, Display, Debug};
use std::marker::Copy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Case {
    pub rank: u8,
    pub file: u8,
}

impl Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.rank, self.file)
    }
}
