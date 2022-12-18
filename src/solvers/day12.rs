use super::{base::AocSolver, error::InputParseError};

struct Grid<'a> {
    data: &'a [u8],
    width: usize,
    height: usize,
}

impl<'a> Grid<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let width = data
            .iter()
            .enumerate()
            .find_map(|(i, &c)| if c == b'\n' { Some(i) } else { None })
            .unwrap_or_default();
        let height = (data.len() + 1) / (width + 1);
        Self {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, (row, col): (usize, usize)) -> u8 {
        self.data[row * (self.width + 1) + col]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn find(&self, needle: u8) -> Option<(usize, usize)> {
        for row in 0..self.height() {
            for col in 0..self.width() {
                if self.get((row, col)) == needle {
                    return Some((row, col));
                }
            }
        }
        None
    }
}

fn to_height(marker: u8) -> u8 {
    match marker {
        b'S' => b'a',
        b'E' => b'z',
        m => m,
    }
}

pub struct Solver<'a> {
    grid: Grid<'a>,
    target: (usize, usize),
}

impl<'a> AocSolver<'a, usize, usize> for Solver<'a> {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let grid = Grid::new(input.as_bytes());
        Ok(Self {
            target: grid
                .find(b'E')
                .ok_or_else(|| InputParseError::new("missing target".into()))?,
            grid,
        })
    }

    fn solve_part1(&self) -> anyhow::Result<usize> {
        self.bfs(self.target, b'S')
    }

    fn solve_part2(&self) -> anyhow::Result<Option<usize>> {
        self.bfs(self.target, b'a').map(Some)
    }
}

impl<'a> Solver<'a> {
    fn bfs(&self, start: (usize, usize), needle: u8) -> anyhow::Result<usize> {
        let mut visited: Vec<Vec<bool>> = vec![vec![false; self.grid.width()]; self.grid.height()];
        struct QueueItem {
            distance: usize,
            pos: (usize, usize),
        }
        let mut to_visit = std::collections::VecDeque::<QueueItem>::new();
        visited[start.0][start.1] = true;
        to_visit.push_back(QueueItem {
            distance: 0,
            pos: start,
        });

        while !to_visit.is_empty() {
            let QueueItem { distance, pos } = to_visit.pop_front().unwrap();
            if self.grid.get(pos) == needle {
                return Ok(distance);
            }

            let mut visit = |pos: (usize, usize), new_pos: (usize, usize), distance: usize| {
                if !visited[new_pos.0][new_pos.1]
                    && (to_height(self.grid.get(pos)) <= 1 + to_height(self.grid.get(new_pos)))
                {
                    visited[new_pos.0][new_pos.1] = true;
                    to_visit.push_back(QueueItem {
                        distance,
                        pos: new_pos,
                    });
                }
            };

            if pos.0 > 0 {
                let new_pos = (pos.0 - 1, pos.1);
                visit(pos, new_pos, distance + 1);
            }
            if pos.0 < self.grid.height() - 1 {
                let new_pos = (pos.0 + 1, pos.1);
                visit(pos, new_pos, distance + 1);
            }
            if pos.1 > 0 {
                let new_pos = (pos.0, pos.1 - 1);
                visit(pos, new_pos, distance + 1);
            }
            if pos.1 < self.grid.width() - 1 {
                let new_pos = (pos.0, pos.1 + 1);
                visit(pos, new_pos, distance + 1);
            }
        }

        Err(anyhow::anyhow!("No solution found."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = include_str!("examples/day12");
        test_example_input::<Solver, _, _>(input, 31, Some(29));
    }
}
