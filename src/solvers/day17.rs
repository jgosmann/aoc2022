use std::fmt::{Display, Write};

use super::base::AocSolver;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum RockShape {
    Minus,
    Plus,
    FlippedL,
    I,
    Square,
}

struct RockShapeIterator(u8);

impl Iterator for RockShapeIterator {
    type Item = RockShape;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        Some(match self.0 {
            1 => RockShape::Minus,
            2 => RockShape::Plus,
            3 => RockShape::FlippedL,
            4 => RockShape::I,
            5 => {
                self.0 = 0;
                RockShape::Square
            }
            _ => panic!("should not happen"),
        })
    }
}

impl RockShapeIterator {
    fn new() -> Self {
        Self(0)
    }
}

struct Tetris<const WIDTH: usize, JP>
where
    JP: Iterator<Item = i8>,
{
    chamber: Vec<[bool; WIDTH]>,
    jet_pattern: JP,
    rock_shapes: RockShapeIterator,
    tower_height: usize,
}

impl<const WIDTH: usize, JP> Tetris<WIDTH, JP>
where
    JP: Iterator<Item = i8>,
{
    pub fn new(jet_pattern: JP) -> Self {
        Self {
            chamber: vec![],
            jet_pattern,
            rock_shapes: RockShapeIterator::new(),
            tower_height: 0,
        }
    }

    pub fn tower_height(&self) -> usize {
        self.tower_height
    }

    pub fn drop_block(&mut self) {
        let shape = self.rock_shapes.next().unwrap();
        let mut bottom = self.tower_height + 3;
        let mut left: isize = 2;

        while self.chamber.len() < self.tower_height + 3 + 4 {
            self.chamber.push([false; WIDTH]);
        }

        loop {
            let jet = self.jet_pattern.next().unwrap() as isize;
            if self.check(shape, bottom, left + jet) {
                left += jet;
            }

            if bottom > 0 && self.check(shape, bottom - 1, left) {
                bottom -= 1;
            } else {
                self.materialize(shape, bottom, left as usize);
                break;
            }
        }
    }

    fn check(&self, shape: RockShape, bottom: usize, left: isize) -> bool {
        if left < 0 {
            return false;
        }
        let left = left as usize;
        match shape {
            RockShape::Minus => {
                left + 3 < WIDTH && self.chamber[bottom][left..left + 4].iter().all(|f| !f)
            }
            RockShape::Plus => {
                left + 2 < WIDTH
                    && self.chamber[bottom + 1][left..left + 3].iter().all(|f| !f)
                    && !self.chamber[bottom][left + 1]
                    && !self.chamber[bottom + 2][left + 1]
            }
            RockShape::FlippedL => {
                left + 2 < WIDTH
                    && self.chamber[bottom][left..left + 3].iter().all(|f| !f)
                    && !self.chamber[bottom + 1][left + 2]
                    && !self.chamber[bottom + 2][left + 2]
            }
            RockShape::I => {
                left < WIDTH
                    && self.chamber[bottom..bottom + 4]
                        .iter()
                        .all(|row| !row[left])
            }
            RockShape::Square => {
                left + 1 < WIDTH
                    && !self.chamber[bottom][left]
                    && !self.chamber[bottom][left + 1]
                    && !self.chamber[bottom + 1][left]
                    && !self.chamber[bottom + 1][left + 1]
            }
        }
    }

    fn materialize(&mut self, shape: RockShape, bottom: usize, left: usize) {
        match shape {
            RockShape::Minus => {
                for field in self.chamber[bottom][left..left + 4].iter_mut() {
                    *field = true;
                }
                self.tower_height = self.tower_height.max(bottom + 1);
            }
            RockShape::Plus => {
                self.chamber[bottom + 1][left] = true;
                self.chamber[bottom + 1][left + 1] = true;
                self.chamber[bottom + 1][left + 2] = true;
                self.chamber[bottom][left + 1] = true;
                self.chamber[bottom + 2][left + 1] = true;
                self.tower_height = self.tower_height.max(bottom + 3);
            }
            RockShape::FlippedL => {
                self.chamber[bottom][left] = true;
                self.chamber[bottom][left + 1] = true;
                self.chamber[bottom][left + 2] = true;
                self.chamber[bottom + 1][left + 2] = true;
                self.chamber[bottom + 2][left + 2] = true;
                self.tower_height = self.tower_height.max(bottom + 3);
            }
            RockShape::I => {
                for row in self.chamber[bottom..bottom + 4].iter_mut() {
                    row[left] = true;
                }
                self.tower_height = self.tower_height.max(bottom + 4);
            }
            RockShape::Square => {
                self.chamber[bottom][left] = true;
                self.chamber[bottom][left + 1] = true;
                self.chamber[bottom + 1][left] = true;
                self.chamber[bottom + 1][left + 1] = true;
                self.tower_height = self.tower_height.max(bottom + 2);
            }
        }
    }
}

