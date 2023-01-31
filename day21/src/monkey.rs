use crate::Operation;
use crate::Res;
use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;

pub struct Troop {
    monkeys: HashMap<String, Monkey>,
    results: HashMap<String, Res>,
}

impl Troop {
    pub fn new(monkeys: HashMap<String, Monkey>) -> Self {
        let results = HashMap::new();
        Self { monkeys, results }
    }

    pub fn monkey(&self, name: &str) -> &Monkey {
        self.monkeys.get(name).unwrap()
    }

    pub fn result(&mut self, name: &str) -> &Res {
        let mut deps = vec![name.to_string()];
        while let Some(name) = deps.pop() {
            if self.results.contains_key(&name) {
                continue;
            }
            match &self.monkey(&name).job {
                Job::Number(n) => {
                    self.insert_result(name, Res::from(*n));
                }
                Job::MathOperation { left, right, op } => {
                    let left_result = self.results.get(left);
                    let right_result = self.results.get(right);
                    if let (Some(left), Some(right)) = (left_result, right_result) {
                        self.insert_result(name, Res::new(*op, left.clone(), right.clone()));
                    } else {
                        deps.push(name);
                        if left_result.is_none() {
                            deps.push(left.clone());
                        }
                        if right_result.is_none() {
                            deps.push(right.clone());
                        }
                    }
                }
            };
        }
        self.results.get(name).unwrap()
    }

    pub fn insert_result(&mut self, name: String, result: Res) {
        self.results.insert(name, result);
    }
}

#[derive(Debug)]
pub struct Monkey {
    name: String,
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Number(i64),
    MathOperation {
        left: String,
        right: String,
        op: Operation,
    },
}

impl FromStr for Job {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        let job = if parts.len() == 1 {
            Self::Number(parts[0].parse().unwrap())
        } else {
            Self::MathOperation {
                left: parts[0].to_string(),
                right: parts[2].to_string(),
                op: parts[1].parse().unwrap(),
            }
        };
        Ok(job)
    }
}

impl FromStr for Monkey {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, job) = s.split_once(": ").unwrap();
        Ok(Self {
            name: name.to_string(),
            job: job.parse().unwrap(),
        })
    }
}

impl FromStr for Troop {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .trim()
            .lines()
            .map(|line| {
                let monkey: Monkey = line.parse().unwrap();
                (monkey.name.clone(), monkey)
            })
            .collect();
        Ok(Self::new(map))
    }
}
