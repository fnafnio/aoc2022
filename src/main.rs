use std::path::PathBuf;

use aoc2022::{run_solver, Day, Part};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    day: usize,
    part: usize,
    path: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    use std::time::Instant;
    let total = Instant::now();

    color_eyre::install()?;
    let cli = Cli::parse();

    let day: Day = cli.day.try_into()?;
    let part: Part = cli.part.try_into()?;

    println!("Day {} Part {}", *day, part as usize);

    let input = std::fs::read_to_string(cli.path)?;
    let inner = Instant::now();

    let result = run_solver(day, part, &input);
    
    let inner = inner.elapsed();
    let elapsed = total.elapsed();
    println!("{}", result);
    println!("solver: {:.2?}", inner);
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
