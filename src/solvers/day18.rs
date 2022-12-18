use std::{collections::VecDeque, num::ParseIntError};

use super::{base::AocSolver, error::InputParseError};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Classification {
    Air,
    Lava,
    TrappedAir,
}

pub struct Solver {
    surface: usize,
    surface_without_interior: usize,
}

impl<'a> AocSolver<'a, usize, usize> for Solver {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let voxels = input
            .split('\n')
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let voxel = line
                    .split(',')
                    .map(str::parse)
                    .collect::<Result<Vec<usize>, ParseIntError>>()?;
                Ok((voxel[0], voxel[1], voxel[2]))
            })
            .collect::<Result<Vec<(usize, usize, usize)>, ParseIntError>>()?;
        let max_dim = voxels
            .iter()
            .map(|voxel| voxel.0.max(voxel.1).max(voxel.2))
            .max()
            .ok_or_else(|| InputParseError::new("no input?".into()))?
            + 1;
        let mut scan = vec![vec![vec![Classification::TrappedAir; max_dim]; max_dim]; max_dim];
        for voxel in voxels {
            scan[voxel.0][voxel.1][voxel.2] = Classification::Lava;
        }

        let mut to_visit: VecDeque<(usize, usize, usize)> = VecDeque::new();
        to_visit.push_back((0, 0, 0));
        while let Some((x, y, z)) = to_visit.pop_front() {
            if scan[x][y][z] == Classification::TrappedAir {
                scan[x][y][z] = Classification::Air;
                if x > 0 {
                    to_visit.push_back((x - 1, y, z));
                }
                if y > 0 {
                    to_visit.push_back((x, y - 1, z));
                }
                if z > 0 {
                    to_visit.push_back((x, y, z - 1));
                }
                if x + 1 < scan.len() {
                    to_visit.push_back((x + 1, y, z));
                }
                if y + 1 < scan[x].len() {
                    to_visit.push_back((x, y + 1, z));
                }
                if z + 1 < scan[x][y].len() {
                    to_visit.push_back((x, y, z + 1));
                }
            }
        }

        let (surface, surface_interior) = Self::surface(&scan);
        Ok(Self {
            surface: surface + surface_interior,
            surface_without_interior: surface,
        })
    }

    fn solve_part1(&self) -> anyhow::Result<usize> {
        Ok(self.surface)
    }

    fn solve_part2(&self) -> anyhow::Result<Option<usize>> {
        Ok(Some(self.surface_without_interior))
    }
}

impl Solver {
    fn surface(scan: &Vec<Vec<Vec<Classification>>>) -> (usize, usize) {
        let mut count = 0;
        let mut count_interior = 0;
        for x in 0..scan.len() {
            for y in 0..scan[x].len() {
                for z in 0..scan[x][y].len() {
                    if scan[x][y][z] != Classification::Lava {
                        continue;
                    }
                    if x == 0 {
                        count += 1;
                    }
                    if y == 0 {
                        count += 1;
                    }
                    if z == 0 {
                        count += 1;
                    }
                    if x == scan.len() - 1 {
                        count += 1;
                    }
                    if y == scan[x].len() - 1 {
                        count += 1;
                    }
                    if z == scan[x][y].len() - 1 {
                        count += 1;
                    }
                    if x > 0 {
                        match scan[x - 1][y][z] {
                            Classification::Air => count += 1,
                            Classification::TrappedAir => count_interior += 1,
                            _ => (),
                        }
                    }
                    if y > 0 {
                        match scan[x][y - 1][z] {
                            Classification::Air => count += 1,
                            Classification::TrappedAir => count_interior += 1,
                            _ => (),
                        }
                    }
                    if z > 0 {
                        match scan[x][y][z - 1] {
                            Classification::Air => count += 1,
                            Classification::TrappedAir => count_interior += 1,
                            _ => (),
                        }
                    }
                    if x + 1 < scan.len() {
                        match scan[x + 1][y][z] {
                            Classification::Air => count += 1,
                            Classification::TrappedAir => count_interior += 1,
                            _ => (),
                        }
                    }
                    if y + 1 < scan[x].len() {
                        match scan[x][y + 1][z] {
                            Classification::Air => count += 1,
                            Classification::TrappedAir => count_interior += 1,
                            _ => (),
                        }
                    }
                    if z + 1 < scan[x][y].len() {
                        match scan[x][y][z + 1] {
                            Classification::Air => count += 1,
                            Classification::TrappedAir => count_interior += 1,
                            _ => (),
                        }
                    }
                }
            }
        }
        (count, count_interior)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = include_str!("examples/day18");
        test_example_input::<Solver, _, _>(input, 64, Some(58));
    }
}
