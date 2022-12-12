use crate::Monkey;
use crate::WorryManagement;
use std::convert::Infallible;
use std::str::FromStr;

pub struct Troop {
    round: u32,
    monkeys: Vec<Monkey>,
}

impl Troop {
    pub fn manage_worries_trough_modulo(&mut self) {
        let worry_management = WorryManagement::Modulo(
            self.monkeys
                .iter()
                .map(|monkey| monkey.test_divisor)
                .product(),
        );
        for item in self.monkeys.iter_mut().flat_map(|monkey| &mut monkey.items) {
            item.worry_management = worry_management;
        }
    }

    pub fn do_rounds(&mut self, rounds: u32) {
        for _ in 0..rounds {
            self.do_round();
        }
    }

    pub fn do_round(&mut self) {
        self.round += 1;
        for index in 0..self.monkeys.len() {
            for throw in self.monkeys[index].do_turn() {
                self.monkeys[throw.target].items.extend(throw.items);
            }
        }
    }

    pub fn monkey_business(&self) -> u64 {
        let mut inspections: Vec<_> = self
            .monkeys
            .iter()
            .map(|monkey| monkey.inspections)
            .collect();
        inspections.sort_unstable();
        inspections[inspections.len() - 2] * inspections[inspections.len() - 1]
    }
}

impl FromStr for Troop {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let round = 0;
        let monkeys = input
            .trim()
            .replace('\r', "")
            .split("\n\n")
            .map(|monkey| monkey.parse().unwrap())
            .collect();
        Ok(Self { round, monkeys })
    }
}
