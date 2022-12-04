mod outcome;
mod shape;

use outcome::Outcome;
use shape::Shape;

use anyhow::{Error, Result};
use std::str::FromStr;

struct EncryptedStrategyGuide {
    rounds: Vec<(char, char)>,
}

impl FromStr for EncryptedStrategyGuide {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let rounds = input
            .trim()
            .lines()
            .map(|line| {
                let mut chars = line.chars();
                (chars.next().unwrap(), chars.nth(1).unwrap()) //TODO no unwrap
            })
            .collect();
        Ok(Self { rounds })
    }
}

impl EncryptedStrategyGuide {
    fn assumed_score(&self) -> u32 {
        self.rounds
            .iter()
            .copied()
            .map(|(opponent, suggestion)| {
                let opponent = Shape::decrypt(opponent);
                let suggestion = Shape::decrypt(suggestion);
                suggestion.score_vs(opponent)
            })
            .sum()
    }

    fn real_score(&self) -> u32 {
        self.rounds
            .iter()
            .copied()
            .map(|(opponent, outcome)| {
                let opponent = Shape::decrypt(opponent);
                let outcome = Outcome::decrypt(outcome);
                let my_move = opponent.counter_that_triggers_outcome(outcome);
                my_move.score() + outcome.score()
            })
            .sum()
    }
}

pub fn part_1(input: &str) -> Result<u32> {
    let guide = EncryptedStrategyGuide::from_str(input)?;
    Ok(guide.assumed_score())
}

pub fn part_2(input: &str) -> Result<u32> {
    let guide = EncryptedStrategyGuide::from_str(input)?;
    Ok(guide.real_score())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() -> Result<()> {
        assert_eq!(15, part_1(EXAMPLE)?);
        Ok(())
    }

    #[test]
    fn part_1_input() -> Result<()> {
        assert_eq!(14375, part_1(INPUT)?);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        assert_eq!(12, part_2(EXAMPLE)?);
        Ok(())
    }

    #[test]
    fn part_2_input() -> Result<()> {
        assert_eq!(10274, part_2(INPUT)?);
        Ok(())
    }
}
