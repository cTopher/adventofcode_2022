use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vertex {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

const ORTHOGONAL: [(isize, isize, isize); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

impl Vertex {
    pub fn neighbours(self) -> impl Iterator<Item = Self> {
        ORTHOGONAL.iter().map(move |&(dx, dy, dz)| Self {
            x: self.x.wrapping_add_signed(dx),
            y: self.y.wrapping_add_signed(dy),
            z: self.z.wrapping_add_signed(dz),
        })
    }
}

impl FromStr for Vertex {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',').map(|s| s.parse().unwrap());
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        let z = parts.next().unwrap();
        Ok(Self { x, y, z })
    }
}
