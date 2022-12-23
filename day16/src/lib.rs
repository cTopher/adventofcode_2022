use crate::volcano::Volcano;

mod room;
mod volcano;

#[must_use]
pub fn part_1(input: &str) -> u32 {
    let volcano: Volcano = input.parse().unwrap();
    volcano.max_pressure(30)
}

#[must_use]
pub fn part_2(input: &str) -> u32 {
    let volcano: Volcano = input.parse().unwrap();
    volcano.max_pressure(26)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(1651, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(1828, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(1707, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(0, part_2(INPUT));
    }
}
