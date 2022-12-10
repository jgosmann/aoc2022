use super::base::AocSolver;

pub fn test_example_input<
    'a,
    Solver: AocSolver<'a, T1, T2>,
    T1: std::fmt::Debug + Eq,
    T2: std::fmt::Debug + Eq,
>(
    input: &'a str,
    part1_answer: T1,
    part2_answer: Option<T2>,
) {
    let solver = Solver::new(input).unwrap();
    assert_eq!(solver.solve_part1().unwrap(), part1_answer);
    if let Some(part2_answer) = part2_answer {
        assert_eq!(solver.solve_part2().unwrap(), Some(part2_answer));
    }
}
