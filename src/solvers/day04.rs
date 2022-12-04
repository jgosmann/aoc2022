use std::str::FromStr;

use super::{
    base::{AocAnswer, AocPartSolution, AocSolution, AocSolver},
    error::InputParseError,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Range<T> {
    pub lower_bound: T,
    pub upper_bound: T,
}

impl<T: FromStr> TryFrom<&str> for Range<T> {
    type Error = InputParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split('-');
        return Ok(Self {
            lower_bound: parts
                .next()
                .ok_or(InputParseError::new("missing lower bound".into()))?
                .parse::<T>()
                .map_err(|_err| InputParseError::new("failed to parse lower bound".into()))?,
            upper_bound: parts
                .next()
                .ok_or(InputParseError::new("missing upper bound".into()))?
                .parse::<T>()
                .map_err(|_err| InputParseError::new("failed to parse upper bound".into()))?,
        });
    }
}

impl<T: PartialOrd> Range<T> {
    fn includes_fully(&self, other: &Self) -> bool {
        self.lower_bound <= other.lower_bound && other.upper_bound <= self.upper_bound
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.lower_bound <= other.lower_bound && other.lower_bound <= self.upper_bound)
            || (self.lower_bound <= other.upper_bound && other.upper_bound <= self.upper_bound)
            || (other.lower_bound <= self.lower_bound && self.lower_bound <= other.upper_bound)
            || (other.lower_bound <= self.upper_bound && self.upper_bound <= other.upper_bound)
    }
}

pub struct Solver {
    range_pairs: Vec<(Range<u32>, Range<u32>)>,
}

impl AocSolver for Solver {
    fn new<Iter: Iterator<Item = String>>(input: &mut Iter) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            range_pairs: input
                .filter_map(|line| {
                    let line = line.trim();
                    if line.is_empty() {
                        return None;
                    }
                    Some(Self::parse_line(line))
                })
                .collect::<Result<Vec<(Range<u32>, Range<u32>)>, InputParseError>>()?,
        })
    }

    fn solve(&self) -> anyhow::Result<super::base::AocSolution> {
        Ok(AocSolution {
            part1: AocPartSolution {
                name: "Assignments fully contained in partner's assignment:",
                answer: self
                    .range_pairs
                    .iter()
                    .filter(|&pair| {
                        pair.0.includes_fully(&pair.1) || pair.1.includes_fully(&pair.0)
                    })
                    .count() as AocAnswer,
            },
            part2: Some(AocPartSolution {
                name: "Assigments overlapping with partner:",
                answer: self
                    .range_pairs
                    .iter()
                    .filter(|&pair| pair.0.overlaps(&pair.1))
                    .count() as AocAnswer,
            }),
        })
    }
}

impl Solver {
    fn parse_line(line: &str) -> Result<(Range<u32>, Range<u32>), InputParseError> {
        let mut conv_iter = line.split(',').map(Range::<u32>::try_from);
        Ok((
            conv_iter
                .next()
                .ok_or_else(|| InputParseError::new("expected range".into()))??,
            conv_iter
                .next()
                .ok_or_else(|| InputParseError::new("expected range".into()))??,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = "\
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        ";
        test_example_input::<Solver>(input, 2, Some(4));
    }
}
