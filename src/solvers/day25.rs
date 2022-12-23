use super::base::AocSolver;

pub struct Solver {
    solution: String,
}

fn parse_snafu(snafu: &str) -> isize {
    let mut current = 0;
    for c in snafu.chars() {
        current *= 5;
        current += match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => 0,
        };
    }
    current
}

fn to_snafu(mut value: isize) -> String {
    let mut digits = vec![];
    let mut carry = 0;
    while value > 0 || carry > 0 {
        let digit = value % 5 + carry;
        value /= 5;
        if digit > 2 {
            carry = 1;
            digits.push(digit - 5)
        } else {
            carry = 0;
            digits.push(digit);
        }
    }
    digits
        .iter()
        .rev()
        .map(|digit| match digit {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        })
        .collect()
}

impl<'a> AocSolver<'a, String, String> for Solver {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            solution: to_snafu(input.split('\n').map(parse_snafu).sum()),
        })
    }

    fn solve_part1(&self) -> anyhow::Result<String> {
        Ok(self.solution.clone())
    }

    fn solve_part2(&self) -> anyhow::Result<Option<String>> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = include_str!("examples/day25");
        test_example_input::<Solver, _, _>(input, "2=-1=0".into(), None);
    }
}
