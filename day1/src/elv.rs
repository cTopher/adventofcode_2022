use std::str::FromStr;

use anyhow::{Error, Result};

use crate::FoodItem;

pub struct Elv {
    inventory: Vec<FoodItem>,
}

impl FromStr for Elv {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let inventory = input
            .lines()
            .map(FoodItem::from_str)
            .collect::<Result<_>>()?;
        Ok(Self { inventory })
    }
}

impl Elv {
    pub fn calories(&self) -> u32 {
        self.inventory.iter().copied().map(FoodItem::calories).sum()
    }
}
