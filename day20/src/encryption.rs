use std::convert::Infallible;
use std::str::FromStr;

pub struct File {
    entries: Vec<Entry>,
    zero: usize,
}

struct Entry {
    next: usize,
    prev: usize,
    number: i64,
}

impl File {
    pub fn decrypt(&mut self, key: i64) {
        for entry in &mut self.entries {
            entry.number *= key;
        }
        for _ in 0..10 {
            self.mix();
        }
    }

    pub fn mix(&mut self) {
        for index in 0..self.entries.len() {
            self.mix_single_entry(index);
        }
    }

    pub fn get(&self, index: usize) -> i64 {
        let mut entry = &self.entries[self.zero];
        for _ in 0..index % self.entries.len() {
            entry = &self.entries[entry.next];
        }
        entry.number
    }

    fn mix_single_entry(&mut self, pointer: usize) {
        let max = i64::try_from(self.entries.len()).unwrap() - 1;
        let amount = self.entries[pointer].number % max;
        let mut target = pointer;
        match amount {
            amount if amount < 0 => {
                for _ in amount..=0 {
                    target = self.entries[target].prev;
                }
            }
            amount if amount > 0 => {
                for _ in 0..amount {
                    target = self.entries[target].next;
                }
            }
            _ => return,
        }
        self.move_entry(pointer, target);
    }

    fn move_entry(&mut self, pointer: usize, target: usize) {
        let Entry { prev, next, .. } = self.entries[pointer];
        let new_next = self.entries[target].next;
        self.entries[prev].next = next;
        self.entries[next].prev = prev;
        self.entries[target].next = pointer;
        self.entries[pointer].prev = target;
        self.entries[pointer].next = new_next;
        self.entries[new_next].prev = pointer;
    }
}

impl FromStr for File {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.trim().lines().map(|line| line.parse().unwrap()).into())
    }
}

#[allow(clippy::fallible_impl_from)]
impl<I: IntoIterator<Item = i64>> From<I> for File {
    fn from(iter: I) -> Self {
        let numbers: Vec<_> = iter.into_iter().collect();
        let last_index = numbers.len() - 1;
        let entries: Vec<_> = numbers
            .into_iter()
            .enumerate()
            .map(|(index, number)| Entry {
                next: if index == last_index { 0 } else { index + 1 },
                prev: if index == 0 { last_index } else { index - 1 },
                number,
            })
            .collect();
        let zero = entries.iter().position(|entry| entry.number == 0).unwrap();
        Self { entries, zero }
    }
}
