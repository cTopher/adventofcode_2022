use std::convert::Infallible;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl FromStr for Coordinate {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Path(Vec<Coordinate>);

impl Path {
    pub fn coordinates(&self) -> impl Iterator<Item = Coordinate> + '_ {
        self.0.windows(2).flat_map(|window| {
            if let &[a, b] = window {
                ord_range(a.x, b.x)
                    .flat_map(move |x| ord_range(a.y, b.y).map(move |y| Coordinate { x, y }))
            } else {
                unreachable!();
            }
        })
    }
}

const fn ord_range(a: usize, b: usize) -> RangeInclusive<usize> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

impl FromStr for Path {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split(" -> ").map(|s| s.parse().unwrap()).collect()))
    }
}
