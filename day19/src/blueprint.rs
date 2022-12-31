use crate::Resources;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Blueprint {
    pub id: u32,
    pub robots: [Robot; 4],
    pub max_cost: Resources,
}

impl Blueprint {
    pub fn new(id: u32, robots: [Robot; 4]) -> Self {
        Self {
            id,
            robots,
            max_cost: robots
                .iter()
                .map(|robot| robot.cost)
                .reduce(|a, b| Resources {
                    ore: a.ore.max(b.ore),
                    clay: a.clay.max(b.clay),
                    obsidian: a.obsidian.max(b.obsidian),
                    geode: a.geode.max(b.geode),
                })
                .unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Robot {
    pub cost: Resources,
    pub collection: Resources,
}

impl FromStr for Robot {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, body) = s.trim().split_once(" costs ").unwrap();
        let collection = head
            .strip_prefix("Each ")
            .unwrap()
            .strip_suffix(" robot")
            .unwrap()
            .parse()
            .unwrap();
        let cost = body.parse().unwrap();
        Ok(Self { cost, collection })
    }
}

impl FromStr for Blueprint {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, body) = s.split_once(':').unwrap();
        let id = head.strip_prefix("Blueprint ").unwrap().parse().unwrap();
        let robots: Vec<_> = body
            .strip_suffix('.')
            .unwrap()
            .split('.')
            .map(|s| s.parse().unwrap())
            .collect();
        let robots = robots.try_into().unwrap();
        Ok(Self::new(id, robots))
    }
}
