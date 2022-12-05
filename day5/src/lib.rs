mod crane;
mod instruction;
mod ship;

use crane::{Crane, CrateMover9000, CrateMover9001};
use instruction::Move;
use ship::Ship;

fn parse_input(input: &str) -> (Ship, Vec<Move>) {
    let input = input.replace('\r', "");
    let mut split = input.split("\n\n");
    let ship = split.next().unwrap().parse().unwrap();
    let instructions = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    (ship, instructions)
}

#[must_use]
pub fn part_1(input: &str) -> String {
    let (mut ship, instructions) = parse_input(input);
    CrateMover9000::do_instructions(&mut ship, &instructions);
    ship.top_crates().collect()
}

#[must_use]
pub fn part_2(input: &str) -> String {
    let (mut ship, instructions) = parse_input(input);
    CrateMover9001::do_instructions(&mut ship, &instructions);
    ship.top_crates().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!("CMZ", part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!("FJSRQCFTN", part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!("MCD", part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!("CJVLJQPHS", part_2(INPUT));
    }
}
