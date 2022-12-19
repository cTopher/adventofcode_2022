use crate::Item;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    pub fn apply(self, item: &mut Item) {
        match self {
            Self::Add(value) => item.worry_level += value,
            Self::Multiply(value) => item.worry_level *= value,
            Self::Square => item.worry_level *= item.worry_level,
        }
    }
}

impl FromStr for Operation {
    type Err = Infallible;

    #[allow(clippy::option_if_let_else)]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(value) = input.strip_prefix("new = old + ") {
            Self::Add(value.parse().unwrap())
        } else if let Some(value) = input.strip_prefix("new = old * ") {
            if value == "old" {
                Self::Square
            } else {
                Self::Multiply(value.parse().unwrap())
            }
        } else {
            panic!("Unknown operation: {input}");
        })
    }
}
