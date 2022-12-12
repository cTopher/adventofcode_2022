use crate::Item;
use crate::Operation;
use std::convert::Infallible;
use std::fmt;
use std::str::FromStr;

pub struct Monkey {
    pub(super) items: Vec<Item>,
    operation: Operation,
    pub(super) test_divisor: u64,
    true_target: usize,
    false_target: usize,
    pub(super) inspections: u64,
}

pub struct Throw {
    pub target: usize,
    pub items: Vec<Item>,
}

impl Monkey {
    pub fn do_turn(&mut self) -> [Throw; 2] {
        let mut true_throw = Throw {
            target: self.true_target,
            items: Vec::new(),
        };
        let mut false_throw = Throw {
            target: self.false_target,
            items: Vec::new(),
        };
        for mut item in self.items.drain(..) {
            self.inspections += 1;
            self.operation.apply(&mut item);
            item.manage_worries();
            if item.worry_level % self.test_divisor == 0 {
                true_throw.items.push(item);
            } else {
                false_throw.items.push(item);
            }
        }
        [true_throw, false_throw]
    }
}

impl FromStr for Monkey {
    type Err = Infallible;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines().skip(1);
        let items = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();
        let operation = parse_prefixed(&mut lines, "Operation: ");
        let test_divisor = parse_prefixed(&mut lines, "Test: divisible by ");
        let true_target = parse_prefixed(&mut lines, "If true: throw to monkey ");
        let false_target = parse_prefixed(&mut lines, "If false: throw to monkey ");
        Ok(Self {
            items,
            operation,
            test_divisor,
            true_target,
            false_target,
            inspections: 0,
        })
    }
}

fn parse_prefixed<'a, F, E, I>(input: &mut I, prefix: &str) -> F
where
    F: FromStr<Err = E>,
    E: fmt::Debug,
    I: Iterator<Item = &'a str>,
{
    input
        .next()
        .unwrap()
        .trim()
        .strip_prefix(prefix)
        .unwrap()
        .parse()
        .unwrap()
}
