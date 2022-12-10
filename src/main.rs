#![cfg_attr(feature = "bench", feature(test))]

mod solvers;

use clap::Parser;
use std::{fmt::Display, fs, marker::PhantomData};

use solvers::{
    base::AocSolver, day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
};

#[derive(Parser, Debug)]
#[command(name = "aoc2022")]
#[command(author = "Jan Gosmann <jan@hyper-world.de>")]
#[command(about = "Solve Advent of Code 2022 puzzles.")]
struct Args {
    day: u8,
}

trait SolveDisplayable {
    fn solve_part1(&self) -> anyhow::Result<Box<dyn Display>>;
    fn solve_part2(&self) -> anyhow::Result<Option<Box<dyn Display>>>;
}

struct DisplayDecorator<'a, S: AocSolver<'a, T1, T2>, T1, T2> {
    solver: S,
    input_lifetime: PhantomData<&'a [u8]>,
    answer_type1: PhantomData<T1>,
    answer_type2: PhantomData<T2>,
}

impl<'a, S: AocSolver<'a, T1, T2>, T1: Display + 'static, T2: Display + 'static> SolveDisplayable
    for DisplayDecorator<'a, S, T1, T2>
{
    fn solve_part1(&self) -> anyhow::Result<Box<dyn Display>> {
        Ok(Box::new(self.solver.solve_part1()?))
    }

    fn solve_part2(&self) -> anyhow::Result<Option<Box<dyn Display>>> {
        if let Some(answer) = self.solver.solve_part2()? {
            Ok(Some(Box::new(answer)))
        } else {
            Ok(None)
        }
    }
}

impl<'a, S: AocSolver<'a, T1, T2>, T1, T2> From<S> for DisplayDecorator<'a, S, T1, T2> {
    fn from(solver: S) -> Self {
        Self {
            solver,
            input_lifetime: PhantomData,
            answer_type1: PhantomData,
            answer_type2: PhantomData,
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    //let stdin = io::stdin();
    let input = fs::read_to_string("./day6")?;

    let solver: Box<dyn SolveDisplayable> = match args.day {
        1 => Box::<DisplayDecorator<_, _, _>>::new(day01::Solver::new(&input)?.into()),
        2 => Box::<DisplayDecorator<_, _, _>>::new(day02::Solver::new(&input)?.into()),
        3 => Box::<DisplayDecorator<_, _, _>>::new(day03::Solver::new(&input)?.into()),
        4 => Box::<DisplayDecorator<_, _, _>>::new(day04::Solver::new(&input)?.into()),
        5 => Box::<DisplayDecorator<_, _, _>>::new(day05::Solver::new(&input)?.into()),
        6 => Box::<DisplayDecorator<_, _, _>>::new(day06::Solver::new(&input)?.into()),
        7 => Box::<DisplayDecorator<_, _, _>>::new(day07::Solver::new(&input)?.into()),
        8 => Box::<DisplayDecorator<_, _, _>>::new(day08::Solver::new(&input)?.into()),
        9 => Box::<DisplayDecorator<_, _, _>>::new(day09::Solver::new(&input)?.into()),
        10 => Box::<DisplayDecorator<_, _, _>>::new(day10::Solver::new(&input)?.into()),
        _ => panic!("invalid day"),
    };

    println!("{}", solver.solve_part1()?);
    if let Some(part2) = solver.solve_part2()? {
        println!("{}", part2);
    }
    Ok(())
}
