use std::string::FromUtf8Error;

use super::{base::AocSolver, error::InputParseError};

#[derive(Clone, Copy, Debug)]
struct Move {
    repeats: usize,
    from: usize,
    to: usize,
}

impl TryFrom<&str> for Move {
    type Error = InputParseError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let mut parts = line.split_ascii_whitespace();
        let mut next_part = || {
            parts
                .next()
                .ok_or_else(|| InputParseError::new("unexpected end of line".into()))
        };
        next_part()?;
        let repeats = str::parse(next_part()?)?;
        next_part()?;
        let from = str::parse::<usize>(next_part()?)? - 1;
        next_part()?;
        let to = str::parse::<usize>(next_part()?)? - 1;

        Ok(Self { repeats, from, to })
    }
}

#[derive(Clone, Debug)]
struct Stacks(Vec<Vec<u8>>);

impl TryFrom<&[&[u8]]> for Stacks {
    type Error = InputParseError;

    fn try_from(lines: &[&[u8]]) -> Result<Self, Self::Error> {
        let num_stacks = (lines.last().map(|&s| s.len()).unwrap_or(0) + 1) / 4;
        let mut stacks = Vec::with_capacity(num_stacks);
        for _ in 0..num_stacks {
            stacks.push(Vec::with_capacity(2 * lines.len()));
        }
        for line in lines.iter().rev() {
            for (i, stack) in stacks.iter_mut().enumerate() {
                let input_col = 4 * i + 1;
                if input_col < line.len() && line[input_col] != b' ' {
                    stack.push(line[input_col]);
                }
            }
        }
        Ok(Self(stacks))
    }
}

impl Stacks {
    fn apply_move(&mut self, mv: &Move) {
        for _ in 0..mv.repeats {
            if let Some(item) = self.0[mv.from].pop() {
                self.0[mv.to].push(item);
            }
        }
    }

    fn apply_multicrate_move(&mut self, mv: &Move) {
        if mv.from != mv.to {
            let from_stack = &mut self.0[mv.from];
            let items: Vec<u8> = from_stack[from_stack.len() - mv.repeats..from_stack.len()].into();
            for _ in 0..mv.repeats {
                from_stack.pop();
            }
            self.0[mv.to].extend(items.iter());
        }
    }

    fn to_answer(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(
            self.0
                .iter()
                .filter_map(|s| s.last().copied())
                .collect::<Vec<u8>>(),
        )
    }
}

pub struct Solver {
    stacks: Stacks,
    moves: Vec<Move>,
}

impl AocSolver<'_, String, String> for Solver {
    fn new(input: &str) -> anyhow::Result<Self> {
        let stack_def: Vec<&[u8]> = input
            .lines()
            .take_while(|line| !line.is_empty())
            .map(str::as_bytes)
            .collect();
        let moves = input
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip_while(|line| line.is_empty())
            .filter(|line| !line.is_empty())
            .map(Move::try_from)
            .collect::<Result<Vec<Move>, _>>()?;
        Ok(Self {
            stacks: Stacks::try_from(&stack_def[0..stack_def.len() - 1])?,
            moves,
        })
    }

    fn solve_part1(&self) -> anyhow::Result<String> {
        let mut stacks = self.stacks.clone();
        for mv in self.moves.iter() {
            stacks.apply_move(mv);
        }
        Ok(stacks.to_answer()?)
    }

    fn solve_part2(&self) -> anyhow::Result<Option<String>> {
        let mut stacks = self.stacks.clone();
        for mv in self.moves.iter() {
            stacks.apply_multicrate_move(mv);
        }
        Ok(Some(stacks.to_answer()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = include_str!("examples/day05");
        test_example_input::<Solver, _, _>(input, "CMZ".into(), Some("MCD".into()));
    }
}
