mod assignment;

use assignment::SectionAssignment;

fn parse_pairs(input: &str) -> impl Iterator<Item = (SectionAssignment, SectionAssignment)> + '_ {
    input.trim().lines().map(|line| {
        let mut split = line.split(',').map(|x| x.parse().unwrap());
        (split.next().unwrap(), split.next().unwrap())
    })
}

#[must_use]
pub fn part_1(input: &str) -> usize {
    parse_pairs(input)
        .filter(|&(a, b)| a.fully_overlaps_with(b))
        .count()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    parse_pairs(input).filter(|&(a, b)| a.overlaps(b)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(2, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(450, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(4, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(837, part_2(INPUT));
    }
}
