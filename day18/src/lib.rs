use crate::model::Droplet;

mod math;
mod model;

#[must_use]
pub fn part_1(input: &str) -> usize {
    let droplet: Droplet = input.parse().unwrap();
    droplet.area()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let mut droplet: Droplet = input.parse().unwrap();
    droplet.fill_with_steam();
    droplet.external_area()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(64, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(3500, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(58, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(2048, part_2(INPUT));
    }
}
