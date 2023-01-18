mod encryption;

use encryption::File;

#[must_use]
pub fn part_1(input: &str) -> i64 {
    let file: File = input.parse().unwrap();
    file.mix();
    file.get(1000) + file.get(2000) + file.get(3000)
}

#[must_use]
pub fn part_2(input: &str) -> i64 {
    let file: File = input.parse().unwrap();
    file.decrypt(811_589_153);
    file.get(1000) + file.get(2000) + file.get(3000)
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
        assert_eq!(8764, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(1_623_178_306, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(535_648_840_980, part_2(INPUT));
    }
}
