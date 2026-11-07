use std::clone::Clone;
use std::cmp::PartialEq;
use std::fmt::{self, Debug, Display};
use std::marker::Copy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Case {
    pub rank: u8,
    pub file: u8,
}

impl Case {
    #[allow(dead_code)]
    pub fn algebraic(&self) -> String {
        let file = (b'a' + self.file) as char;
        let rank = (b'1' + self.rank) as char;
        format!("{file}{rank}")
    }

    #[allow(dead_code)]
    pub fn from_algebraic(s: &str) -> Option<Case> {
        let bytes = s.as_bytes();
        if bytes.len() != 2 {
            return None;
        }
        let file = match bytes[0] {
            b'a'..=b'h' => bytes[0] - b'a',
            _ => return None,
        };
        let rank = match bytes[1] {
            b'1'..=b'8' => bytes[1] - b'1',
            _ => return None,
        };
        Some(Case { rank, file })
    }
}

impl Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.rank, self.file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_algebraic_and_display() {
        let c6 = Case { rank: 5, file: 2 };
        assert_eq!("c6", c6.algebraic());
        assert_eq!("(5, 2)", c6.to_string());

        let a1 = Case { rank: 0, file: 0 };
        assert_eq!("a1", a1.algebraic());

        let h8 = Case { rank: 7, file: 7 };
        assert_eq!("h8", h8.algebraic());
    }

    #[test]
    fn case_from_algebraic_valid_and_invalid() {
        assert_eq!(Some(Case { rank: 5, file: 2 }), Case::from_algebraic("c6"));
        assert_eq!(Some(Case { rank: 0, file: 0 }), Case::from_algebraic("a1"));
        assert_eq!(Some(Case { rank: 7, file: 7 }), Case::from_algebraic("h8"));

        assert_eq!(None, Case::from_algebraic(""));
        assert_eq!(None, Case::from_algebraic("a"));
        assert_eq!(None, Case::from_algebraic("a12"));
        assert_eq!(None, Case::from_algebraic("i1"));
        assert_eq!(None, Case::from_algebraic("a0"));
        assert_eq!(None, Case::from_algebraic("a9"));
    }
}
