mod encryption;

use encryption::File;

#[must_use]
pub fn part_1(input: &str) -> isize {
    let mut file: File = input.parse().unwrap();
    file.decrypt();
    let zero = file.position(0);
    file.get(zero + 1000) + file.get(zero + 2000) + file.get(zero + 3000)
}

#[must_use]
pub fn part_2(input: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(3, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        // not -8747
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
