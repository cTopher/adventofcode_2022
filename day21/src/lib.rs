use crate::monkey::Monkeys;

mod monkey;

#[must_use]
pub fn part_1(input: &str) -> i64 {
    let monkeys: Monkeys = input.parse().unwrap();
    monkeys.result("root")
}

#[must_use]
pub fn part_2(input: &str) -> i64 {
    let monkeys: Monkeys = input.parse().unwrap();
    monkeys.result("root")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(152, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(232974643455000, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(0, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(0, part_2(INPUT));
    }
}
