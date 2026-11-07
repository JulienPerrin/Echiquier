use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Result {
    WhiteWin,
    BlackWin,
    Draw,
}

impl Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Result::WhiteWin => write!(f, "White wins"),
            Result::BlackWin => write!(f, "Black wins"),
            Result::Draw => write!(f, "Draw"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_variants_and_display() {
        assert_eq!(Result::WhiteWin.to_string(), "White wins");
        assert_eq!(Result::BlackWin.to_string(), "Black wins");
        assert_eq!(Result::Draw.to_string(), "Draw");
        assert_ne!(Result::WhiteWin, Result::BlackWin);
        assert_ne!(Result::WhiteWin, Result::Draw);
    }
}
