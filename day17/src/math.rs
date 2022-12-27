use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Add<Direction> for Position {
    type Output = Option<Self>;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::Down if self.y > 0 => Some(Self::new(self.x, self.y - 1)),
            Direction::Left if self.x > 0 => Some(Self::new(self.x - 1, self.y)),
            Direction::Right => Some(Self::new(self.x + 1, self.y)),
            _ => None,
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Down,
    Left,
    Right,
}

#[allow(clippy::fallible_impl_from)]
impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            c => panic!("Invalid direction: {c}"),
        }
    }
}
