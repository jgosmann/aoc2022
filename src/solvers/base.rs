pub trait AocSolver<T> {
    fn new<Iter: Iterator<Item = String>>(input: &mut Iter) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn solve_part1(&self) -> anyhow::Result<T>;
    fn solve_part2(&self) -> anyhow::Result<Option<T>>;
}
