mod blueprint;
mod factory;
mod resource;

use blueprint::Blueprint;
use factory::max_geodes;
use resource::Resources;

fn parse_blueprints(input: &str) -> impl Iterator<Item = Blueprint> + '_ {
    input.lines().map(|line| line.parse().unwrap())
}

#[must_use]
pub fn part_1(input: &str) -> u32 {
    parse_blueprints(input)
        .map(|blueprint| blueprint.id * max_geodes(blueprint, 24))
        .sum()
}

#[must_use]
pub fn part_2(input: &str) -> u32 {
    parse_blueprints(input)
        .take(3)
        .map(|blueprint| max_geodes(blueprint, 32))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(33, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(1834, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(3472, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(2240, part_2(INPUT));
    }
}
