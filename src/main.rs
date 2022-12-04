mod solvers;

use clap::Parser;
use std::io::{self, BufRead};

use solvers::{base::AocSolver, day01, day02, day03, day04};

#[derive(Parser, Debug)]
#[command(name = "aoc2022")]
#[command(author = "Jan Gosmann <jan@hyper-world.de>")]
#[command(about = "Solve Advent of Code 2022 puzzles.")]
struct Args {
    day: u8,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let stdin = io::stdin();
    let mut input = stdin.lock().lines().map(Result::unwrap);

    let solver: Box<dyn AocSolver> = match args.day {
        1 => Box::new(day01::Solver::new(&mut input)?),
        2 => Box::new(day02::Solver::new(&mut input)?),
        3 => Box::new(day03::Solver::new(&mut input)?),
        4 => Box::new(day04::Solver::new(&mut input)?),
        _ => panic!("invalid day"),
    };
    let solution = solver.solve()?;
    println!("{} {}", solution.part1.name, solution.part1.answer);
    if let Some(part2) = solution.part2 {
        println!("{} {}", part2.name, part2.answer);
    }
    Ok(())
}
