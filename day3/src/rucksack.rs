use std::collections::HashSet;
use std::str::FromStr;

use crate::Supply;

#[derive(Clone)]
pub struct Rucksack {
    supplies: Vec<Supply>,
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let supplies = input.chars().map(Supply::new).collect();
        Ok(Self { supplies })
    }
}

impl Rucksack {
    pub fn error(&self) -> Supply {
        let split = self.supplies.len() / 2;
        let first_compartment: HashSet<_> = self.supplies.iter().copied().take(split).collect();
        let second_compartment: HashSet<_> = self.supplies.iter().copied().skip(split).collect();
        *first_compartment
            .intersection(&second_compartment)
            .next()
            .unwrap()
    }

    pub fn into_set(self) -> HashSet<Supply> {
        self.supplies.into_iter().collect()
    }
}
