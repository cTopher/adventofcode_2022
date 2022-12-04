use std::collections::HashSet;
use std::str::FromStr;

use anyhow::{Error, Result};

use crate::Supply;

#[derive(Clone)]
pub struct Rucksack(pub Compartment, pub Compartment);

#[derive(Clone)]
pub struct Compartment {
    pub supplies: HashSet<Supply>,
}

impl FromStr for Rucksack {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let split = input.len() / 2;
        Ok(Self(input[0..split].parse()?, input[split..].parse()?))
    }
}

impl FromStr for Compartment {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let supplies = input.chars().map(Supply::new).collect();
        Ok(Self { supplies })
    }
}

impl Rucksack {
    pub fn error(&self) -> Supply {
        let Self(first_compartment, second_compartment) = self;
        *first_compartment
            .supplies
            .intersection(&second_compartment.supplies)
            .next()
            .unwrap()
    }
}

#[allow(clippy::implicit_hasher)]
impl From<Rucksack> for HashSet<Supply> {
    fn from(Rucksack(c1, c2): Rucksack) -> Self {
        let mut supplies = c1.supplies;
        supplies.extend(c2.supplies);
        supplies
    }
}
