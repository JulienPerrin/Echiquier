use std::clone::Clone;
use std::cmp::PartialEq;
use std::fmt;
use std::marker::Copy;

#[derive(Clone, Copy, fmt::Debug, PartialEq)]
pub struct Case {
    pub range: u8,
    pub file: u8,
}

impl fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.range, self.file)
    }
}
