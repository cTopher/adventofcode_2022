use crate::Outcome;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn decrypt(input: char) -> Self {
        match input {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("illegal Shape code: {input}"),
        }
    }

    pub const fn score(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub const fn counter(self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub const fn counter_that_triggers_outcome(self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => self.counter(),
            Outcome::Draw => self,
            Outcome::Loss => match self {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
        }
    }

    pub fn outcome_vs(self, opponent: Self) -> Outcome {
        if self == opponent {
            Outcome::Draw
        } else if self.counter() == opponent {
            Outcome::Loss
        } else {
            assert_eq!(self, opponent.counter());
            Outcome::Win
        }
    }

    pub fn score_vs(self, opponent: Self) -> u32 {
        self.score() + self.outcome_vs(opponent).score()
    }
}
