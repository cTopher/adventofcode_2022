use crate::room::Room;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Volcano {
    valves: Vec<ImportantValve>,
    start_distances: Vec<u32>,
}

#[derive(Debug, Clone)]
struct ImportantValve {
    flow_rate: u32,
    distances: Vec<u32>,
}

impl FromStr for Volcano {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rooms: Vec<_> = s.lines().map(|line| line.parse().unwrap()).collect();
        Ok(Self::new(&rooms))
    }
}

fn calculate_distances(rooms: &[Room]) -> Vec<Vec<u32>> {
    let mut distances = vec![vec![u32::MAX; rooms.len()]; rooms.len()];
    for (index, room) in rooms.iter().enumerate() {
        for tunnel in &room.tunnels {
            distances[index][index] = 0;
            distances[index][rooms.iter().position(|r| &r.valve.name == tunnel).unwrap()] = 1;
        }
    }
    for k in 0..rooms.len() {
        for i in 0..rooms.len() {
            for j in 0..rooms.len() {
                distances[i][j] = distances[i][k]
                    .saturating_add(distances[k][j])
                    .min(distances[i][j]);
            }
        }
    }
    distances
}

impl Volcano {
    fn new(rooms: &[Room]) -> Self {
        let mut distances = calculate_distances(rooms);
        for distances in &mut distances {
            *distances = distances
                .iter()
                .enumerate()
                .filter(|(index, _)| rooms[*index].important())
                .map(|(_, distance)| *distance)
                .collect();
        }
        let valves = rooms
            .iter()
            .enumerate()
            .filter(|(_, room)| room.important())
            .map(|(index, room)| ImportantValve {
                flow_rate: room.valve.flow_rate,
                distances: distances[index].clone(),
            })
            .collect();
        let start_distances = rooms
            .iter()
            .position(|room| room.valve.name == "AA")
            .map(|index| distances[index].clone())
            .unwrap();
        Self {
            valves,
            start_distances,
        }
    }

    pub fn pressures(&self, duration: u32) -> Vec<u32> {
        let mut pressure = vec![0; 2usize.pow(u32::try_from(self.start_distances.len()).unwrap())];
        let mut paths = self.start_paths(duration);
        while let Some(path) = paths.pop() {
            if path.pressure > pressure[path.valves] {
                pressure[path.valves] = path.pressure;
            }
            for (position, valve) in self.valves.iter().enumerate() {
                if path.valves & 1 << position == 0 {
                    let distance = self.valves[path.position].distances[position];
                    if distance < path.remaining_duration {
                        let remaining_duration = path.remaining_duration - distance - 1;
                        paths.push(Path {
                            valves: path.valves | 1 << position,
                            pressure: path.pressure + valve.flow_rate * remaining_duration,
                            remaining_duration,
                            position,
                        });
                    }
                }
            }
        }
        pressure
    }

    fn start_paths(&self, duration: u32) -> Vec<Path> {
        self.valves
            .iter()
            .enumerate()
            .map(|(index, valve)| {
                let distance = self.start_distances[index];
                let remaining_duration = duration - distance - 1;
                Path {
                    pressure: valve.flow_rate * remaining_duration,
                    position: index,
                    valves: 1 << index,
                    remaining_duration,
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
struct Path {
    pressure: u32,
    position: usize,
    valves: usize,
    remaining_duration: u32,
}
