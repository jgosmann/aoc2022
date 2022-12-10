use super::base::AocSolver;

type CaloryCount = u64;

struct TopK<T, const K: usize> {
    size: usize,
    k_largest: [T; K],
}

impl<T: Copy + Default + PartialOrd, const K: usize> TopK<T, K> {
    fn new() -> Self {
        Self {
            size: 0,
            k_largest: [T::default(); K],
        }
    }

    fn push(&mut self, value: T) {
        let mut i = self.size;
        while i > 0 && self.k_largest[i - 1] <= value {
            if i < self.size {
                self.k_largest[i] = self.k_largest[i - 1];
            }
            i -= 1;
        }
        if i < K && self.k_largest[i] < value {
            self.k_largest[i] = value
        }

        if self.size < K {
            self.size += 1;
        }
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.k_largest.iter()
    }

    fn peek(&self) -> Option<&T> {
        self.k_largest.get(0)
    }
}

pub struct Solver {
    top_k_calories: TopK<CaloryCount, 3>,
}

impl AocSolver<u64, u64> for Solver {
    fn new<Iter: Iterator<Item = String>>(input: &mut Iter) -> anyhow::Result<Self> {
        let mut top_k_calories = TopK::new();
        let mut calories_carried: u64 = 0;

        for line in input {
            let line = line.trim();
            if line.is_empty() {
                top_k_calories.push(calories_carried);
                calories_carried = 0;
            } else {
                println!("<{:?}>", line);
                calories_carried += line.parse::<CaloryCount>().unwrap();
            }
        }
        Ok(Self { top_k_calories })
    }

    fn solve_part1(&self) -> anyhow::Result<u64> {
        Ok(self.top_k_calories.peek().copied().unwrap_or_default())
    }

    fn solve_part2(&self) -> anyhow::Result<Option<u64>> {
        Ok(Some(self.top_k_calories.iter().sum::<u64>()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = "\
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        ";
        test_example_input::<Solver, _, _>(input, 24000, Some(45000));
    }
}
