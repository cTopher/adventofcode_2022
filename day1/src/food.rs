use std::str::FromStr;

#[derive(Copy, Clone)]
pub struct FoodItem {
    calories: u32,
}

impl FromStr for FoodItem {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let calories = input.parse().unwrap();
        Ok(Self { calories })
    }
}

impl FoodItem {
    pub const fn calories(self) -> u32 {
        self.calories
    }
}
