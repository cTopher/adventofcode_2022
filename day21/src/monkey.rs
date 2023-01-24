use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;

pub struct Monkeys {
    map: HashMap<String, Monkey>,
}

impl Monkeys {
    pub fn result(&self, name: &str) -> i64 {
        let mut deps = vec![self.map.get(name).unwrap()];
        let mut results: HashMap<&str, i64> = HashMap::new();
        while let Some(monkey) = deps.pop() {
            if results.contains_key(monkey.name.as_str()) {
                continue;
            }
            match &monkey.job {
                Job::Number(n) => {
                    results.insert(&monkey.name, *n);
                }
                Job::MathOperation { left, right, op } => {
                    let left_result = results.get(left.as_str());
                    let right_result = results.get(right.as_str());
                    if let (Some(&left), Some(&right)) = (left_result, right_result) {
                        results.insert(&monkey.name, op.calculate(left, right));
                    } else {
                        deps.push(monkey);
                        if left_result.is_none() {
                            deps.push(self.map.get(left).unwrap());
                        }
                        if right_result.is_none() {
                            deps.push(self.map.get(right).unwrap());
                        }
                    }
                }
            };
        }
        *results.get(name).unwrap()
    }
}

#[derive(Debug)]
struct Monkey {
    name: String,
    job: Job,
}

#[derive(Debug)]
enum Job {
    Number(i64),
    MathOperation {
        left: String,
        right: String,
        op: Operation,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    const fn calculate(self, left: i64, right: i64) -> i64 {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
        }
    }
}

impl FromStr for Operation {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            op => panic!("Unknown operation: {op}"),
        }
    }
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

impl FromStr for Monkeys {
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
        Ok(Self { map })
    }
}
