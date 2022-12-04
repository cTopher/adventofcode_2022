#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    pub fn decrypt(input: char) -> Self {
        match input {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("illegal Outcome code: {input}"),
        }
    }

    pub const fn score(self) -> u32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}
