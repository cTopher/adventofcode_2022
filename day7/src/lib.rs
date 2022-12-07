extern crate core;

use crate::file::Directory;
use std::str::FromStr;

mod file;

#[must_use]
pub fn part_1(input: &str) -> usize {
    let root = Directory::from_str(input).unwrap();
    root.redundant_size_sum()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let root = Directory::from_str(input).unwrap();
    let space_to_free_up = 30_000_000 + root.size() - 70_000_000;
    root.smallest_directory_greater_than(space_to_free_up)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(95437, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(1_555_642, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(24_933_642, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(5_974_547, part_2(INPUT));
    }
}
