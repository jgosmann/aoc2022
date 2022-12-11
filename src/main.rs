#![cfg_attr(feature = "bench", feature(test))]

mod solvers;

use ansi_term::Style;
use anyhow::Context;
use clap::Parser;
use std::{fmt::Display, fs, marker::PhantomData, time::Instant};

use solvers::{
    base::AocSolver, day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11,
};

#[derive(Parser, Debug)]
#[command(name = "aoc2022")]
#[command(author = "Jan Gosmann <jan@hyper-world.de>")]
#[command(about = "Solve Advent of Code 2022 puzzles.")]
struct Args {
    day: Option<u8>,
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

fn main() -> anyhow::Result<()> {
    let time_start = Instant::now();
    let args = Args::parse();

    if let Some(day) = args.day {
        solve_day(day)?;
    } else {
        for day in 1..=11 {
            solve_day(day)?;
            println!();
            println!();
        }
    }

    println!("Took {:?}", time_start.elapsed());
    Ok(())
}

fn solve_day(day: u8) -> anyhow::Result<()> {
    let time_start = Instant::now();
    let input_file = format!("./day{:0>2}", day);
    let input = fs::read_to_string(&input_file).context(input_file)?;
    let time_read_finished = Instant::now();

    let solver: Box<dyn SolveDisplayable> = match day {
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
        11 => Box::<DisplayDecorator<_, _, _>>::new(day11::Solver::new(&input)?.into()),
        _ => panic!("invalid day"),
    };
    let time_preprocess_finished = Instant::now();

    let solution_part1 = solver.solve_part1()?;
    let time_part1_solved = Instant::now();

    let solution_part2 = solver.solve_part2()?;
    let time_part2_solved = Instant::now();

    println!(
        "{}",
        Style::new()
            .underline()
            .bold()
            .paint(format!("Day {}", day))
    );
    println!(
        "Input read in {:?}",
        time_read_finished.duration_since(time_start)
    );
    println!(
        "Preprocessing finished in {:?}",
        time_preprocess_finished.duration_since(time_read_finished)
    );

    println!();
    println!(
        "Solution part 1 ({:?}):",
        time_part1_solved.duration_since(time_preprocess_finished)
    );
    println!("{}", Style::new().bold().paint(solution_part1.to_string()));

    println!();
    if let Some(part2) = solution_part2 {
        println!(
            "Solution part 2 ({:?}):",
            time_part2_solved.duration_since(time_preprocess_finished)
        );
        println!("{}", Style::new().bold().paint(part2.to_string()));
    }
    Ok(())
}
