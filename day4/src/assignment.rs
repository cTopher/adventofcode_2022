use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SectionAssignment {
    start: u32,
    end: u32,
}

impl FromStr for SectionAssignment {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut split = input.split('-').map(|num| num.parse().unwrap());
        let start = split.next().unwrap();
        let end = split.next().unwrap();
        Ok(Self { start, end })
    }
}

impl SectionAssignment {
    pub const fn fully_overlaps_with(self, other: Self) -> bool {
        self.contains(other) || other.contains(self)
    }

    pub const fn overlaps(self, other: Self) -> bool {
        self.end >= other.start && other.end >= self.start
    }

    pub const fn contains(self, other: Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}
