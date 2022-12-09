use crate::{Direction, Motion};
use std::collections::HashSet;
use std::ops::Add;

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

pub struct Rope {
    knots: Vec<Position>,
    tail_positions: HashSet<Position>,
}

impl Rope {
    pub fn new(knots: usize) -> Self {
        let knots = (0..knots).map(|_| Position::default()).collect();
        let mut tail_positions = HashSet::new();
        tail_positions.insert(Position::default());
        Self {
            knots,
            tail_positions,
        }
    }

    pub fn apply_motions(&mut self, motions: impl Iterator<Item = Motion>) {
        for motion in motions {
            self.apply_motion(motion);
        }
    }

    pub fn apply_motion(&mut self, motion: Motion) {
        for _ in 0..motion.distance {
            self.knots[0] = self.knots[0] + motion.direction;
            for i in 1..self.knots.len() {
                let prev = self.knots[i - 1];
                let knot = &mut self.knots[i];
                let dx = prev.x - knot.x;
                let dy = prev.y - knot.y;
                if dx.abs() > 1 || dy.abs() > 1 {
                    *knot = Position {
                        x: knot.x + dx.signum(),
                        y: knot.y + dy.signum(),
                    };
                } else {
                    break;
                }
            }
            self.tail_positions.insert(*self.knots.last().unwrap());
        }
    }

    pub fn tail_positions(&self) -> usize {
        self.tail_positions.len()
    }
}
