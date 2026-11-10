use super::case::Case;
use super::piece_move::PieceMove;
use std::clone::Clone;
use std::cmp::PartialEq;
use std::fmt::{self, Debug, Display};
use std::marker::Copy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Echiquier {
    pub cases: [Case; 64],
}

impl Echiquier {
    pub const fn new() -> Echiquier {
        Echiquier {
            cases: [
                Case { rank: 0, file: 0 },
                Case { rank: 0, file: 1 },
                Case { rank: 0, file: 2 },
                Case { rank: 0, file: 3 },
                Case { rank: 0, file: 4 },
                Case { rank: 0, file: 5 },
                Case { rank: 0, file: 6 },
                Case { rank: 0, file: 7 },
                Case { rank: 1, file: 0 },
                Case { rank: 1, file: 1 },
                Case { rank: 1, file: 2 },
                Case { rank: 1, file: 3 },
                Case { rank: 1, file: 4 },
                Case { rank: 1, file: 5 },
                Case { rank: 1, file: 6 },
                Case { rank: 1, file: 7 },
                Case { rank: 2, file: 0 },
                Case { rank: 2, file: 1 },
                Case { rank: 2, file: 2 },
                Case { rank: 2, file: 3 },
                Case { rank: 2, file: 4 },
                Case { rank: 2, file: 5 },
                Case { rank: 2, file: 6 },
                Case { rank: 2, file: 7 },
                Case { rank: 3, file: 0 },
                Case { rank: 3, file: 1 },
                Case { rank: 3, file: 2 },
                Case { rank: 3, file: 3 },
                Case { rank: 3, file: 4 },
                Case { rank: 3, file: 5 },
                Case { rank: 3, file: 6 },
                Case { rank: 3, file: 7 },
                Case { rank: 4, file: 0 },
                Case { rank: 4, file: 1 },
                Case { rank: 4, file: 2 },
                Case { rank: 4, file: 3 },
                Case { rank: 4, file: 4 },
                Case { rank: 4, file: 5 },
                Case { rank: 4, file: 6 },
                Case { rank: 4, file: 7 },
                Case { rank: 5, file: 0 },
                Case { rank: 5, file: 1 },
                Case { rank: 5, file: 2 },
                Case { rank: 5, file: 3 },
                Case { rank: 5, file: 4 },
                Case { rank: 5, file: 5 },
                Case { rank: 5, file: 6 },
                Case { rank: 5, file: 7 },
                Case { rank: 6, file: 0 },
                Case { rank: 6, file: 1 },
                Case { rank: 6, file: 2 },
                Case { rank: 6, file: 3 },
                Case { rank: 6, file: 4 },
                Case { rank: 6, file: 5 },
                Case { rank: 6, file: 6 },
                Case { rank: 6, file: 7 },
                Case { rank: 7, file: 0 },
                Case { rank: 7, file: 1 },
                Case { rank: 7, file: 2 },
                Case { rank: 7, file: 3 },
                Case { rank: 7, file: 4 },
                Case { rank: 7, file: 5 },
                Case { rank: 7, file: 6 },
                Case { rank: 7, file: 7 },
            ],
        }
    }

    fn exists_case(&self, rank: i8, file: i8) -> bool {
        rank >= 0 && file >= 0 && rank < 8 && file < 8
    }

    pub fn get_case(&self, rank: u8, file: u8) -> Case {
        let index: u8 = rank * 8 + file;
        self.cases[index as usize]
    }

    pub fn get_relative_case(&self, case: &Case, piece_move: PieceMove) -> Option<Case> {
        let rank = (case.rank as i8) + piece_move.up;
        let file = (case.file as i8) + piece_move.right;
        if !self.exists_case(rank, file) {
            return None;
        }
        Some(self.get_case(rank as u8, file as u8))
    }
}

impl Display for Echiquier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
    1      2      3      4      5      6      7      8
