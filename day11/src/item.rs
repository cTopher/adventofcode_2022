use std::convert::Infallible;
use std::str::FromStr;

pub struct Item {
    pub worry_level: u64,
    pub worry_management: WorryManagement,
}

#[derive(Clone, Copy)]
pub enum WorryManagement {
    Relief,
    Modulo(u64),
}

impl Item {
    pub fn manage_worries(&mut self) {
        match self.worry_management {
            WorryManagement::Relief => self.worry_level /= 3,
            WorryManagement::Modulo(x) => self.worry_level %= x,
        }
    }
}

impl FromStr for Item {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            worry_level: s.parse().unwrap(),
            worry_management: WorryManagement::Relief,
        })
    }
}
