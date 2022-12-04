mod rucksack;
mod supply;

use rucksack::Rucksack;
use std::collections::HashSet;
use supply::Supply;

fn parse_rucksacks(input: &str) -> impl Iterator<Item = Rucksack> + '_ {
    input.trim().lines().map(|line| line.parse().unwrap())
}

#[must_use]
pub fn part_1(input: &str) -> u32 {
    parse_rucksacks(input)
        .map(|rucksack| rucksack.error().priority())
        .sum()
}

#[must_use]
pub fn part_2(input: &str) -> u32 {
    let mut sum = 0;
    let mut rucksacks = parse_rucksacks(input);
    loop {
        let first = match rucksacks.next() {
            None => return sum,
            Some(sack) => HashSet::from(sack),
        };
        let second = rucksacks.next().unwrap().into();
        let third = rucksacks.next().unwrap().into();
        let badge = first
            .intersection(&second)
            .copied()
            .collect::<HashSet<_>>()
            .intersection(&third)
            .next()
            .unwrap()
            .priority();
        sum += badge;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(157, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(7793, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(70, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(2499, part_2(INPUT));
    }
}
