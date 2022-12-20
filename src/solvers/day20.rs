use std::num::ParseIntError;

use super::base::AocSolver;

struct DoublyLinkedList<T> {
    values: Vec<T>,
    next: Vec<usize>,
    prev: Vec<usize>,
}

impl<T> DoublyLinkedList<T> {
    #[allow(clippy::needless_range_loop)]
    fn new(values: Vec<T>) -> Self {
        let mut next: Vec<usize> = (1..=values.len()).collect();
        *next.last_mut().unwrap() = 0;
        let mut prev: Vec<usize> = vec![0; values.len()];
        prev[0] = values.len() - 1;
        for i in 1..prev.len() {
            prev[i] = i - 1;
        }
        Self { values, next, prev }
    }

    fn move_by(&mut self, index: usize, dist: isize) {
        let mut dist = dist % (self.values.len() as isize - 1);
        self.next[self.prev[index]] = self.next[index];
        self.prev[self.next[index]] = self.prev[index];

        let mut insertion_point = self.next[index];
        while dist > 0 {
            insertion_point = self.next[insertion_point];
            dist -= 1;
        }
        while dist < 0 {
            insertion_point = self.prev[insertion_point];
            dist += 1;
        }

        self.next[index] = insertion_point;
        self.prev[index] = self.prev[insertion_point];
        self.next[self.prev[insertion_point]] = index;
        self.prev[insertion_point] = index;
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn values(&self) -> &[T] {
        &self.values
    }

    fn advance_by(&self, mut index: usize, mut dist: usize) -> usize {
        while dist > 0 {
            index = self.next[index];
            dist -= 1;
        }
        index
    }
}

pub struct Solver {
    input: Vec<isize>,
}

impl<'a> AocSolver<'a, isize, isize> for Solver {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let list = input
            .split('\n')
            .filter(|line| !line.trim().is_empty())
            .map(str::parse)
            .collect::<Result<Vec<isize>, ParseIntError>>()?;
        Ok(Self { input: list })
    }

    fn solve_part1(&self) -> anyhow::Result<isize> {
        let mut list = DoublyLinkedList::new(self.input.clone());
        for i in 0..list.len() {
            list.move_by(i, list.values()[i]);
        }
        Self::get_solution(&list)
    }

    fn solve_part2(&self) -> anyhow::Result<Option<isize>> {
        let mut list = DoublyLinkedList::new(self.input.iter().map(|v| v * 811589153).collect());
        for _ in 0..10 {
            for i in 0..list.len() {
                list.move_by(i, list.values()[i]);
            }
        }
        Ok(Some(Self::get_solution(&list)?))
    }
}

impl Solver {
    fn get_solution(list: &DoublyLinkedList<isize>) -> anyhow::Result<isize> {
        let zero_index = list
            .values()
            .iter()
            .enumerate()
            .find(|(_, &value)| value == 0)
            .map(|(i, _)| i)
            .ok_or_else(|| anyhow::anyhow!("no solution, missing 0"))?;
        let c1 = list.advance_by(zero_index, 1000);
        let c2 = list.advance_by(c1, 1000);
        let c3 = list.advance_by(c2, 1000);
        Ok(list.values()[c1] + list.values()[c2] + list.values()[c3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = include_str!("examples/day20");
        test_example_input::<Solver, _, _>(input, 3, Some(1623178306));
    }
}
