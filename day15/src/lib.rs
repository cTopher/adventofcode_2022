use std::collections::HashSet;

use math::Coordinate;
use sensor::{tuning_frequency, RowCoverage, Sensor, SensorBorders};

mod math;
mod sensor;

fn parse_sensors(input: &str) -> impl Iterator<Item = Sensor> + '_ {
    input.trim().lines().map(|line| line.parse().unwrap())
}

#[must_use]
pub fn part_1(input: &str, y: i64) -> u64 {
    let mut coverage = RowCoverage::new(y);
    for sensor in parse_sensors(input) {
        coverage.add_sensor(&sensor);
    }
    coverage.size()
}

/// this is some fugly shit but oh well... the math behind it is mostly correct though :p
#[must_use]
pub fn part_2(input: &str, max: i64) -> i64 {
    let mut borders = SensorBorders::new();
    let sensors: Vec<_> = parse_sensors(input).collect();
    for sensor in &sensors {
        borders.add_sensor(sensor);
    }
    let SensorBorders {
        downwards_y0,
        upwards_y0,
    } = borders;
    let down: HashSet<_> = downwards_y0
        .into_iter()
        .filter(|(_, amount)| *amount > 1)
        .map(|(y0, _)| y0)
        .collect();
    let up: HashSet<_> = upwards_y0
        .into_iter()
        .filter(|(_, amount)| *amount > 1)
        .map(|(y0, _)| y0)
        .collect();
    let options: Vec<_> = down
        .iter()
        .flat_map(|down| {
            up.iter().map(move |up| {
                let x = (down - up) / 2;
                let y = up + x;
                Coordinate { x, y }
            })
        })
        .filter(|&c| {
            c.x >= 0
                && c.y >= 0
                && c.x <= max
                && c.y <= max
                && sensors.iter().all(|s| !s.can_see(c))
        })
        .collect();
    assert_eq!(options.len(), 1);
    tuning_frequency(*options.first().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_example() {
        assert_eq!(26, part_1(EXAMPLE, 10));
    }

    #[test]
    fn part_1_input() {
        assert_eq!(4_811_413, part_1(INPUT, 2_000_000));
    }

    #[test]
    fn part_2_example() {
        assert_eq!(56_000_011, part_2(EXAMPLE, 20));
    }

    #[test]
    fn part_2_input() {
        assert_eq!(13_171_855_019_123, part_2(INPUT, 4_000_000));
    }
}
