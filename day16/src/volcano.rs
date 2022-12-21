use std::collections::HashSet;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Volcano {
    rooms: Vec<Room>,
}

#[derive(Debug, Clone)]
pub struct Room {
    valve: Valve,
    distances: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct Valve {
    name: String,
    flow_rate: u32,
}

impl Volcano {
    fn new(valves: Vec<Valve>, tunnels: Vec<Vec<String>>) -> Self {
        let len = valves.len();
        let rooms = valves
            .into_iter()
            .map(|valve| Room {
                valve,
                distances: vec![u32::MAX; len],
            })
            .collect();
        let mut result = Self { rooms };
        result.calculate_distances(tunnels);
        result
    }

    fn index(&self, name: &str) -> usize {
        self.rooms
            .iter()
            .position(|room| room.valve.name == name)
            .unwrap()
    }

    fn calculate_distances(&mut self, tunnels: Vec<Vec<String>>) {
        let tunnels: Vec<Vec<_>> = tunnels
            .into_iter()
            .map(|tunnel| tunnel.into_iter().map(|name| self.index(&name)).collect())
            .collect();
        for (index, tunnels) in tunnels.iter().enumerate() {
            let distances = &mut self.rooms[index].distances;
            distances[index] = 0;
            for &tunnel in tunnels {
                distances[tunnel] = 1;
            }
        }
        for k in 0..self.rooms.len() {
            for i in 0..self.rooms.len() {
                for j in 0..self.rooms.len() {
                    let distance = self.rooms[i].distances[k].saturating_add(self.rooms[k].distances[j]);
                    self.rooms[i].distances[j] = self.rooms[i].distances[j].min(distance);
                }
            }
        }
    }

    pub fn max_pressure(&self, duration: u32) -> u32 {
        let mut max = 0;
        let mut paths = vec![self.start(duration)];
        while let Some(path) = paths.pop() {
            if path.pressure > max {
                max = path.pressure;
            }
            for &index in &path.interesting_valves {
                let distance = self.rooms[path.position].distances[index];
                if distance < path.remaining_duration {
                    let mut path = path.clone();
                    path.remaining_duration -= distance + 1;
                    path.pressure += self.rooms[index].valve.flow_rate * path.remaining_duration;
                    path.interesting_valves.remove(&index);
                    path.position = index;
                    paths.push(path);
                }
            }
        }
        max
    }

    fn start(&self, duration: u32) -> Path {
        Path {
            pressure: 0,
            position: self.rooms.iter().position(|room| room.valve.name == "AA").unwrap(),
            interesting_valves: self.rooms.iter().enumerate().filter_map(|(index, room)| {
                if room.valve.flow_rate > 0 {
                    Some(index)
                } else {
                    None
                }
            }).collect(),
            remaining_duration: duration,
        }
    }
}

#[derive(Debug, Clone)]
struct Path {
    pressure: u32,
    position: usize,
    interesting_valves: HashSet<usize>,
    remaining_duration: u32,
}

impl FromStr for Volcano {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut valves = Vec::new();
        let mut tunnels = Vec::new();
        for line in s.lines() {
            let (valve_str, tunnels_str) = line.split_once("; ").unwrap();
            valves.push(valve_str.parse().unwrap());
            tunnels.push(
                tunnels_str
                    .splitn(5, " ")
                    .last()
                    .unwrap()
                    .split(", ")
                    .map(ToString::to_string)
                    .collect(),
            );
        }
        Ok(Self::new(valves, tunnels))
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
