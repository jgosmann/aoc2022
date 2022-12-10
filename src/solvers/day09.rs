use super::{base::AocSolver, error::InputParseError};

type Pos = (i32, i32);

struct Rope<const LENGTH: usize> {
    segments: [Pos; LENGTH],
}

impl<const LENGTH: usize> Rope<LENGTH> {
    pub fn new() -> Self {
        Self {
            segments: [(0, 0); LENGTH],
        }
    }

    pub fn tail(&self) -> Pos {
        *self.segments.last().unwrap()
    }

    pub fn move_rope(&mut self, delta: Pos) {
        self.segments[0] = (self.segments[0].0 + delta.0, self.segments[0].1 + delta.1);
        for i in 1..LENGTH {
            let stretch: Pos = (
                self.segments[i - 1].0 - self.segments[i].0,
                self.segments[i - 1].1 - self.segments[i].1,
            );
            if stretch.0.abs() >= 2 || stretch.1.abs() >= 2 {
                self.segments[i] = (
                    self.segments[i].0 + stretch.0.signum(),
                    self.segments[i].1 + stretch.1.signum(),
                );
            }
        }
    }
}

fn parse_direction(direction: u8) -> Result<Pos, InputParseError> {
    match direction {
        b'R' => Ok((1, 0)),
        b'L' => Ok((-1, 0)),
        b'U' => Ok((0, 1)),
        b'D' => Ok((0, -1)),
        _ => Err(InputParseError::new("invalid direction".into())),
    }
}

pub struct Solver {
    tail_positions_part1: std::collections::BTreeSet<Pos>,
    tail_positions_part2: std::collections::BTreeSet<Pos>,
}

impl AocSolver<usize, usize> for Solver {
    fn new<Iter: Iterator<Item = String>>(input: &mut Iter) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut rope_part1 = Rope::<2>::new();
        let mut rope_part2 = Rope::<10>::new();
        let mut tail_positions_part1 = std::collections::BTreeSet::new();
        let mut tail_positions_part2 = std::collections::BTreeSet::new();

        for line in input {
            let mut split_iter = line.trim().split(' ');
            let direction = parse_direction(
                split_iter
                    .next()
                    .ok_or_else(|| InputParseError::new("expected diretion".into()))?
                    .as_bytes()[0],
            )?;
            let distance: usize = split_iter
                .next()
                .ok_or_else(|| InputParseError::new("expected distance".into()))?
                .parse()?;
            for _ in 0..distance {
                rope_part1.move_rope(direction);
                rope_part2.move_rope(direction);
                tail_positions_part1.insert(rope_part1.tail());
                tail_positions_part2.insert(rope_part2.tail());
            }
        }

        Ok(Solver {
            tail_positions_part1,
            tail_positions_part2,
        })
    }

    fn solve_part1(&self) -> anyhow::Result<usize> {
        Ok(self.tail_positions_part1.len())
    }

    fn solve_part2(&self) -> anyhow::Result<Option<usize>> {
        Ok(Some(self.tail_positions_part2.len()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = "\
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2\
        ";
        test_example_input::<Solver, _, _>(input, 13, Some(1));
    }

    #[test]
    fn test_larger_example() {
        let input = "\
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20\
        ";
        let mut input = input.split('\n').map(String::from);
        let solver = Solver::new(&mut input).unwrap();
        assert_eq!(solver.solve_part2().unwrap(), Some(36));
    }
}
