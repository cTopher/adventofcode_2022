use packet::Packet;

mod packet;

const SEPARATOR: &str = "\n\n";

fn parse_pair(s: &str) -> (Packet, Packet) {
    let mut pair = s.lines().map(|line| line.parse().unwrap());
    (pair.next().unwrap(), pair.next().unwrap())
}

#[must_use]
pub fn part_1(input: &str) -> usize {
    input
        .split(SEPARATOR)
        .map(parse_pair)
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(index, _)| index + 1)
        .sum()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let packets: Vec<Packet> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    let divider_1 = packets
        .iter()
        .filter(|&packet| packet < &Packet::Integer(2))
        .count()
        + 1;
    let divider_2 = packets
        .iter()
        .filter(|&packet| packet < &Packet::Integer(6))
        .count()
        + 2;
    divider_1 * divider_2
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(13, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(6070, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(140, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(20758, part_2(INPUT));
    }
}
