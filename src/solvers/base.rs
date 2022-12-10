pub trait AocSolver<T1, T2> {
    fn new<Iter: Iterator<Item = String>>(input: &mut Iter) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn solve_part1(&self) -> anyhow::Result<T1>;
    fn solve_part2(&self) -> anyhow::Result<Option<T2>>;
}
