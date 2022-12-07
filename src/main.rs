use aoc2022::{run_solver, Day, Part};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    day: usize,
    part: usize,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let day: Day = cli.day.try_into()?;
    let part: Part = cli.part.try_into()?;

    println!("Day {} Part {}", *day, part as usize);

    let input = include_str!("../input/day_6/input");
    let result = run_solver(day, part, input);

    println!("{}", result);

    Ok(())
}
