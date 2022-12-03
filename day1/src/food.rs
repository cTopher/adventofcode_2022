use anyhow::{Error, Result};
use std::str::FromStr;

#[derive(Copy, Clone)]
pub struct FoodItem {
    calories: u32,
}

impl FromStr for FoodItem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let calories = input.parse()?;
        Ok(Self { calories })
    }
}

impl FoodItem {
    pub const fn calories(self) -> u32 {
        self.calories
    }
}
