use super::{base::AocSolver, error::InputParseError};
use anyhow::anyhow;

pub struct Solver {
    datastream: Vec<u8>,
}

impl AocSolver<usize> for Solver {
    fn new<Iter: Iterator<Item = String>>(input: &mut Iter) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            datastream: input
                .next()
                .ok_or_else(|| InputParseError::new("missing input".into()))?
                .as_bytes()
                .to_vec(),
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
                .map(|c| 1u32 << (c - b'a'))
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
        let mut input_iter = vec![String::from(input)].into_iter();
        assert_eq!(
            Solver::new(&mut input_iter).unwrap().solve_part1().unwrap(),
            answer
        );
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb\n", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz\n", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg\n", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg\n", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw\n", 26)]
    fn test_part2(#[case] input: &str, #[case] answer: usize) {
        let mut input_iter = vec![String::from(input)].into_iter();
        assert_eq!(
            Solver::new(&mut input_iter).unwrap().solve_part2().unwrap(),
            Some(answer)
        );
    }
}
