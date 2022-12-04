use std::str::FromStr;

use crate::FoodItem;

pub struct Elv {
    inventory: Vec<FoodItem>,
}

impl FromStr for Elv {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let inventory = input.lines().map(|line| line.parse().unwrap()).collect();
        Ok(Self { inventory })
    }
}

impl Elv {
    pub fn calories(&self) -> u32 {
        self.inventory.iter().copied().map(FoodItem::calories).sum()
    }
}
