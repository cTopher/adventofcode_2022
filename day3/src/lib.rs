mod rucksack;
mod supply;

use rucksack::Rucksack;
use std::collections::HashSet;
use supply::Supply;

use std::str::FromStr;

use anyhow::Result;

fn parse_rucksacks(input: &str) -> Result<Vec<Rucksack>> {
    input.trim().lines().map(Rucksack::from_str).collect()
}

pub fn part_1(input: &str) -> Result<u32> {
    Ok(parse_rucksacks(input)?
        .iter()
        .map(|rucksack| rucksack.error().priority())
        .sum())
}

pub fn part_2(input: &str) -> Result<u32> {
    Ok(parse_rucksacks(input)?
        .chunks(3)
        .map(|group| {
            let badge = group
                .iter()
                .cloned()
                .map(HashSet::from)
                .reduce(|overlap, sack| overlap.intersection(&sack).copied().collect())
                .unwrap();
            badge.into_iter().next().unwrap().priority()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() -> Result<()> {
        assert_eq!(157, part_1(EXAMPLE)?);
        Ok(())
    }

    #[test]
    fn part_1_input() -> Result<()> {
        assert_eq!(7793, part_1(INPUT)?);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        assert_eq!(70, part_2(EXAMPLE)?);
        Ok(())
    }

    #[test]
    fn part_2_input() -> Result<()> {
        assert_eq!(2499, part_2(INPUT)?);
        Ok(())
    }
}
