use std::collections::HashMap;

use super::{base::AocSolver, error::InputParseError};

#[derive(Copy, Clone, Debug)]
enum Statement<'a> {
    Const(i64),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

impl<'a> Statement<'a> {
    fn lhs(&self) -> Option<&'a str> {
        match self {
            Statement::Const(_) => None,
            Statement::Add(lhs, _) => Some(lhs),
            Statement::Sub(lhs, _) => Some(lhs),
            Statement::Mul(lhs, _) => Some(lhs),
            Statement::Div(lhs, _) => Some(lhs),
        }
    }

    fn rhs(&self) -> Option<&'a str> {
        match self {
            Statement::Const(_) => None,
            Statement::Add(_, rhs) => Some(rhs),
            Statement::Sub(_, rhs) => Some(rhs),
            Statement::Mul(_, rhs) => Some(rhs),
            Statement::Div(_, rhs) => Some(rhs),
        }
    }
}

pub struct Solver<'a> {
    monkeys: HashMap<&'a str, Statement<'a>>,
}

impl<'a> AocSolver<'a, i64, i64> for Solver<'a> {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut monkeys: HashMap<&str, Statement> = HashMap::new();
        for line in input.split('\n') {
            if line.trim().is_empty() {
                continue;
            }
            let mut parts = line.split(": ");
            let key = parts
                .next()
                .ok_or_else(|| InputParseError::new("missing key".into()))?;
            let value = parts
                .next()
                .ok_or_else(|| InputParseError::new("missing value".into()))?;
            if value.as_bytes().iter().all(u8::is_ascii_digit) {
                monkeys.insert(key, Statement::Const(value.parse()?));
            } else {
                let mut value_parts = value.split(' ');
                let lhs = value_parts
                    .next()
                    .ok_or_else(|| InputParseError::new("missing lhs".into()))?;
                let op = value_parts
                    .next()
                    .ok_or_else(|| InputParseError::new("missing op".into()))?;
                let rhs = value_parts
                    .next()
                    .ok_or_else(|| InputParseError::new("missing rhs".into()))?;
                let value = match op {
                    "+" => Ok(Statement::Add(lhs, rhs)),
                    "-" => Ok(Statement::Sub(lhs, rhs)),
                    "*" => Ok(Statement::Mul(lhs, rhs)),
                    "/" => Ok(Statement::Div(lhs, rhs)),
                    _ => Err(InputParseError::new("invalid op".into())),
                };
                monkeys.insert(key, value?);
            }
        }

        Ok(Self { monkeys })
    }

    fn solve_part1(&self) -> anyhow::Result<i64> {
        Ok(Self::eval(&self.monkeys, "root"))
    }

    fn solve_part2(&self) -> anyhow::Result<Option<i64>> {
        let (with_human, others) = if Self::contains(
            &self.monkeys,
            self.monkeys
                .get("root")
                .unwrap()
                .lhs()
                .ok_or_else(|| InputParseError::new("invalid root".into()))?,
            "humn",
        ) {
            (
                self.monkeys.get("root").unwrap().lhs().unwrap(),
                self.monkeys.get("root").unwrap().rhs().unwrap(),
            )
        } else {
            (
                self.monkeys.get("root").unwrap().rhs().unwrap(),
                self.monkeys.get("root").unwrap().lhs().unwrap(),
            )
        };
        let target_value = Self::eval(&self.monkeys, others);

        Ok(Some(Self::solve(&self.monkeys, with_human, target_value)))
    }
}

impl<'a> Solver<'a> {
    fn eval(monkeys: &HashMap<&'a str, Statement<'a>>, node: &'a str) -> i64 {
        match *monkeys.get(node).unwrap() {
            Statement::Const(value) => value,
            Statement::Add(lhs, rhs) => Self::eval(monkeys, lhs) + Self::eval(monkeys, rhs),
            Statement::Sub(lhs, rhs) => Self::eval(monkeys, lhs) - Self::eval(monkeys, rhs),
            Statement::Mul(lhs, rhs) => Self::eval(monkeys, lhs) * Self::eval(monkeys, rhs),
            Statement::Div(lhs, rhs) => Self::eval(monkeys, lhs) / Self::eval(monkeys, rhs),
        }
    }

    fn contains(monkeys: &HashMap<&'a str, Statement<'a>>, node: &'a str, needle: &'a str) -> bool {
        if node == needle {
            return true;
        }
        match monkeys.get(node).unwrap() {
            Statement::Const(_) => false,
            statement => {
                Self::contains(monkeys, statement.lhs().unwrap(), needle)
                    || Self::contains(monkeys, statement.rhs().unwrap(), needle)
            }
        }
    }

    fn solve(monkeys: &HashMap<&'a str, Statement<'a>>, node: &'a str, target_value: i64) -> i64 {
        if node == "humn" {
            return target_value;
        }

        let node = monkeys.get(node).unwrap();

        let (with_human, others) = if Self::contains(monkeys, node.lhs().unwrap(), "humn") {
            (node.lhs().unwrap(), node.rhs().unwrap())
        } else {
            (node.rhs().unwrap(), node.lhs().unwrap())
        };

        let other_value = Self::eval(monkeys, others);
        let new_target_value = match node {
            Statement::Const(_) => panic!(),
            Statement::Add(_, _) => target_value - other_value,
            Statement::Sub(lhs, _) => {
                if *lhs == with_human {
                    target_value + other_value
                } else {
                    other_value - target_value
                }
            }
            Statement::Mul(_, _) => target_value / other_value,
            Statement::Div(lhs, _) => {
                if *lhs == with_human {
                    target_value * other_value
                } else {
                    other_value / target_value
                }
            }
        };
        Self::solve(monkeys, with_human, new_target_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = include_str!("examples/day21");
        test_example_input::<Solver, _, _>(input, 152, Some(301));
    }
}
