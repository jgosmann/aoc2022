pub trait AocSolver<'a, T1, T2> {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn solve_part1(&self) -> anyhow::Result<T1>;
    fn solve_part2(&self) -> anyhow::Result<Option<T2>>;
}
