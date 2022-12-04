use std::str::FromStr;

use elv::Elv;
use expedition::Expedition;
use food::FoodItem;

mod elv;
mod expedition;
mod food;

#[must_use]
pub fn part_1(input: &str) -> u32 {
    Expedition::from_str(input).unwrap().max_calories()
}

#[must_use]
pub fn part_2(input: &str) -> u32 {
    Expedition::from_str(input).unwrap().top_3_calories()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(24000, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(69206, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(45000, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(197_400, part_2(INPUT));
    }
}
