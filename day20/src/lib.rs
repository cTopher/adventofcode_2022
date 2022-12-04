fn parse_(input: &str) -> impl Iterator<Item = &str> + '_ {
    input.trim().lines()
}

#[must_use]
pub fn part_1(input: &str) -> usize {
    parse_(input).count()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    parse_(input).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(0, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(0, part_1(INPUT));
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