A : {} {} {} {} {} {} {} {}
B : {} {} {} {} {} {} {} {}
C : {} {} {} {} {} {} {} {}
D : {} {} {} {} {} {} {} {}
E : {} {} {} {} {} {} {} {}
F : {} {} {} {} {} {} {} {}
G : {} {} {} {} {} {} {} {}
H : {} {} {} {} {} {} {} {}",
            self.cases[0],
            self.cases[1],
            self.cases[2],
            self.cases[3],
            self.cases[4],
            self.cases[5],
            self.cases[6],
            self.cases[7],
            self.cases[8],
            self.cases[9],
            self.cases[10],
            self.cases[11],
            self.cases[12],
            self.cases[13],
            self.cases[14],
            self.cases[15],
            self.cases[16],
            self.cases[17],
            self.cases[18],
            self.cases[19],
            self.cases[20],
            self.cases[21],
            self.cases[22],
            self.cases[23],
            self.cases[24],
            self.cases[25],
            self.cases[26],
            self.cases[27],
            self.cases[28],
            self.cases[29],
            self.cases[30],
            self.cases[31],
            self.cases[32],
            self.cases[33],
            self.cases[34],
            self.cases[35],
            self.cases[36],
            self.cases[37],
            self.cases[38],
            self.cases[39],
            self.cases[40],
            self.cases[41],
            self.cases[42],
            self.cases[43],
            self.cases[44],
            self.cases[45],
            self.cases[46],
            self.cases[47],
            self.cases[48],
            self.cases[49],
            self.cases[50],
            self.cases[51],
            self.cases[52],
            self.cases[53],
            self.cases[54],
            self.cases[55],
            self.cases[56],
            self.cases[57],
            self.cases[58],
            self.cases[59],
            self.cases[60],
            self.cases[61],
            self.cases[62],
            self.cases[63]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fabrique_echiquier() {
        let echiquier: Echiquier = Echiquier::new();
        let a1: Case = echiquier.get_case(0, 0);
        assert_eq!(0, a1.rank);
        assert_eq!(0, a1.file);
        let a8: Case = echiquier.get_case(0, 7);
        assert_eq!(0, a8.rank);
        assert_eq!(7, a8.file);
        let h1: Case = echiquier.get_case(7, 0);
        assert_eq!(7, h1.rank);
        assert_eq!(0, h1.file);
        let h8: Case = echiquier.get_case(7, 7);
        assert_eq!(7, h8.rank);
        assert_eq!(7, h8.file);
    }

    #[test]
    fn check_existance() {
        let echiquier: Echiquier = Echiquier::new();
        assert!(echiquier.exists_case(0, 0));
        assert!(echiquier.exists_case(7, 0));
        assert!(echiquier.exists_case(0, 7));
        assert!(echiquier.exists_case(7, 7));
        assert!(!echiquier.exists_case(-1, 0));
        assert!(!echiquier.exists_case(0, -1));
        assert!(!echiquier.exists_case(0, 8));
        assert!(!echiquier.exists_case(8, 0));
    }

    #[test]
    fn get_from_a1_to_b3_() {
        let echiquier: Echiquier = Echiquier::new();
        assert!(echiquier.exists_case(0, 0));
        let a1: Case = echiquier.get_case(0, 0);
        let no_deplacement: PieceMove = PieceMove { right: 0, up: 0 };
        assert_eq!(Some(a1), echiquier.get_relative_case(&a1, no_deplacement));
        let one_right_two_up: PieceMove = PieceMove { right: 1, up: 2 };
        let b3: Case = echiquier.get_case(2, 1);
        assert_eq!(Some(b3), echiquier.get_relative_case(&a1, one_right_two_up));
    }

    #[test]
    fn get_out_of_bonds_square_from_a1() {
        let echiquier: Echiquier = Echiquier::new();
        assert!(echiquier.exists_case(0, 0));
        let a1: Case = echiquier.get_case(0, 0);
        assert!(
            echiquier
                .get_relative_case(&a1, PieceMove { right: -1, up: 0 })
                .is_none()
        );
        assert!(
            echiquier
                .get_relative_case(&a1, PieceMove { right: 0, up: -1 })
                .is_none()
        );
        assert!(
            echiquier
                .get_relative_case(&a1, PieceMove { right: 8, up: 0 })
                .is_none()
        );
        assert!(
            echiquier
                .get_relative_case(&a1, PieceMove { right: 0, up: 8 })
                .is_none()
        );
    }

    #[test]
    fn print() {
        let echiquier: Echiquier = Echiquier::new();
        assert_eq!(
            "\
    1      2      3      4      5      6      7      8
A : (0, 0) (0, 1) (0, 2) (0, 3) (0, 4) (0, 5) (0, 6) (0, 7)
B : (1, 0) (1, 1) (1, 2) (1, 3) (1, 4) (1, 5) (1, 6) (1, 7)
C : (2, 0) (2, 1) (2, 2) (2, 3) (2, 4) (2, 5) (2, 6) (2, 7)
D : (3, 0) (3, 1) (3, 2) (3, 3) (3, 4) (3, 5) (3, 6) (3, 7)
E : (4, 0) (4, 1) (4, 2) (4, 3) (4, 4) (4, 5) (4, 6) (4, 7)
F : (5, 0) (5, 1) (5, 2) (5, 3) (5, 4) (5, 5) (5, 6) (5, 7)
G : (6, 0) (6, 1) (6, 2) (6, 3) (6, 4) (6, 5) (6, 6) (6, 7)
H : (7, 0) (7, 1) (7, 2) (7, 3) (7, 4) (7, 5) (7, 6) (7, 7)",
            echiquier.to_string()
        );
    }
}
