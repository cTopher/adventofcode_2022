use motion::{Direction, Motion};
use rope::Rope;

mod motion;
mod rope;

fn parse_motions(input: &str) -> impl Iterator<Item = Motion> + '_ {
    input.trim().lines().map(|line| line.parse().unwrap())
}

#[must_use]
pub fn part_1(input: &str) -> usize {
    let mut rope = Rope::new(2);
    rope.apply_motions(parse_motions(input));
    rope.tail_positions()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let mut rope = Rope::new(10);
    rope.apply_motions(parse_motions(input));
    rope.tail_positions()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = include_str!("../example-1.txt");
    const EXAMPLE_2: &str = include_str!("../example-2.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(13, part_1(EXAMPLE_1));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(5902, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(36, part_2(EXAMPLE_2));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(2445, part_2(INPUT));
    }
}
