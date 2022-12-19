use std::convert::Infallible;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
}

impl Coordinate {
    pub const fn distance_to(self, other: Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl FromStr for Coordinate {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(", ").unwrap();
        Ok(Self {
            x: x.strip_prefix("x=").unwrap().parse().unwrap(),
            y: y.strip_prefix("y=").unwrap().parse().unwrap(),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Range {
    pub(crate) min: i64,
    pub(crate) max: i64,
}

impl Range {
    pub fn union(self, other: Self) -> Option<Self> {
        if self.max < other.min || self.min > other.max {
            None
        } else {
            Some(Self {
                min: self.min.min(other.min),
                max: self.max.max(other.max),
            })
        }
    }

    pub const fn size(self) -> u64 {
        self.max.abs_diff(self.min) + 1
    }
}
