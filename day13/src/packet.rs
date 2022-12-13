use std::cmp::Ordering;
use std::convert::Infallible;
use std::fmt;
use std::iter::Peekable;
use std::str::{Chars, FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    List(Vec<Self>),
    Integer(u32),
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::List(list) => {
                write!(f, "[")?;
                for (index, data) in list.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", data)?;
                }
                write!(f, "]")
            }
            Self::Integer(integer) => write!(f, "{integer}"),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(left), Self::Integer(right)) => left.cmp(right),
            (Self::List(left), Self::List(right)) => left.cmp(right),
            (Self::Integer(left), Self::List(right)) => vec![Self::Integer(*left)].cmp(right),
            (Self::List(left), Self::Integer(right)) => left.cmp(&vec![Self::Integer(*right)]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    pub fn parse(chars: &mut Peekable<Chars<'_>>) -> Self {
        match chars.peek().unwrap() {
            '[' => Self::parse_list(chars),
            _ => Self::parse_integer(chars),
        }
    }

    fn parse_list(chars: &mut Peekable<Chars<'_>>) -> Self {
        assert_eq!(chars.next(), Some('['));
        let mut list = Vec::new();
        loop {
            match chars.peek().unwrap() {
                ']' => {
                    chars.next();
                    return Self::List(list);
                }
                ',' => {
                    chars.next();
                }
                _ => list.push(Self::parse(chars)),
            }
        }
    }

    fn parse_integer(chars: &mut Peekable<Chars<'_>>) -> Self {
        let mut integer = String::new();
        loop {
            match chars.peek().unwrap() {
                ']' | ',' => return Self::Integer(integer.parse().unwrap()),
                _ => integer.push(chars.next().unwrap()),
            }
        }
    }
}

impl FromStr for Packet {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(&mut s.chars().peekable()))
    }
}
