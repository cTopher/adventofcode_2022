mod tree;

use crate::tree::Tree;
use std::str::FromStr;
use tree::TreeGrid;

#[must_use]
pub fn part_1(input: &str) -> usize {
    TreeGrid::from_str(input)
        .unwrap()
        .trees()
        .filter(Tree::visible)
        .count()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    TreeGrid::from_str(input)
        .unwrap()
        .trees()
        .map(|tree| tree.scenic_score())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(21, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(1662, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(8, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(537_600, part_2(INPUT));
    }
}
