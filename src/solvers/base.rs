pub type AocAnswer = u64;

#[derive(Debug, Clone, Copy)]
pub struct AocPartSolution {
    pub name: &'static str,
    pub answer: AocAnswer,
}

#[derive(Debug, Clone, Copy)]
pub struct AocSolution {
    pub part1: AocPartSolution,
    pub part2: Option<AocPartSolution>,
}

pub trait AocSolver {
    fn new<Iter: Iterator<Item = String>>(input: &mut Iter) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn solve(&self) -> anyhow::Result<AocSolution>;
}
