mod math;
mod model;

use math::{Direction, Position};
use model::{Chamber, Jet};

#[must_use]
pub fn part_1(input: &str) -> usize {
    let jet: Jet = input.parse().unwrap();
    let mut chamber = Chamber::new(jet);
    chamber.drop_rocks(2022);
    chamber.height
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let jet: Jet = input.parse().unwrap();
    let mut chamber = Chamber::new(jet);
    chamber.drop_rocks(1_000_000_000_000);
    chamber.height
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(3068, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(3100, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(1_514_285_714_288, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(1_540_634_005_751, part_2(INPUT));
    }
}
