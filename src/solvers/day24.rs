use std::collections::VecDeque;

use super::{base::AocSolver, error::InputParseError};

#[derive(Debug)]
struct Map {
    rightwards: Vec<Vec<usize>>,
    leftwards: Vec<Vec<usize>>,
    downwards: Vec<Vec<usize>>,
    upwards: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Map {
    fn is_blocked(&self, timestep: usize, pos: (usize, usize)) -> bool {
        self.rightwards[pos.0]
            .iter()
            .any(|blizzard_col| ((blizzard_col + timestep) % self.width) == pos.1)
            || self.leftwards[pos.0].iter().any(|blizzard_col| {
                ((self.width + blizzard_col - timestep % self.width) % self.width) == pos.1
            })
            || self.downwards[pos.1]
                .iter()
                .any(|blizzard_row| ((blizzard_row + timestep) % self.height) == pos.0)
            || self.upwards[pos.1].iter().any(|blizzard_row| {
                ((self.height + blizzard_row - timestep % self.height) % self.height) == pos.0
            })
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

struct PathFinder {
    map: Map,
    start: (usize, usize),
    target: (usize, usize),
}

pub struct Solver {
    path_finder: PathFinder,
    steps_part1: usize,
}

impl<'a> AocSolver<'a, usize, usize> for Solver {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut rightwards: Vec<Vec<usize>> = vec![];
        let mut leftwards: Vec<Vec<usize>> = vec![];
        let mut downwards: Vec<Vec<usize>> = vec![];
        let mut upwards: Vec<Vec<usize>> = vec![];
        let mut width: usize = 0;
        let mut height: usize = 0;
        let mut start = (0, 0);
        let mut target = (0, 0);

        for (i, line) in input
            .split('\n')
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .enumerate()
        {
            if i == 0 {
                width = line.len() - 2;
                downwards = vec![vec![]; width];
                upwards = vec![vec![]; width];
                start = (
                    0,
                    line.as_bytes()
                        .iter()
                        .position(|&c| c == b'.')
                        .ok_or_else(|| InputParseError::new("missing start".into()))?,
                );
                println!("width={}, start={:?}", width, start);
                continue;
            } else if line.as_bytes().iter().filter(|&&c| c == b'#').count() == width + 1 {
                height = i - 1;
                target = (
                    i,
                    line.as_bytes()
                        .iter()
                        .position(|&c| c == b'.')
                        .ok_or_else(|| InputParseError::new("missing target".into()))?,
                );
                println!("height={}, target={:?}", height, target);
                break;
            } else {
                rightwards.push(vec![]);
                leftwards.push(vec![]);
                for (j, c) in line.as_bytes().iter().enumerate() {
                    match c {
                        b'>' => {
                            rightwards[i - 1].push(j - 1);
                        }
                        b'<' => {
                            leftwards[i - 1].push(j - 1);
                        }
                        b'^' => {
                            upwards[j - 1].push(i - 1);
                        }
                        b'v' => {
                            downwards[j - 1].push(i - 1);
                        }
                        _ => {}
                    }
                }
            }
        }

        let map = Map {
            width,
            height,
            rightwards,
            leftwards,
            downwards,
            upwards,
        };
        let path_finder = PathFinder { map, start, target };
        let steps = path_finder.find_path(0, start, target)?;

        Ok(Self {
            path_finder,
            steps_part1: steps,
        })
    }

    fn solve_part1(&self) -> anyhow::Result<usize> {
        Ok(self.steps_part1)
    }

    fn solve_part2(&self) -> anyhow::Result<Option<usize>> {
        let steps1 = self.path_finder.find_path(
            self.steps_part1,
            self.path_finder.target,
            self.path_finder.start,
        )?;
        let steps2 =
            self.path_finder
                .find_path(steps1, self.path_finder.start, self.path_finder.target)?;
        Ok(Some(steps2))
    }
}

impl PathFinder {
    fn check_pos(&self, timestep: usize, pos: (usize, usize)) -> bool {
        if 0 < pos.0 && pos.0 - 1 < self.map.height() && 0 < pos.1 && pos.1 - 1 < self.map.width() {
            !self.map.is_blocked(timestep, (pos.0 - 1, pos.1 - 1))
        } else {
            pos == self.start || pos == self.target
        }
    }

    fn find_path(
        &self,
        initial_steps: usize,
        start: (usize, usize),
        target: (usize, usize),
    ) -> anyhow::Result<usize> {
        let mut queue: VecDeque<(usize, (usize, usize))> = VecDeque::new();
        queue.push_back((initial_steps, start));
        while let Some((steps, pos)) = queue.pop_front() {
            if pos == target {
                return Ok(steps);
            }

            if pos.0 > 0 && self.check_pos(steps + 1, (pos.0 - 1, pos.1)) {
                let next = (steps + 1, (pos.0 - 1, pos.1));
                if !queue.contains(&next) {
                    queue.push_back(next);
                }
            }
            if self.check_pos(steps + 1, (pos.0 + 1, pos.1)) {
                let next = (steps + 1, (pos.0 + 1, pos.1));
                if !queue.contains(&next) {
                    queue.push_back(next);
                }
            }
            if pos.1 > 0 && self.check_pos(steps + 1, (pos.0, pos.1 - 1)) {
                let next = (steps + 1, (pos.0, pos.1 - 1));
                if !queue.contains(&next) {
                    queue.push_back(next);
                }
            }
            if self.check_pos(steps + 1, (pos.0, pos.1 + 1)) {
                let next = (steps + 1, (pos.0, pos.1 + 1));
                if !queue.contains(&next) {
                    queue.push_back(next);
                }
            }
            if self.check_pos(steps + 1, pos) {
                let next = (steps + 1, pos);
                if !queue.contains(&next) {
                    queue.push_back(next);
                }
            }
        }
        Err(anyhow::anyhow!("no solution"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = include_str!("examples/day24");
        test_example_input::<Solver, _, _>(input, 18, Some(54));
    }
}
