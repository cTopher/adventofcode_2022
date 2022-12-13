mod hill;
mod position;

use hill::Heightmap;
use position::Position;

#[must_use]
pub fn part_1(input: &str) -> usize {
    let heightmap: Heightmap = input.parse().unwrap();
    heightmap.shortest_path()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let heightmap: Heightmap = input.parse().unwrap();
    heightmap.shortest_hike()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(31, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(361, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(29, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(354, part_2(INPUT));
    }
}
