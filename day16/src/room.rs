use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Room {
    pub(super) valve: Valve,
    pub(super) tunnels: Vec<String>,
}

impl Room {
    pub const fn important(&self) -> bool {
        self.valve.flow_rate > 0
    }
}

#[derive(Debug, Clone)]
pub struct Valve {
    pub(super) name: String,
    pub(super) flow_rate: u32,
}

impl FromStr for Room {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (valve, tunnels) = s.split_once("; ").unwrap();
        Ok(Self {
            valve: valve.parse().unwrap(),
            tunnels: tunnels
                .splitn(5, ' ')
                .last()
                .unwrap()
                .split(", ")
                .map(ToString::to_string)
                .collect(),
        })
    }
}

impl FromStr for Valve {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Valve ").unwrap();
        let name = s[0..2].to_string();
        let flow_rate = s[2..]
            .strip_prefix(" has flow rate=")
            .unwrap()
            .parse()
            .unwrap();
        Ok(Self { name, flow_rate })
    }
}
