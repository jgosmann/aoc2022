use super::base::{AocAnswer, AocSolver};

pub fn test_example_input<Solver: AocSolver>(
    input: &str,
    part1_answer: AocAnswer,
    part2_answer: Option<AocAnswer>,
) {
    let mut input = input.split('\n').map(|line| String::from(line.trim()));
    let solution = Solver::new(&mut input).unwrap().solve().unwrap();
    assert_eq!(solution.part1.answer, part1_answer);
    if let Some(part2_answer) = part2_answer {
        assert_eq!(
            solution.part2.expect("missing part 2 solution").answer,
            part2_answer
        );
    }
}
