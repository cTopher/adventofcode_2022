use crate::Direction;
use crate::Position;
use std::convert::Infallible;
use std::str::FromStr;

pub struct Chamber {
    rock: Vec<[bool; 7]>,
    jet: Jet,
    rock_index: u8,
    dropped: usize,
    pub(super) height: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Cycle {
    start_dropped: usize,
    start_height: usize,
}

impl Chamber {
    #[must_use]
    pub const fn new(jet: Jet) -> Self {
        Self {
            rock: Vec::new(),
            jet,
            rock_index: 0,
            height: 0,
            dropped: 0,
        }
    }

    pub fn drop_rocks(&mut self, amount: usize) {
        let mut cycle = vec![Cycle::default(); self.jet.pattern.len()];
        while self.dropped < amount {
            self.drop_rock();
            if self.rock_index == 0 {
                let entry = &mut cycle[self.jet.index];
                if entry.start_dropped == 0 {
                    entry.start_dropped = self.dropped;
                    entry.start_height = self.height;
                } else {
                    let cycle_drop = self.dropped - entry.start_dropped;
                    let cycle_height = self.height - entry.start_height;
                    let cycles = (amount - self.dropped) / cycle_drop;
                    self.dropped += cycles * cycle_drop;
                    self.height += cycles * cycle_height;
                    break;
                }
            }
        }
        while self.dropped < amount {
            self.drop_rock();
        }
    }

    pub fn drop_rock(&mut self) {
        let mut rock = self.new_rock();
        loop {
            self.blow(&mut rock);
            if !self.drop(&mut rock) {
                break;
            }
        }
        self.add(&rock);
        self.dropped += 1;
    }

    fn blow(&mut self, rock: &mut Rock) {
        let prev = rock.position;
        if rock.transpose(self.jet.next()) && !self.fits(rock) {
            rock.position = prev;
        }
    }

    fn drop(&mut self, rock: &mut Rock) -> bool {
        let prev = rock.position;
        if !rock.transpose(Direction::Down) {
            false
        } else if !self.fits(rock) {
            rock.position = prev;
            false
        } else {
            true
        }
    }

    fn add(&mut self, rock: &Rock) {
        for Position { x, y } in rock.rocks() {
            for _ in self.rock.len()..=y {
                self.rock.push([false; 7]);
                self.height += 1;
            }
            self.rock[y][x] = true;
        }
    }

    fn fits(&self, rock: &Rock) -> bool {
        rock.rocks().all(|pos| self.is_empty(pos))
    }

    fn is_empty(&self, Position { x, y }: Position) -> bool {
        x < 7
            && self
                .rock
                .get(y)
                .map_or(true, |row| row.get(x).map_or(false, |&rock| !rock))
    }

    fn new_rock(&mut self) -> Rock {
        let position = Position::new(2, self.rock.len() + 3);
        let rock = Rock::new(position, self.rock_index);
        self.rock_index = (self.rock_index + 1) % 5;
        rock
    }
}

struct Rock {
    position: Position,
    shape: Vec<Position>,
}

impl Rock {
    fn rocks(&self) -> impl Iterator<Item = Position> + '_ {
        self.shape.iter().map(|&pos| pos + self.position)
    }

    fn new(position: Position, shape: u8) -> Self {
        let shape = match shape {
            0 => vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(2, 0),
                Position::new(3, 0),
            ],
            1 => vec![
                Position::new(1, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
                Position::new(1, 2),
            ],
            2 => vec![
                Position::new(0, 0),
                Position::new(1, 0),
                Position::new(2, 0),
                Position::new(2, 1),
                Position::new(2, 2),
            ],
            3 => vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
                Position::new(0, 3),
            ],
            4 => vec![
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(1, 0),
                Position::new(1, 1),
            ],
            _ => panic!("Invalid shape index: {shape}"),
        };
        Self { position, shape }
    }

    fn transpose(&mut self, direction: Direction) -> bool {
        if let Some(position) = self.position + direction {
            self.position = position;
            true
        } else {
            false
        }
    }
}

pub struct Jet {
    index: usize,
    pattern: Vec<Direction>,
}

impl Jet {
    fn next(&mut self) -> Direction {
        let direction = self.pattern[self.index];
        self.index = (self.index + 1) % self.pattern.len();
        direction
    }
}

impl FromStr for Jet {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            index: 0,
            pattern: s.trim().chars().map(Direction::from).collect(),
        })
    }
}
