use std::str::FromStr;

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub struct Motion {
    pub direction: Direction,
    pub distance: u8,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => panic!("Invalid direction: {}", input),
        }
    }
}

impl FromStr for Motion {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = input.split_once(' ').unwrap();
        let direction = direction.parse().unwrap();
        let distance = distance.parse().unwrap();
        Ok(Self {
            direction,
            distance,
        })
    }
}
