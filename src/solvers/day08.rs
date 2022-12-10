use super::base::AocSolver;

pub struct Solver<'a> {
    input: &'a [u8],
    width: usize,
}

impl<'a> AocSolver<'a, usize, usize> for Solver<'a> {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let width = input.find('\n').map(|i| i + 1).unwrap_or(input.len());
        Ok(Self {
            input: input.as_bytes(),
            width,
        })
    }

    fn solve_part1(&self) -> anyhow::Result<usize> {
        let mut visible_count = 0;
        let mut visible = vec![false; self.input.len()];
        let mut max_height;

        for i in 1..self.height() - 1 {
            max_height = self.input[i * self.width];
            for j in 1..self.width - 2 {
                let idx = j + i * self.width;
                if self.input[idx] > max_height {
                    visible[idx] = true;
                    visible_count += 1;
                    max_height = self.input[idx];
                }
            }

            max_height = self.input[(i + 1) * self.width - 2];
            for j in (1..self.width - 2).rev() {
                let idx = j + i * self.width;
                if self.input[idx] > max_height {
                    if !visible[idx] {
                        visible[idx] = true;
                        visible_count += 1;
                    }
                    max_height = self.input[idx];
                }
            }
        }

        for i in 1..self.width - 2 {
            max_height = self.input[i];
            for j in 1..self.height() - 1 {
                let idx = j * self.width + i;
                if self.input[idx] > max_height {
                    if !visible[idx] {
                        visible_count += 1;
                        visible[idx] = true;
                    }
                    max_height = self.input[idx];
                }
            }

            max_height = self.input[i + (self.height() - 1) * self.width];
            for j in (1..self.height() - 1).rev() {
                let idx = j * self.width + i;
                if self.input[idx] > max_height {
                    if !visible[idx] {
                        visible_count += 1;
                        visible[idx] = true;
                    }
                    max_height = self.input[idx];
                }
            }
        }

        Ok(visible_count + 2 * (self.width - 1) + 2 * (self.height() - 2))
    }

    fn solve_part2(&self) -> anyhow::Result<Option<usize>> {
        let mut best = 0;
        for i in 1..self.width - 2 {
            for j in 1..self.height() - 1 {
                let current = self.evaluate_tree_house_spot((i, j));
                if current > best {
                    best = current;
                }
            }
        }
        Ok(Some(best))
    }
}

impl<'a> Solver<'a> {
    fn height(&self) -> usize {
        (self.input.len() + self.width - 1) / self.width
    }

    fn evaluate_tree_house_spot(&self, pos: (usize, usize)) -> usize {
        let mut north = 1;
        let mut south = 1;
        let mut west = 1;
        let mut east = 1;
        let height = self.input[pos.0 + pos.1 * self.width];
        while pos.0 + east < self.width - 2
            && self.input[pos.0 + east + pos.1 * self.width] < height
        {
            east += 1
        }
        while west < pos.0 && self.input[pos.0 - west + pos.1 * self.width] < height {
            west += 1
        }
        while pos.1 + south < self.height() - 1
            && self.input[pos.0 + (pos.1 + south) * self.width] < height
        {
            south += 1
        }
        while north < pos.1 && self.input[pos.0 + (pos.1 - north) * self.width] < height {
            north += 1
        }
        (north) * (south) * (east) * (west)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = include_str!("examples/day08");
        test_example_input::<Solver, _, _>(input, 21, Some(8));
    }
}
