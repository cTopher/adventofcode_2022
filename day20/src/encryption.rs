use std::collections::VecDeque;
use std::convert::Infallible;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub struct File {
    enumeration: VecDeque<(usize, isize)>,
}

impl File {
    pub fn decrypt(&mut self) {
        let mut pointer = 0;
        for index in 0..self.enumeration.len() {
            while self.enumeration[pointer].0 != index {
                pointer += 1;
            }
            self.mix_at(pointer);
        }
    }

    pub fn get(&self, index: usize) -> isize {
        let (_, value) = self.enumeration[index % self.enumeration.len()];
        value
    }

    fn mix_at(&mut self, mut pointer: usize) {
        let (_, amount) = self.enumeration[pointer];
        let amount = amount % (self.signed_len() - 1);
        if amount < 0 {
            for _ in amount..0 {
                match pointer {
                    0 => {
                        let front = self.enumeration.pop_front().unwrap();
                        self.enumeration.push_back(front);
                        pointer = self.last_index() - 1;
                        self.enumeration.swap(pointer, pointer + 1);
                    }
                    1 => {
                        self.enumeration.swap(0, 1);
                        let front = self.enumeration.pop_front().unwrap();
                        self.enumeration.push_back(front);
                        pointer = self.last_index();
                    }
                    _ => {
                        self.enumeration.swap(pointer - 1, pointer);
                        pointer -= 1;
                    }
                }
            }
        } else {
            for _ in 0..amount {
                if pointer == self.last_index() - 1 {
                    self.enumeration.swap(pointer, pointer + 1);
                    let back = self.enumeration.pop_back().unwrap();
                    self.enumeration.push_front(back);
                    pointer = 0;
                } else {
                    self.enumeration.swap(pointer, pointer + 1);
                    pointer += 1;
                }
            }
        }
    }
    pub fn position(&self, value: isize) -> usize {
        self.enumeration
            .iter()
            .position(|(_, v)| *v == value)
            .unwrap()
    }

    fn last_index(&self) -> usize {
        self.enumeration.len() - 1
    }

    pub fn signed_len(&self) -> isize {
        self.enumeration.len().try_into().unwrap()
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x: Vec<_> = self
            .enumeration
            .iter()
            .map(|(_, value)| value.to_string())
            .collect();
        x.join(", ").fmt(f)
    }
}

impl FromStr for File {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let enumeration = s
            .trim()
            .lines()
            .map(|line| line.parse().unwrap())
            .enumerate()
            .collect();
        Ok(Self { enumeration })
    }
}
