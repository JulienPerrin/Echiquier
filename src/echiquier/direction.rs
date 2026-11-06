#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub const UP: (i8, i8) = (0, 1);
    pub const DOWN: (i8, i8) = (0, -1);
    pub const LEFT: (i8, i8) = (-1, 0);
    pub const RIGHT: (i8, i8) = (1, 0);
    pub const UP_LEFT: (i8, i8) = (-1, 1);
    pub const UP_RIGHT: (i8, i8) = (1, 1);
    pub const DOWN_LEFT: (i8, i8) = (-1, -1);
    pub const DOWN_RIGHT: (i8, i8) = (1, -1);

    pub const ORTHOGONAL: [(i8, i8); 4] = [Self::UP, Self::DOWN, Self::LEFT, Self::RIGHT];

    pub const DIAGONAL: [(i8, i8); 4] = [
        Self::UP_LEFT,
        Self::UP_RIGHT,
        Self::DOWN_LEFT,
        Self::DOWN_RIGHT,
    ];

    pub const ALL: [(i8, i8); 8] = [
        Self::ORTHOGONAL[0],
        Self::ORTHOGONAL[1],
        Self::ORTHOGONAL[2],
        Self::ORTHOGONAL[3],
        Self::DIAGONAL[0],
        Self::DIAGONAL[1],
        Self::DIAGONAL[2],
        Self::DIAGONAL[3],
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_grouped_constants() {
        assert_eq!((0, 1), Direction::UP);
        assert_eq!((0, -1), Direction::DOWN);
        assert_eq!((-1, 0), Direction::LEFT);
        assert_eq!((1, 0), Direction::RIGHT);
        assert_eq!((-1, 1), Direction::UP_LEFT);
        assert_eq!((1, 1), Direction::UP_RIGHT);
        assert_eq!((-1, -1), Direction::DOWN_LEFT);
        assert_eq!((1, -1), Direction::DOWN_RIGHT);

        assert_eq!(4, Direction::ORTHOGONAL.len());
        assert_eq!(4, Direction::DIAGONAL.len());
        assert_eq!(8, Direction::ALL.len());
    }
}
