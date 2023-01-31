use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
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

// impl Operation {
//     const fn calculate(self, left: i64, right: i64) -> i64 {
//         match self {
//             Self::Add => left + right,
//             Self::Sub => left - right,
//             Self::Mul => left * right,
//             Self::Div => left / right,
//         }
//     }
// }

#[derive(Debug, Clone)]
pub enum Res {
    Number(i64),
    Var,
    Add(Box<Res>, Box<Res>),
    Sub(Box<Res>, Box<Res>),
    Mul(Box<Res>, Box<Res>),
    Div(Box<Res>, Box<Res>),
}

impl Res {
    pub fn new(operation: Operation, left: Self, right: Self) -> Self {
        if let (Self::Number(left), Self::Number(right)) = (&left, &right) {
            match operation {
                Operation::Add => Self::Number(left + right),
                Operation::Sub => Self::Number(left - right),
                Operation::Mul => Self::Number(left * right),
                Operation::Div => Self::Number(left / right),
            }
        } else {
            match operation {
                Operation::Add => Self::Add(Box::new(left), Box::new(right)),
                Operation::Sub => Self::Sub(Box::new(left), Box::new(right)),
                Operation::Mul => Self::Mul(Box::new(left), Box::new(right)),
                Operation::Div => Self::Div(Box::new(left), Box::new(right)),
            }
        }
    }

    pub const fn unwrap_number(&self) -> i64 {
        match self {
            Self::Number(n) => *n,
            _ => panic!("Not a number"),
        }
    }

    pub fn solve_equation(left: Self, right: Self) -> i64 {
        let (mut lhs, mut result) = match (left, right) {
            (Self::Number(left), right) => (right, left),
            (left, Self::Number(right)) => (left, right),
            _ => panic!(),
        };
        loop {
            lhs = match lhs {
                Self::Number(_) => panic!("No variable"),
                Self::Var => return result,
                Self::Add(left, right) => {
                    if let Self::Number(left) = *left {
                        result -= left;
                        *right
                    } else if let Self::Number(right) = *right {
                        result -= right;
                        *left
                    } else {
                        panic!();
                    }
                }
                Self::Sub(left, right) => {
                    if let Self::Number(left) = *left {
                        result = left - result;
                        *right
                    } else if let Self::Number(right) = *right {
                        result += right;
                        *left
                    } else {
                        panic!();
                    }
                }
                Self::Mul(left, right) => {
                    if let Self::Number(left) = *left {
                        result /= left;
                        *right
                    } else if let Self::Number(right) = *right {
                        result /= right;
                        *left
                    } else {
                        panic!();
                    }
                }
                Self::Div(left, right) => {
                    if let Self::Number(left) = *left {
                        result = left / result;
                        *right
                    } else if let Self::Number(right) = *right {
                        result *= right;
                        *left
                    } else {
                        panic!();
                    }
                }
            };
        }
    }
}

impl From<i64> for Res {
    fn from(n: i64) -> Self {
        Self::Number(n)
    }
}
