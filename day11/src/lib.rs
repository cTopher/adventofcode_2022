use item::Item;
use item::WorryManagement;
use monkey::Monkey;
use operation::Operation;
use troop::Troop;

mod item;
mod monkey;
mod operation;
mod troop;

#[must_use]
pub fn part_1(input: &str) -> u64 {
    let mut troop: Troop = input.parse().unwrap();
    troop.do_rounds(20);
    troop.monkey_business()
}

#[must_use]
pub fn part_2(input: &str) -> u64 {
    let mut troop: Troop = input.parse().unwrap();
    troop.manage_worries_trough_modulo();
    troop.do_rounds(10000);
    troop.monkey_business()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(10605, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(113_232, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(2_713_310_158, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(29_703_395_016, part_2(INPUT));
    }
}
