use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Move {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut split = input.split(' ');
        Ok(Self {
            amount: split.nth(1).unwrap().parse().unwrap(),
            from: split.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            to: split.nth(1).unwrap().parse::<usize>().unwrap() - 1,
        })
    }
}
