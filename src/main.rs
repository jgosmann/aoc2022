#![cfg_attr(feature = "bench", feature(test))]

mod solvers;

use clap::Parser;
use std::{
    fmt::Display,
    io::{self, BufRead},
    marker::PhantomData,
};

use solvers::{base::AocSolver, day01, day02, day03, day04, day05, day06, day07, day08, day09};

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

struct DisplayDecorator<S: AocSolver<T>, T> {
    solver: S,
    answer_type: PhantomData<T>,
}

impl<S: AocSolver<T>, T: Display + 'static> SolveDisplayable for DisplayDecorator<S, T> {
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

impl<S: AocSolver<T>, T> From<S> for DisplayDecorator<S, T> {
    fn from(solver: S) -> Self {
        Self {
            solver,
            answer_type: PhantomData,
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let stdin = io::stdin();
    let mut input = stdin.lock().lines().map(Result::unwrap);

    let solver: Box<dyn SolveDisplayable> = match args.day {
        1 => Box::<DisplayDecorator<_, _>>::new(day01::Solver::new(&mut input)?.into()),
        2 => Box::<DisplayDecorator<_, _>>::new(day02::Solver::new(&mut input)?.into()),
        3 => Box::<DisplayDecorator<_, _>>::new(day03::Solver::new(&mut input)?.into()),
        4 => Box::<DisplayDecorator<_, _>>::new(day04::Solver::new(&mut input)?.into()),
        5 => Box::<DisplayDecorator<_, _>>::new(day05::Solver::new(&mut input)?.into()),
        6 => Box::<DisplayDecorator<_, _>>::new(day06::Solver::new(&mut input)?.into()),
        7 => Box::<DisplayDecorator<_, _>>::new(day07::Solver::new(&mut input)?.into()),
        8 => Box::<DisplayDecorator<_, _>>::new(day08::Solver::new(&mut input)?.into()),
        9 => Box::<DisplayDecorator<_, _>>::new(day09::Solver::new(&mut input)?.into()),
        _ => panic!("invalid day"),
    };

    println!("{}", solver.solve_part1()?);
    if let Some(part2) = solver.solve_part2()? {
        println!("{}", part2);
    }
    Ok(())
}
