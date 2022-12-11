#![cfg_attr(feature = "bench", feature(test))]

mod solvers;

use ansi_term::Style;
use anyhow::Context;
use clap::Parser;
use std::{fmt::Display, marker::PhantomData, time::Instant};

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
    fn solve_part1(&self) -> anyhow::Result<Box<dyn Display + Send>>;
    fn solve_part2(&self) -> anyhow::Result<Option<Box<dyn Display + Send>>>;
}

struct DisplayDecorator<'a, S: AocSolver<'a, T1, T2>, T1, T2> {
    solver: S,
    input_lifetime: PhantomData<&'a [u8]>,
    answer_type1: PhantomData<T1>,
    answer_type2: PhantomData<T2>,
}

impl<'a, S: AocSolver<'a, T1, T2>, T1: Display + Send + 'static, T2: Display + Send + 'static>
    SolveDisplayable for DisplayDecorator<'a, S, T1, T2>
{
    fn solve_part1(&self) -> anyhow::Result<Box<dyn Display + Send>> {
        Ok(Box::new(self.solver.solve_part1()?))
    }

    fn solve_part2(&self) -> anyhow::Result<Option<Box<dyn Display + Send>>> {
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

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let time_start = Instant::now();
    let args = Args::parse();

    if let Some(day) = args.day {
        solve_day(day).await?;
    } else {
        let tasks: Vec<_> = (1..=11)
            .map(|day| tokio::spawn(async move { solve_day(day).await }))
            .collect();
        for task in tasks {
            print!("{}", task.await??);
        }
    }

    println!("Took {:?}", time_start.elapsed());
    Ok(())
}

struct SolvedDay {
    day: u8,
    time_start: Instant,
    time_read_finished: Instant,
    time_preprocess_finished: Instant,
    time_part1_solved: Instant,
    time_part2_solved: Instant,
    solution_part1: Box<dyn Display + Send>,
    solution_part2: Option<Box<dyn Display + Send>>,
}

impl Display for SolvedDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Style::new()
            .underline()
            .bold()
            .paint(format!("Day {}\n", self.day))
            .fmt(f)?;
        f.write_fmt(format_args!(
            "Input read in {:?}\n",
            self.time_read_finished.duration_since(self.time_start)
        ))?;
        f.write_fmt(format_args!(
            "Preprocessing finished in {:?}\n\n",
            self.time_preprocess_finished
                .duration_since(self.time_read_finished)
        ))?;
        f.write_fmt(format_args!(
            "Solution part 1 ({:?}):\n{}\n\n",
            self.time_part1_solved
                .duration_since(self.time_preprocess_finished),
            Style::new().bold().paint(self.solution_part1.to_string())
        ))?;
        if let Some(part2) = &self.solution_part2 {
            f.write_fmt(format_args!(
                "Solution part 2 ({:?}):\n{}\n\n",
                self.time_part2_solved
                    .duration_since(self.time_preprocess_finished),
                Style::new().bold().paint(part2.to_string())
            ))?;
        }
        Ok(())
    }
}

async fn solve_day(day: u8) -> anyhow::Result<SolvedDay> {
    let time_start = Instant::now();
    let input_path = format!("./day{:0>2}", day);
    let input = tokio::fs::read(&input_path).await.context(input_path)?;
    let input = std::str::from_utf8(&input)?;
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

    Ok(SolvedDay {
        day,
        time_start,
        time_read_finished,
        time_preprocess_finished,
        time_part1_solved,
        time_part2_solved,
        solution_part1,
        solution_part2,
    })
}
