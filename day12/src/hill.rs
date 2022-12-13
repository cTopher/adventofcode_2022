use std::cmp::{max, Ordering};
use std::collections::BinaryHeap;
use std::convert::Infallible;
use std::iter;
use std::str::FromStr;

use crate::Position;

#[derive(Debug)]
pub struct Heightmap {
    grid: Vec<Vec<u8>>,
    start: Position,
    end: Position,
}

impl FromStr for Heightmap {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Vec<Vec<u8>> = s
            .trim()
            .lines()
            .map(|line| line.as_bytes().into())
            .collect();
        let find = |char| {
            grid.iter()
                .enumerate()
                .find_map(|(y, row)| {
                    row.iter().enumerate().find_map(|(x, &c)| {
                        if c == char {
                            Some(Position { x, y })
                        } else {
                            None
                        }
                    })
                })
                .unwrap()
        };
        let start = find(b'S');
        let end = find(b'E');
        grid[start.y][start.x] = b'a';
        grid[end.y][end.x] = b'z';
        Ok(Self { grid, start, end })
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Path {
    pub position: Position,
    pub distance: usize,
    pub cost: usize,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then(other.distance.cmp(&self.distance))
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Heightmap {
    fn cost(&self, position: &Position, distance: usize) -> usize {
        let remaining_distance = position.distance(self.end);
        let remaining_climb = b'z'.saturating_sub(self.height(position).unwrap());
        distance + max(remaining_distance, remaining_climb as usize)
    }

    fn height(&self, position: &Position) -> Option<u8> {
        self.grid.get(position.y)?.get(position.x).copied()
    }

    fn as_path(&self, position: Position, distance: usize) -> Path {
        let cost = self.cost(&position, distance);
        Path {
            position,
            distance,
            cost,
        }
    }

    pub fn shortest_path(&self) -> usize {
        self.shortest_path_from(iter::once(self.start))
    }

    pub fn shortest_hike(&self) -> usize {
        let positions = self.grid.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &c)| {
                if c == b'a' {
                    Some(Position { x, y })
                } else {
                    None
                }
            })
        });
        self.shortest_path_from(positions)
    }

    fn shortest_path_from(&self, start: impl Iterator<Item = Position>) -> usize {
        let mut heap = BinaryHeap::new();
        let mut distances: Vec<_> = self
            .grid
            .iter()
            .map(|row| vec![usize::MAX; row.len()])
            .collect();
        for start in start {
            heap.push(self.as_path(start, 0));
            distances[start.y][start.x] = 0;
        }
        while let Some(path) = heap.pop() {
            if path.position == self.end {
                return path.distance;
            }
            if path.distance > distances[path.position.y][path.position.x] {
                continue;
            }
            let height = self.height(&path.position).unwrap();
            for neighbour in path.position.neighbors() {
                if let Some(neighbour_height) = self.height(&neighbour) {
                    if neighbour_height <= height + 1 {
                        let path = self.as_path(neighbour, path.distance + 1);
                        if path.distance < distances[neighbour.y][neighbour.x] {
                            distances[neighbour.y][neighbour.x] = path.distance;
                            heap.push(path);
                        }
                    }
                }
            }
        }
        unreachable!("no path found");
    }
}
