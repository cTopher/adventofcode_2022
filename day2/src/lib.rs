mod outcome;
mod shape;

use outcome::Outcome;
use shape::Shape;

use std::str::FromStr;

struct EncryptedStrategyGuide {
    rounds: Vec<(char, char)>,
}

impl FromStr for EncryptedStrategyGuide {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
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

#[must_use]
pub fn part_1(input: &str) -> u32 {
    EncryptedStrategyGuide::from_str(input)
        .unwrap()
        .assumed_score()
}

#[must_use]
pub fn part_2(input: &str) -> u32 {
    EncryptedStrategyGuide::from_str(input)
        .unwrap()
        .real_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(15, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(14375, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(12, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(10274, part_2(INPUT));
    }
}
