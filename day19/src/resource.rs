use std::convert::Infallible;
use std::ops::{Add, AddAssign, Mul, Sub};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Resources {
    pub ore: u32,
    pub clay: u32,
    pub obsidian: u32,
    pub geode: u32,
}

impl Resources {
    pub const ORE: Self = Self {
        ore: 1,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    pub const CLAY: Self = Self {
        ore: 0,
        clay: 1,
        obsidian: 0,
        geode: 0,
    };
    pub const OBSIDIAN: Self = Self {
        ore: 0,
        clay: 0,
        obsidian: 1,
        geode: 0,
    };
    pub const GEODE: Self = Self {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 1,
    };
    pub const ZERO: Self = Self {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };

    pub const fn saturating_sub(self, other: Self) -> Self {
        Self {
            ore: self.ore.saturating_sub(other.ore),
            clay: self.clay.saturating_sub(other.clay),
            obsidian: self.obsidian.saturating_sub(other.obsidian),
            geode: self.geode.saturating_sub(other.geode),
        }
    }

    pub fn checked_div_ceil(self, rhs: Self) -> Option<u32> {
        let mut result = 0;
        if self.ore > 0 {
            result = result.max(checked_div_ceil(self.ore, rhs.ore)?);
        }
        if self.clay > 0 {
            result = result.max(checked_div_ceil(self.clay, rhs.clay)?);
        }
        if self.obsidian > 0 {
            result = result.max(checked_div_ceil(self.obsidian, rhs.obsidian)?);
        }
        if self.geode > 0 {
            result = result.max(checked_div_ceil(self.geode, rhs.geode)?);
        }
        Some(result)
    }
}

#[inline]
fn checked_div_ceil(lhs: u32, rhs: u32) -> Option<u32> {
    let d = lhs.checked_div(rhs)?;
    let r = lhs % rhs;
    Some(if r > 0 { d + 1 } else { d })
}

impl Mul<Resources> for u32 {
    type Output = Resources;

    fn mul(self, rhs: Resources) -> Self::Output {
        Self::Output {
            ore: self * rhs.ore,
            clay: self * rhs.clay,
            obsidian: self * rhs.obsidian,
            geode: self * rhs.geode,
        }
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl FromStr for Resources {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut resources = Self::ZERO;
        for part in s.split(" and ") {
            match part.split_once(' ') {
                Some((count, resource_type)) => {
                    let amount: u32 = count.parse().unwrap();
                    resources += amount * parse_unary(resource_type);
                }
                None => resources += parse_unary(part),
            }
        }
        Ok(resources)
    }
}

fn parse_unary(s: &str) -> Resources {
    match s {
        "ore" => Resources::ORE,
        "clay" => Resources::CLAY,
        "obsidian" => Resources::OBSIDIAN,
        "geode" => Resources::GEODE,
        _ => panic!("Invalid resource: {s}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_div() {
        assert_eq!(Some(0), Resources::ZERO.checked_div_ceil(Resources::ORE));
        assert_eq!(None, Resources::ORE.checked_div_ceil(Resources::ZERO));
        assert_eq!(
            5,
            Resources {
                ore: 5,
                clay: 3,
                obsidian: 0,
                geode: 0,
            }
            .checked_div_ceil(Resources {
                ore: 1,
                clay: 1,
                obsidian: 1,
                geode: 1,
            })
            .unwrap()
        );
    }
}
