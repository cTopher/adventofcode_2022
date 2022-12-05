use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
pub struct Ship {
    stacks: Vec<Vec<Crate>>,
}

pub type Crate = char;

impl FromStr for Ship {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut ship = Self::new();
        for line in input.lines() {
            for (index, char) in line.char_indices().filter(|(_, char)| char.is_uppercase()) {
                ship.add_crate(index / 4, char);
            }
        }
        for stack in &mut ship.stacks {
            stack.reverse();
        }
        Ok(ship)
    }
}

impl Ship {
    pub const fn new() -> Self {
        let stacks = Vec::new();
        Self { stacks }
    }

    pub fn take_crate(&mut self, stack: usize) -> Crate {
        self.stacks[stack].pop().unwrap()
    }

    pub fn take_crates(&mut self, stack: usize, amount: usize) -> VecDeque<Crate> {
        let mut crates = VecDeque::new();
        let from = &mut self.stacks[stack];
        for _ in 0..amount {
            crates.push_front(from.pop().unwrap());
        }
        crates
    }

    pub fn add_crate(&mut self, stack: usize, crate_: Crate) {
        for _ in self.stacks.len()..=stack {
            self.stacks.push(Vec::new());
        }
        self.stacks[stack].push(crate_);
    }

    pub fn add_crates<I: IntoIterator<Item = Crate>>(&mut self, stack: usize, crates: I) {
        self.stacks[stack].extend(crates);
    }

    pub fn top_crates(&self) -> impl Iterator<Item = &Crate> {
        self.stacks.iter().filter_map(|stack| stack.last())
    }
}
