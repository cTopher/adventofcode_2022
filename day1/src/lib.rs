use std::str::FromStr;

use anyhow::Result;

use elv::Elv;
use expedition::Expedition;
use food::FoodItem;

mod elv;
mod expedition;
mod food;

pub fn part_1(input: &str) -> Result<u32> {
    let expedition = Expedition::from_str(input)?;
    Ok(expedition.max_calories())
}

pub fn part_2(input: &str) -> Result<u32> {
    let expedition = Expedition::from_str(input)?;
    Ok(expedition.top_3_calories())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() -> Result<()> {
        assert_eq!(24000, part_1(EXAMPLE)?);
        Ok(())
    }

    #[test]
    fn part_1_input() -> Result<()> {
        assert_eq!(69206, part_1(INPUT)?);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        assert_eq!(45000, part_2(EXAMPLE)?);
        Ok(())
    }

    #[test]
    fn part_2_input() -> Result<()> {
        assert_eq!(197_400, part_2(INPUT)?);
        Ok(())
    }
}
