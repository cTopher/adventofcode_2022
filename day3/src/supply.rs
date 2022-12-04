#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Supply(char);

const LOWERCASE_PRIORITY_OFFSET: u32 = 96;
const UPPERCASE_PRIORITY_OFFSET: u32 = 38;

impl Supply {
    pub const fn new(input: char) -> Self {
        Self(input)
    }

    pub fn priority(self) -> u32 {
        let Self(char) = self;
        if char.is_lowercase() {
            char as u32 - LOWERCASE_PRIORITY_OFFSET
        } else {
            char as u32 - UPPERCASE_PRIORITY_OFFSET
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priorities() {
        assert_eq!(1, Supply::new('a').priority());
        assert_eq!(26, Supply::new('z').priority());
        assert_eq!(27, Supply::new('A').priority());
        assert_eq!(52, Supply::new('Z').priority());
    }
}
