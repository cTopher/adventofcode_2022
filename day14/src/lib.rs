mod cave;
mod path;

use cave::Cave;
use path::Coordinate;
use path::Path;

#[must_use]
pub fn part_1(input: &str) -> usize {
    let mut cave: Cave = input.parse().unwrap();
    cave.flow_sand()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let mut cave: Cave = input.parse().unwrap();
    cave.set_floor(cave.rock_bottom() + 2);
    cave.fill_with_sand()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(24, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(737, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(93, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(28145, part_2(INPUT));
    }
}
