use std::collections::{HashMap, HashSet};

use super::base::AocSolver;

pub struct Solver {
    solution_part1: usize,
    solution_part2: usize,
}

fn count_empty_ground_tiles(elves: &HashSet<(isize, isize)>) -> usize {
    let row_min = elves.iter().map(|e| e.0).min().unwrap();
    let row_max = elves.iter().map(|e| e.0).max().unwrap();
    let col_min = elves.iter().map(|e| e.1).min().unwrap();
    let col_max = elves.iter().map(|e| e.1).max().unwrap();

    let area = (row_max - row_min + 1) as usize * (col_max - col_min + 1) as usize;

    area - elves.len()
}

impl<'a> AocSolver<'a, usize, usize> for Solver {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut elves = vec![];
        let mut row: usize = 0;
        let mut col: usize = 0;
        for c in input.as_bytes() {
            match c {
                b'#' => {
                    elves.push((row as isize, col as isize));
                    col += 1;
                }
                b'\n' => {
                    row += 1;
                    col = 0;
                }
                _ => {
                    col += 1;
                }
            }
        }

        let mut solution_part1 = 0;
        let mut elves: HashSet<(isize, isize)> = elves.into_iter().collect();
        let directions = [
            [(-1, 0), (-1, -1), (-1, 1)],
            [(1, 0), (1, -1), (1, 1)],
            [(0, -1), (-1, -1), (1, -1)],
            [(0, 1), (-1, 1), (1, 1)],
        ];
        let mut round: usize = 0;
        loop {
            round += 1;
            let mut proposals: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
            for elf in &elves {
                let mut proposed = false;
                if [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .iter()
                .any(|delta| elves.contains(&(elf.0 + delta.0, elf.1 + delta.1)))
                {
                    for dir_idx in 0..directions.len() {
                        let check_directions = directions[(dir_idx + round - 1) % directions.len()];
                        if check_directions
                            .iter()
                            .all(|dir| !elves.contains(&(elf.0 + dir.0, elf.1 + dir.1)))
                        {
                            let proposal =
                                (elf.0 + check_directions[0].0, elf.1 + check_directions[0].1);
                            let proposal_list = proposals.entry(proposal).or_insert_with(Vec::new);
                            proposal_list.push(*elf);
                            proposed = true;
                            break;
                        }
                    }
                }
                if !proposed {
                    if !proposals.contains_key(elf) {
                        proposals.insert(*elf, vec![]);
                    }
                    let proposal_list = proposals.get_mut(elf).unwrap();
                    proposal_list.push(*elf);
                }
            }

            let mut elves_new = HashSet::new();
            for (proposal, proposers) in proposals.iter() {
                if proposers.len() == 1 {
                    elves_new.insert(*proposal);
                } else {
                    for proposer in proposers {
                        elves_new.insert(*proposer);
                    }
                }
            }

            if round == 10 {
                solution_part1 = count_empty_ground_tiles(&elves_new);
            }
            if elves == elves_new {
                break;
            }
            elves = elves_new;
        }

        Ok(Self {
            solution_part1,
            solution_part2: round,
        })
    }

    fn solve_part1(&self) -> anyhow::Result<usize> {
        Ok(self.solution_part1)
    }

    fn solve_part2(&self) -> anyhow::Result<Option<usize>> {
        Ok(Some(self.solution_part2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = include_str!("examples/day23");
        test_example_input::<Solver, _, _>(input, 110, Some(20));
    }
}
