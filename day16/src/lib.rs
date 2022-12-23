use crate::volcano::Volcano;

mod room;
mod volcano;

#[must_use]
pub fn part_1(input: &str) -> u32 {
    let volcano: Volcano = input.parse().unwrap();
    volcano.pressures(30).into_iter().max().unwrap()
}

#[must_use]
pub fn part_2(input: &str) -> u32 {
    let volcano: Volcano = input.parse().unwrap();
    let pressures: Vec<_> = volcano.pressures(26).into_iter().enumerate().filter(|(_, pressure)| *pressure > 0).collect();
    pressures
        .iter()
        .filter(|(i1,_)| i1 % 2 == 0)
        .flat_map(|&(i1, p1)| {
            pressures
                .iter()
                .filter_map(move |&(i2, p2)| if i1 & i2 == 0 {
                    Some(p1 + p2)
                } else {
                    None
                })
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(1651, part_1(EXAMPLE));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(1828, part_1(INPUT));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(1707, part_2(EXAMPLE));
    }

    // 4297 10034 14331 18436
    // 4301 10034 14335 18432
    // 10034 4297 14331 18436
    // 10034 4301 14335 18432
    #[test]
    fn part_2_input() {
        assert_eq!(2292, part_2(INPUT));
    }
}
