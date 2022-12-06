use std::collections::HashSet;

fn find_marker(input: &str, start_marker_size: usize) -> usize {
    input
        .as_bytes()
        .windows(start_marker_size)
        .position(all_different)
        .unwrap()
        + start_marker_size
}

fn all_different(slice: &[u8]) -> bool {
    slice.len() == slice.iter().copied().collect::<HashSet<_>>().len()
}

#[must_use]
pub fn part_1(input: &str) -> usize {
    find_marker(input, 4)
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    find_marker(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_examples() {
        assert_eq!(5, part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, part_1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(1702, part_1(INPUT));
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(5, part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, part_1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(3559, part_2(INPUT));
    }
}