impl<const WIDTH: usize, JP> Display for Tetris<WIDTH, JP>
where
    JP: Iterator<Item = i8>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (self.tower_height - 3..self.tower_height).rev() {
            f.write_char('|')?;
            for cell in self.chamber[row] {
                if cell {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_str("|\n")?;
        }
        f.write_char('+')?;
        for _ in 0..WIDTH {
            f.write_char('-')?;
        }
        f.write_str("+\n")?;
        Ok(())
    }
}

pub struct Solver<'a> {
    input: &'a str,
}

impl<'a> AocSolver<'a, usize, usize> for Solver<'a> {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self { input })
    }

    fn solve_part1(&self) -> anyhow::Result<usize> {
        let mut tetris: Tetris<7, _> = Tetris::new(
            self.input
                .as_bytes()
                .iter()
                .filter_map(|c| match c {
                    b'<' => Some(-1),
                    b'>' => Some(1),
                    _ => None,
                })
                .cycle(),
        );
        for _ in 0..2022 {
            tetris.drop_block();
        }
        Ok(tetris.tower_height())
    }

    fn solve_part2(&self) -> anyhow::Result<Option<usize>> {
        Ok(Some(self.find_repeated_cycles()))
    }
}

impl<'a> Solver<'a> {
    fn find_repeated_cycles(&self) -> usize {
        let mut tetris: Tetris<7, _> = Tetris::new(
            self.input
                .as_bytes()
                .iter()
                .filter_map(|c| match c {
                    b'<' => Some(-1),
                    b'>' => Some(1),
                    _ => None,
                })
                .cycle(),
        );
        let cycle_size = 5 * self.input.trim().len();
        let mut heights: Vec<usize> = vec![0];
        let mut cycle: usize = 0;
        loop {
            for _ in 0..cycle_size {
                tetris.drop_block();
            }
            heights.push(tetris.tower_height());
            for cycles_count in 1..=cycle / 2 {
                let h1 = heights[heights.len() - 1];
                let h2 = heights[heights.len() - cycles_count - 1];
                let h3 = heights[heights.len() - 2 * cycles_count - 1];
                if h1 - h2 == h2 - h3
                    && (0..h1 - h2).all(|x| tetris.chamber[h2 + x] == tetris.chamber[h3 + x])
                {
                    let rocks_before_start = (cycle - 2 * cycles_count + 1) * cycle_size;
                    let num_repeats =
                        (1000000000000 - rocks_before_start) / (cycles_count * cycle_size);
                    let remaining_rocks =
                        (1000000000000 - rocks_before_start) % (cycles_count * cycle_size);
                    let prev_height = tetris.tower_height();
                    for _ in 0..remaining_rocks {
                        tetris.drop_block();
                    }
                    return h3 + (h2 - h3) * num_repeats + tetris.tower_height() - prev_height;
                }
            }
            cycle += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = include_str!("examples/day17");
        test_example_input::<Solver, _, _>(input, 3068, Some(1514285714288));
    }
}
