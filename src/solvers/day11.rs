use std::{cell::RefCell, collections::VecDeque, num::ParseIntError};

use super::{base::AocSolver, day01::TopK, error::InputParseError};

#[derive(Clone, Copy, Debug)]
enum Operation {
    AddConst(u64),
    MultiplyConst(u64),
    Square,
}

impl TryFrom<&str> for Operation {
    type Error = InputParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let operation = value
            .strip_prefix("new = old ")
            .ok_or_else(|| InputParseError::new("unsupported operation format".into()))?;
        if let Some(summand) = operation.strip_prefix('+') {
            Ok(Self::AddConst(summand.trim().parse()?))
        } else if let Some(factor) = operation.strip_prefix('*') {
            match factor.trim() {
                "old" => Ok(Self::Square),
                value => Ok(Self::MultiplyConst(value.parse()?)),
            }
        } else {
            Err(InputParseError::new("unsupported operation".into()))
        }
    }
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        match self {
            Operation::AddConst(summand) => old + summand,
            Operation::MultiplyConst(factor) => old * factor,
            Operation::Square => old * old,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisor: u64,
    target_if_divisible: usize,
    target_if_not_divisible: usize,
    num_items_inspected: usize,
}

#[derive(Clone, Debug)]
struct MonkeyBusiness {
    monkeys: Vec<RefCell<Monkey>>,
    modulus: u64,
}

impl MonkeyBusiness {
    fn round(&mut self, calm_down_factor: u64) {
        for monkey in &self.monkeys {
            let mut monkey = monkey.borrow_mut();
            while let Some(item) = monkey.items.pop_front() {
                monkey.num_items_inspected += 1;
                let worry_level = (monkey.operation.apply(item) / calm_down_factor) % self.modulus;
                if worry_level % monkey.divisor == 0 {
                    self.monkeys[monkey.target_if_divisible]
                        .borrow_mut()
                        .items
                        .push_back(worry_level);
                } else {
                    self.monkeys[monkey.target_if_not_divisible]
                        .borrow_mut()
                        .items
                        .push_back(worry_level);
                }
            }
        }
    }

    fn level(&self) -> u64 {
        let mut top2 = TopK::<u64, 2>::new();
        for monkey in &self.monkeys {
            top2.push(monkey.borrow().num_items_inspected as u64);
        }
        top2.iter().product()
    }
}

struct MonkeyReader<'a, I: Iterator<Item = &'a str>> {
    lines: I,
}

impl<'a, I: Iterator<Item = &'a str>> MonkeyReader<'a, I> {
    pub fn new(lines: I) -> Self {
        Self { lines }
    }

    fn try_next(&mut self) -> anyhow::Result<Option<Monkey>> {
        let mut start_line = self.lines.next();
        while let Some("") = start_line.map(str::trim) {
            start_line = self.lines.next();
        }
        match start_line.map(|line| line.starts_with("Monkey ")) {
            None => return Ok(None),
            Some(false) => return Ok(None),
            _ => {}
        }
        let starting_items: VecDeque<u64> = self
            .lines
            .next()
            .ok_or_else(|| InputParseError::new("missing starting items".into()))?
            .trim()
            .strip_prefix("Starting items: ")
            .ok_or_else(|| InputParseError::new("invalid key".into()))?
            .split(", ")
            .map(|item| item.parse::<u64>())
            .collect::<Result<VecDeque<u64>, ParseIntError>>()?;
        let operation = Operation::try_from(
            self.lines
                .next()
                .ok_or_else(|| InputParseError::new("missing operation".into()))?
                .trim()
                .strip_prefix("Operation: ")
                .ok_or_else(|| InputParseError::new("invalid key".into()))?,
        )?;
        let divisor: u64 = self
            .lines
            .next()
            .ok_or_else(|| InputParseError::new("missing divisor".into()))?
            .trim()
            .strip_prefix("Test: divisible by ")
            .ok_or_else(|| InputParseError::new("invalid key".into()))?
            .parse()?;
        let target_if_divisible: usize = self
            .lines
            .next()
            .ok_or_else(|| InputParseError::new("missing target".into()))?
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .ok_or_else(|| InputParseError::new("invalid key".into()))?
            .parse()?;
        let target_if_not_divisible: usize = self
            .lines
            .next()
            .ok_or_else(|| InputParseError::new("missing target".into()))?
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .ok_or_else(|| InputParseError::new("invalid key".into()))?
            .parse()?;
        Ok(Some(Monkey {
            items: starting_items,
            operation,
            divisor,
            target_if_divisible,
            target_if_not_divisible,
            num_items_inspected: 0,
        }))
    }
}

impl<'a, I: Iterator<Item = &'a str>> Iterator for MonkeyReader<'a, I> {
    type Item = anyhow::Result<Monkey>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(item)) => Some(Ok(item)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

pub struct Solver {
    monkey_business: MonkeyBusiness,
}

impl<'a> AocSolver<'a, u64, u64> for Solver {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut monkeys: Vec<RefCell<Monkey>> = vec![];
        for monkey in MonkeyReader::new(input.lines()) {
            monkeys.push(RefCell::new(monkey?));
        }
        let modulus = monkeys.iter().map(|m| m.borrow().divisor).product();
        Ok(Self {
            monkey_business: MonkeyBusiness { monkeys, modulus },
        })
    }

    fn solve_part1(&self) -> anyhow::Result<u64> {
        let mut monkey_business = self.monkey_business.clone();
        for _ in 0..20 {
            monkey_business.round(3);
        }
        Ok(monkey_business.level())
    }

    fn solve_part2(&self) -> anyhow::Result<Option<u64>> {
        let mut monkey_business = self.monkey_business.clone();
        for _ in 0..10000 {
            monkey_business.round(1);
        }
        Ok(Some(monkey_business.level()))
    }
}

impl Solver {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = include_str!("examples/day11");
        test_example_input::<Solver, _, _>(input, 10605, Some(2713310158));
    }
}
