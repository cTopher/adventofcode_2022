use crate::monkey::{Job, Troop};
use crate::result::Operation;
use crate::result::Res;

mod monkey;
mod result;

#[must_use]
pub fn part_1(input: &str) -> i64 {
    let mut troop: Troop = input.parse().unwrap();
    troop.result("root").unwrap_number()
}

#[must_use]
pub fn part_2(input: &str) -> i64 {
    let mut troop: Troop = input.parse().unwrap();
    troop.insert_result("humn".to_string(), Res::Var);
    let (left, right) = match &troop.monkey("root").job {
        Job::Number(_) => panic!("Unexpected root number"),
        Job::MathOperation { left, right, .. } => (left.clone(), right.clone()),
    };
    Res::solve_equation(troop.result(&left).clone(), troop.result(&right).clone())
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
        assert_eq!(232_974_643_455_000, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(301, part_2(EXAMPLE));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(3_740_214_169_961, part_2(INPUT));
    }
}
