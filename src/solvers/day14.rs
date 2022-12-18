use super::{base::AocSolver, error::InputParseError};

type Point = (usize, usize);
type Line = Vec<Point>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Element {
    Air,
    Rock,
    Sand,
}

#[derive(Debug)]
struct Cave {
    min_x: usize,
    max_x: usize,
    max_y: usize,
    map: Vec<Element>,
}

impl Cave {
    pub fn new(min_x: usize, max_x: usize, max_y: usize) -> Self {
        Self {
            min_x,
            max_x,
            max_y,
            map: vec![Element::Air; (max_x + 1 - min_x + 2) * (max_y + 1)],
        }
    }

    fn point2idx(&self, point: &Point) -> usize {
        (point.1 * (self.max_x - self.min_x + 1 + 2)) + (point.0 + 1 - self.min_x)
    }

    pub fn add_rock(&mut self, line: &[Point]) {
        for i in 1..line.len() {
            self.add_rock_between_points(&line[i - 1], &line[i]);
        }
    }

    pub fn add_rock_between_points(&mut self, start: &Point, end: &Point) {
        if start.0 == end.0 {
            let (start, end) = if start.1 > end.1 {
                (end, start)
            } else {
                (start, end)
            };
            for y in start.1..=end.1 {
                let idx = self.point2idx(&(start.0, y));
                self.map[idx] = Element::Rock;
            }
        } else if start.1 == end.1 {
            let (start, end) = if start.0 > end.0 {
                (end, start)
            } else {
                (start, end)
            };
            for x in start.0..=end.0 {
                let idx = self.point2idx(&(x, start.1));
                self.map[idx] = Element::Rock;
            }
        } else {
            panic!("must be called with horizontal or vertical line");
        }
    }

    pub fn fill_with_sand(&mut self) -> usize {
        let mut count = 0;
        while self.drop_sand().is_some() {
            count += 1;
            if self.map[self.point2idx(&(500, 0))] == Element::Sand {
                break;
            }
        }
        count
    }

    fn drop_sand(&mut self) -> Option<Point> {
        let mut current_pos = (500, 0);
        loop {
            if current_pos.1 >= self.max_y {
                return None;
            }
            if self.map[self.point2idx(&(current_pos.0, current_pos.1 + 1))] == Element::Air {
                current_pos = (current_pos.0, current_pos.1 + 1);
            } else if self.map[self.point2idx(&(current_pos.0 - 1, current_pos.1 + 1))]
                == Element::Air
            {
                current_pos = (current_pos.0 - 1, current_pos.1 + 1);
            } else if self.map[self.point2idx(&(current_pos.0 + 1, current_pos.1 + 1))]
                == Element::Air
            {
                current_pos = (current_pos.0 + 1, current_pos.1 + 1);
            } else {
                let idx = self.point2idx(&current_pos);
                self.map[idx] = Element::Sand;
                return Some(current_pos);
            }
        }
    }
}

pub struct Solver {
    lines: Vec<Line>,
    min_x: usize,
    max_x: usize,
    max_y: usize,
}

impl<'a> AocSolver<'a, usize, usize> for Solver {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let lines: Vec<Line> = input
            .split('\n')
            .filter_map(|line| {
                if line.trim().is_empty() {
                    return None;
                }
                Some(
                    line.split(" -> ")
                        .map(|point_def| {
                            let mut coords = point_def.split(',').map(str::parse);
                            Ok((
                                coords.next().ok_or_else(|| {
                                    InputParseError::new("missing coordinate".into())
                                })??,
                                coords.next().ok_or_else(|| {
                                    InputParseError::new("missing coordinate".into())
                                })??,
                            ))
                        })
                        .collect::<Result<Vec<Point>, InputParseError>>(),
                )
            })
            .collect::<Result<Vec<Line>, InputParseError>>()?;
        let min_x = *lines
            .iter()
            .flatten()
            .map(|(x, _)| x)
            .min()
            .ok_or_else(|| InputParseError::new("no rock".into()))?;
        let max_x = *lines
            .iter()
            .flatten()
            .map(|(x, _)| x)
            .max()
            .ok_or_else(|| InputParseError::new("no rock".into()))?;
        let max_y = *lines
            .iter()
            .flatten()
            .map(|(_, y)| y)
            .max()
            .ok_or_else(|| InputParseError::new("no rock".into()))?;

        Ok(Self {
            lines,
            min_x,
            max_x,
            max_y,
        })
    }

    fn solve_part1(&self) -> anyhow::Result<usize> {
        let mut cave = Cave::new(self.min_x, self.max_x, self.max_y);
        for line in &self.lines {
            cave.add_rock(line);
        }

        Ok(cave.fill_with_sand())
    }

    fn solve_part2(&self) -> anyhow::Result<Option<usize>> {
        let max_y = self.max_y + 2;
        let mut cave = Cave::new(
            self.min_x.min(500 - max_y),
            self.max_x.max(500 + max_y),
            max_y,
        );
        for line in &self.lines {
            cave.add_rock(line)
        }
        cave.add_rock(&[(500 - max_y, max_y), (500 + max_y, max_y)]);
        Ok(Some(cave.fill_with_sand()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = include_str!("examples/day14");
        test_example_input::<Solver, _, _>(input, 24, Some(93));
    }
}
