use crate::math::Range;
use crate::Coordinate;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::str::FromStr;

pub struct Sensor {
    position: Coordinate,
    beacon: Coordinate,
    range: u64,
}

impl Sensor {
    const fn new(position: Coordinate, beacon: Coordinate) -> Self {
        Self {
            position,
            beacon,
            range: position.distance_to(beacon),
        }
    }

    pub const fn border_contains(&self, coordinate: Coordinate) -> bool {
        self.position.distance_to(coordinate) == self.range + 1
    }
}

impl FromStr for Sensor {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, beacon) = s.split_once(':').unwrap();
        let position = position
            .strip_prefix("Sensor at ")
            .unwrap()
            .parse()
            .unwrap();
        let beacon = beacon
            .strip_prefix(" closest beacon is at ")
            .unwrap()
            .parse()
            .unwrap();
        Ok(Self::new(position, beacon))
    }
}

pub struct RowCoverage {
    ranges: HashSet<Range>,
    y: i64,
    beacons: HashSet<Coordinate>,
}

impl RowCoverage {
    pub fn new(y: i64) -> Self {
        Self {
            ranges: HashSet::new(),
            y,
            beacons: HashSet::new(),
        }
    }

    pub fn add_sensor(&mut self, sensor: &Sensor) {
        let x = sensor.position.x;
        if let Some(d) = sensor.range.checked_sub(sensor.position.y.abs_diff(self.y)) {
            self.add_range(Range {
                min: x.checked_sub_unsigned(d).unwrap(),
                max: x.checked_add_unsigned(d).unwrap(),
            });
            if sensor.beacon.y == self.y {
                self.beacons.insert(sensor.beacon);
            }
        }
    }

    fn add_range(&mut self, mut range: Range) {
        while let Some((intersection, union)) = self
            .ranges
            .iter()
            .find_map(|&r| r.union(range).map(|u| (r, u)))
        {
            self.ranges.remove(&intersection);
            range = union;
        }
        self.ranges.insert(range);
    }

    pub fn size(&self) -> u64 {
        let range: u64 = self.ranges.iter().copied().map(Range::size).sum();
        range - u64::try_from(self.beacons.len()).unwrap()
    }
}

pub struct SensorBorders {
    pub downwards_y0: HashMap<i64, usize>,
    pub upwards_y0: HashMap<i64, usize>,
}

impl SensorBorders {
    pub fn new() -> Self {
        Self {
            downwards_y0: HashMap::new(),
            upwards_y0: HashMap::new(),
        }
    }

    pub fn add_sensor(&mut self, sensor: &Sensor) {
        let x = sensor.position.x;
        let y = sensor.position.y;
        let d = sensor.range + 1;
        self.add_downwards(y.checked_add_unsigned(d).unwrap() + x);
        self.add_upwards(y.checked_sub_unsigned(d).unwrap() - x);
        self.add_downwards(y.checked_sub_unsigned(d).unwrap() + x);
        self.add_upwards(y.checked_add_unsigned(d).unwrap() - x);
    }

    fn add_downwards(&mut self, y0: i64) {
        self.downwards_y0
            .entry(y0)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    fn add_upwards(&mut self, y0: i64) {
        self.upwards_y0
            .entry(y0)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
}

pub const fn tuning_frequency(beacon: Coordinate) -> i64 {
    beacon.x * 4_000_000 + beacon.y
}
