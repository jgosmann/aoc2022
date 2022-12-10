use super::base::AocSolver;

pub fn test_example_input<
    Solver: AocSolver<T1, T2>,
    T1: std::fmt::Debug + Eq,
    T2: std::fmt::Debug + Eq,
>(
    input: &str,
    part1_answer: T1,
    part2_answer: Option<T2>,
) {
    let mut input = input.split('\n').map(String::from);
    let solver = Solver::new(&mut input).unwrap();
    assert_eq!(solver.solve_part1().unwrap(), part1_answer);
    if let Some(part2_answer) = part2_answer {
        assert_eq!(solver.solve_part2().unwrap(), Some(part2_answer));
    }
}
