use std::str::FromStr;

pub enum Instruction {
    AddX(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input.strip_prefix("addx ").map_or(Ok(Self::Noop), |value| {
            Ok(Self::AddX(value.parse().unwrap()))
        })
    }
}
