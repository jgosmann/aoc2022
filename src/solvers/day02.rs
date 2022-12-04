use std::str::Chars;

use super::{
    base::{AocPartSolution, AocSolution, AocSolver},
    error::InputParseError,
};

#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct EnumParseError {
    value: String,
    enum_name: &'static str,
}

impl std::fmt::Display for EnumParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "'{}' cannot be parsed as {}",
            self.value, self.enum_name
        ))
    }
}

impl std::error::Error for EnumParseError {}

impl TryFrom<char> for Shape {
    type Error = EnumParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(EnumParseError {
                value: value.into(),
                enum_name: "Shape",
            }),
        }
    }
}

type Score = u64;

impl Shape {
    fn score(&self) -> Score {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn outcome_against(&self, other: Self) -> Outcome {
        use Shape::*;
        match (self, other) {
            (Rock, Paper) => Outcome::Lost,
            (Rock, Scissors) => Outcome::Won,
            (Paper, Rock) => Outcome::Won,
            (Paper, Scissors) => Outcome::Lost,
            (Scissors, Rock) => Outcome::Lost,
            (Scissors, Paper) => Outcome::Won,
            _ => Outcome::Draw,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Won,
    Draw,
    Lost,
}

impl Outcome {
    fn score(&self) -> Score {
        match self {
            Self::Won => 6,
            Self::Draw => 3,
            Self::Lost => 0,
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = EnumParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Lost),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Won),
            _ => Err(EnumParseError {
                value: value.into(),
                enum_name: "Outcome",
            }),
        }
    }
}

struct Tally(Score);

impl Tally {
    fn new() -> Self {
        Self(0)
    }

    fn tally_round(&mut self, opponent: Shape, me: Shape) {
        self.0 += me.score() + me.outcome_against(opponent).score();
    }

    fn total(&self) -> Score {
        self.0
    }
}

type Instruction = (Shape, char);
type StrategyGuide = Vec<Instruction>;

pub struct Solver {
    strategy_guide: StrategyGuide,
}

impl AocSolver for Solver {
    fn new<Iter: Iterator<Item = String>>(input: &mut Iter) -> anyhow::Result<Self> {
        Ok(Self {
            strategy_guide: input
                .filter_map(|line| {
                    let line = line.trim();
                    if line.is_empty() {
                        return None;
                    }
                    Some(Self::parse_line(line.chars()))
                })
                .collect::<anyhow::Result<StrategyGuide>>()?,
        })
    }

    fn solve(&self) -> anyhow::Result<super::base::AocSolution> {
        Ok(AocSolution {
            part1: AocPartSolution {
                name: "Score part 1:",
                answer: self.solve_part1()?,
            },
            part2: Some(AocPartSolution {
                name: "Score part 2:",
                answer: self.solve_part2()?,
            }),
        })
    }
}

impl Solver {
    fn parse_line(mut chars: Chars) -> anyhow::Result<Instruction> {
        let mut next_char = || {
            chars
                .next()
                .ok_or(InputParseError::new("line too short".into()))
        };

        let opponent = Shape::try_from(next_char()?)?;
        if let Ok(char) = next_char() {
            if char != ' ' {
                return Err(InputParseError::new("missing or invalid separator".into()).into());
            }
        }
        let strategy = next_char()?;
        Ok((opponent, strategy))
    }

    fn solve_part1(&self) -> anyhow::Result<Score> {
        let mut tally = Tally::new();
        for (opponent, me) in &self.strategy_guide {
            let me = Shape::try_from(*me)?;
            tally.tally_round(*opponent, me);
        }
        Ok(tally.total())
    }

    fn solve_part2(&self) -> anyhow::Result<Score> {
        let mut tally = Tally::new();
        for (opponent, desired_outcome) in &self.strategy_guide {
            let desired_outcome = Outcome::try_from(*desired_outcome)?;
            tally.tally_round(*opponent, Self::choose_part2(*opponent, desired_outcome));
        }
        Ok(tally.total())
    }

    fn choose_part2(opponent: Shape, desired_outcome: Outcome) -> Shape {
        match desired_outcome {
            Outcome::Draw => opponent,
            Outcome::Won => Self::winning_shape_against(opponent),
            Outcome::Lost => Self::losing_shape_against(opponent),
        }
    }

    fn winning_shape_against(shape: Shape) -> Shape {
        use Shape::*;
        match shape {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn losing_shape_against(shape: Shape) -> Shape {
        use Shape::*;
        match shape {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = "\
            A Y
            B X
            C Z
        ";
        test_example_input::<Solver>(input, 15, Some(12));
    }
}
