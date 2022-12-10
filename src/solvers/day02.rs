use super::{base::AocSolver, error::InputParseError};

#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct EnumParseError {
    value: u8,
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

impl TryFrom<u8> for Shape {
    type Error = EnumParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' | b'X' => Ok(Self::Rock),
            b'B' | b'Y' => Ok(Self::Paper),
            b'C' | b'Z' => Ok(Self::Scissors),
            _ => Err(EnumParseError {
                value,
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

impl TryFrom<u8> for Outcome {
    type Error = EnumParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'X' => Ok(Self::Lost),
            b'Y' => Ok(Self::Draw),
            b'Z' => Ok(Self::Won),
            _ => Err(EnumParseError {
                value,
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

type Instruction = (Shape, u8);
type StrategyGuide = Vec<Instruction>;

pub struct Solver {
    strategy_guide: StrategyGuide,
}

impl AocSolver<'_, u64, u64> for Solver {
    fn new(input: &str) -> anyhow::Result<Self> {
        Ok(Self {
            strategy_guide: input
                .lines()
                .filter_map(|line| {
                    if line.is_empty() {
                        return None;
                    }
                    Some(Self::parse_line(line.as_bytes()))
                })
                .collect::<anyhow::Result<StrategyGuide>>()?,
        })
    }

    fn solve_part1(&self) -> anyhow::Result<u64> {
        let mut tally = Tally::new();
        for (opponent, me) in &self.strategy_guide {
            let me = Shape::try_from(*me)?;
            tally.tally_round(*opponent, me);
        }
        Ok(tally.total())
    }

    fn solve_part2(&self) -> anyhow::Result<Option<u64>> {
        let mut tally = Tally::new();
        for (opponent, desired_outcome) in &self.strategy_guide {
            let desired_outcome = Outcome::try_from(*desired_outcome)?;
            tally.tally_round(*opponent, Self::choose_part2(*opponent, desired_outcome));
        }
        Ok(Some(tally.total()))
    }
}

impl Solver {
    fn parse_line(line: &[u8]) -> anyhow::Result<Instruction> {
        let mut chars = line.iter().copied();
        let mut next_char = || {
            chars
                .next()
                .ok_or_else(|| InputParseError::new("line too short".into()))
        };

        let opponent = Shape::try_from(next_char()?)?;
        if let Ok(char) = next_char() {
            if char != b' ' {
                return Err(InputParseError::new("missing or invalid separator".into()).into());
            }
        }
        let strategy = next_char()?;
        Ok((opponent, strategy))
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
        let input = include_str!("examples/day02");
        test_example_input::<Solver, _, _>(input, 15, Some(12));
    }
}
