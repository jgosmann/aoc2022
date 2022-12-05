use super::base::AocSolver;

pub fn test_example_input<Solver: AocSolver<T>, T: std::fmt::Debug + Eq>(
    input: &str,
    part1_answer: T,
    part2_answer: Option<T>,
) {
    let mut input = input.split('\n').map(String::from);
    let solver = Solver::new(&mut input).unwrap();
    assert_eq!(solver.solve_part1().unwrap(), part1_answer);
    if let Some(part2_answer) = part2_answer {
        assert_eq!(solver.solve_part2().unwrap(), Some(part2_answer));
    }
}
