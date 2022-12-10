use super::base::AocSolver;
use anyhow::anyhow;

pub struct Solver {
    datastream: Vec<u32>,
}

impl AocSolver<'_, usize, usize> for Solver {
    fn new(input: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            datastream: input
                .as_bytes()
                .iter()
                .copied()
                .filter(|c| (b'a'..=b'z').contains(c))
                .map(|c| 1u32 << (c - b'a'))
                .collect(),
        })
    }

    fn solve_part1(&self) -> anyhow::Result<usize> {
        self.detect_marker(4)
            .ok_or_else(|| anyhow!("no start-of-packet marker found"))
    }

    fn solve_part2(&self) -> anyhow::Result<Option<usize>> {
        Ok(Some(self.detect_marker(14).ok_or_else(|| {
            anyhow!("no start-of-packet marker found")
        })?))
    }
}

impl Solver {
    fn detect_marker(&self, marker_length: usize) -> Option<usize> {
        for i in marker_length - 1..self.datastream.len() {
            let unique_char_count = self.datastream[i + 1 - marker_length..=i]
                .iter()
                .copied()
                .reduce(|accum, item| accum | item)
                .expect("no item in window")
                .count_ones();
            if unique_char_count as usize == marker_length {
                return Some(i + 1);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz\n", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg\n", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg\n", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw\n", 11)]
    fn test_part1(#[case] input: &str, #[case] answer: usize) {
        assert_eq!(Solver::new(input).unwrap().solve_part1().unwrap(), answer);
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb\n", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz\n", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg\n", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg\n", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw\n", 26)]
    fn test_part2(#[case] input: &str, #[case] answer: usize) {
        assert_eq!(
            Solver::new(input).unwrap().solve_part2().unwrap(),
            Some(answer)
        );
    }
}

#[cfg(all(feature = "bench", test))]
mod bench {
    extern crate test;
    use std::fs;

    use self::test::Bencher;
    use super::*;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = fs::read_to_string("./day6").unwrap();
        b.iter(|| {
            Solver::new(&input).unwrap().solve_part1().unwrap();
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = fs::read_to_string("./day6").unwrap();
        b.iter(|| {
            Solver::new(&input).unwrap().solve_part2().unwrap();
        });
    }
}
