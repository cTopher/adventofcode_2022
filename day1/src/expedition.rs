use std::str::FromStr;

use crate::Elv;

pub struct Expedition {
    elves: Vec<Elv>,
}

impl FromStr for Expedition {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let elves = input
            .trim()
            .replace('\r', "")
            .split("\n\n")
            .map(|x| x.parse().unwrap())
            .collect();
        Ok(Self { elves })
    }
}

impl Expedition {
    pub fn max_calories(&self) -> u32 {
        self.elves.iter().map(Elv::calories).max().unwrap()
    }

    pub fn top_3_calories(&self) -> u32 {
        let mut calories: Vec<_> = self.elves.iter().map(Elv::calories).collect();
        calories.sort_unstable();
        calories.iter().rev().take(3).sum()
    }
}
