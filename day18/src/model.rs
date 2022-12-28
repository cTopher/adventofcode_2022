use crate::math::Vertex;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Material {
    Air,
    Steam,
    Lava,
}

#[derive(Debug)]
pub struct Droplet {
    vertices: Vec<Vec<Vec<Material>>>,
}

impl Droplet {
    const fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    fn add_lava(&mut self, Vertex { x, y, z }: Vertex) {
        for _ in self.vertices.len()..=x {
            self.vertices.push(Vec::new());
        }
        let plane = &mut self.vertices[x];
        for _ in plane.len()..=y {
            plane.push(Vec::new());
        }
        let row = &mut plane[y];
        for _ in row.len()..=z {
            row.push(Material::Air);
        }
        row[z] = Material::Lava;
    }

    pub fn fill_with_steam(&mut self) {
        let mut queue: Vec<_> = self
            .get_all(Material::Air)
            .filter(|vertex| {
                vertex
                    .neighbours()
                    .any(|neighbour| self.get(neighbour) == Material::Steam)
            })
            .collect();
        for &Vertex { x, y, z } in &queue {
            self.vertices[x][y][z] = Material::Steam;
        }
        while let Some(v) = queue.pop() {
            for neighbour in v.neighbours() {
                if self.get(neighbour) == Material::Air {
                    self.vertices[neighbour.x][neighbour.y][neighbour.z] = Material::Steam;
                    queue.push(neighbour);
                }
            }
        }
    }

    pub fn area(&self) -> usize {
        self.get_all(Material::Lava)
            .map(|v| {
                v.neighbours()
                    .filter(|n| self.get(*n) != Material::Lava)
                    .count()
            })
            .sum()
    }

    pub fn external_area(&self) -> usize {
        self.get_all(Material::Lava)
            .map(|v| {
                v.neighbours()
                    .filter(|n| self.get(*n) == Material::Steam)
                    .count()
            })
            .sum()
    }

    fn get_all(&self, material: Material) -> impl Iterator<Item = Vertex> + '_ {
        self.vertices
            .iter()
            .enumerate()
            .flat_map(move |(x, plane)| {
                plane.iter().enumerate().flat_map(move |(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(move |(z, m)| (Vertex { x, y, z }, *m))
                })
            })
            .filter(move |(_, m)| *m == material)
            .map(|(v, _)| v)
    }

    fn get(&self, Vertex { x, y, z }: Vertex) -> Material {
        self.vertices
            .get(x)
            .and_then(|plane| plane.get(y))
            .and_then(|row| row.get(z))
            .copied()
            .unwrap_or(Material::Steam)
    }
}

impl FromStr for Droplet {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut droplet = Self::new();
        for line in s.trim().lines() {
            let vertex = line.parse().unwrap();
            droplet.add_lava(vertex);
        }
        Ok(droplet)
    }
}
