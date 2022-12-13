use std::ops::Add;

use Direction::{Down, Left, Right, Up};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Add<Direction> for Position {
    type Output = Option<Self>;

    fn add(self, direction: Direction) -> Self::Output {
        match (self.x, self.y, direction) {
            (x, y, Up) if y > 0 => Some(Self { x, y: y - 1 }),
            (x, y, Down) => Some(Self { x, y: y + 1 }),
            (x, y, Left) if x > 0 => Some(Self { x: x - 1, y }),
            (x, y, Right) => Some(Self { x: x + 1, y }),
            _ => None,
        }
    }
}

const DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];

impl Position {
    pub fn neighbors(self) -> impl Iterator<Item = Self> {
        DIRECTIONS
            .iter()
            .filter_map(move |&direction| self + direction)
    }

    pub const fn distance(self, other: Self) -> usize {
        delta(self.x, other.x) + delta(self.y, other.y)
    }
}

const fn delta(a: usize, b: usize) -> usize {
    if a >= b {
        a - b
    } else {
        b - a
    }
}
