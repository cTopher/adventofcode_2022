use std::collections::HashSet;
use std::convert::Infallible;
use std::str::FromStr;

use crate::{Coordinate, Path};

#[derive(Default, Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Material {
    #[default]
    Air,
    Rock,
    Sand,
}

#[derive(Default, Debug)]
pub struct Cave {
    tiles: Vec<Vec<Material>>,
    rock_bottom: usize,
    endless: bool,
}

impl Cave {
    pub const fn new() -> Self {
        Self {
            tiles: Vec::new(),
            rock_bottom: 0,
            endless: true,
        }
    }

    fn material(&self, coordinate: Coordinate) -> Material {
        if !self.endless && coordinate.y >= self.rock_bottom {
            return Material::Rock;
        }
        self.tiles
            .get(coordinate.y)
            .and_then(|row| row.get(coordinate.x))
            .copied()
            .unwrap_or_default()
    }

    pub fn blocked(&self, coordinate: Coordinate) -> bool {
        self.material(coordinate) != Material::Air
    }

    pub const fn rock_bottom(&self) -> usize {
        self.rock_bottom
    }

    pub fn set_floor(&mut self, position: usize) {
        self.endless = false;
        self.rock_bottom = position;
    }

    pub fn flow_sand(&mut self) -> usize {
        let mut count = 0;
        while let Some(coordinate) = self.flow_sand_single() {
            count += 1;
            if coordinate.y == 0 {
                break;
            }
        }
        count
    }

    fn flow_sand_single(&mut self) -> Option<Coordinate> {
        let mut coordinate = Coordinate { x: 500, y: 0 };
        loop {
            if coordinate.y >= self.rock_bottom {
                return None;
            }
            let x = coordinate.x;
            let y = coordinate.y + 1;
            let next = [x, x - 1, x + 1].into_iter().find_map(|x| {
                let coordinate = Coordinate { x, y };
                if self.blocked(coordinate) {
                    None
                } else {
                    Some(coordinate)
                }
            });
            if let Some(next) = next {
                coordinate = next;
            } else {
                self.set_material(coordinate, Material::Sand);
                return Some(coordinate);
            }
        }
    }

    // this needs to be cleaned up but it works rather fast
    pub fn fill_with_sand(&self) -> usize {
        let mut todo = vec![Coordinate { x: 500, y: 0 }];
        let mut sand: HashSet<_> = todo.iter().copied().collect();
        while let Some(coordinate) = todo.pop() {
            let y = coordinate.y + 1;
            for x in coordinate.x - 1..=coordinate.x + 1 {
                let coordinate = Coordinate { x, y };
                if !self.blocked(coordinate) && sand.insert(coordinate) {
                    todo.push(coordinate);
                }
            }
        }
        sand.len()
    }

    fn set_material(&mut self, coordinate: Coordinate, material: Material) {
        for _ in self.tiles.len()..=coordinate.y {
            self.tiles.push(Vec::new());
        }
        let row = &mut self.tiles[coordinate.y];
        for _ in row.len()..=coordinate.x {
            row.push(Material::Air);
        }
        row[coordinate.x] = material;
        if material == Material::Rock {
            self.rock_bottom = self.rock_bottom.max(coordinate.y);
        }
    }
}

impl FromStr for Cave {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cave = Self::new();
        for line in s.trim().lines() {
            let path: Path = line.parse().unwrap();
            for coordinate in path.coordinates() {
                cave.set_material(coordinate, Material::Rock);
            }
        }
        Ok(cave)
    }
}
