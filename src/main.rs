use aoc2022::{day_1, day_2, day_3, day_4, day_5, day_6, run_solver, Part};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    day: usize,
    part: Part,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    println!("{:?}", cli);

    let input = include_str!("../input/day_6/input");
    // aoc2022::run_solver(6, "bla");
    run_solver(6, input, part);

    run_all_solutions();

    Ok(())
}

fn run_all_solutions() {
    println!("---------------------------------");
    day_1::day_1();
    println!("---------------------------------");
    day_2::day_2();
    println!("---------------------------------");
    day_3::day_3();
    println!("---------------------------------");
    day_4::day_4();
    println!("---------------------------------");
    day_5::day_5();
    println!("---------------------------------");
    day_6::day_6();
    println!("---------------------------------");
}

fn run_solution(day: usize, part: usize) {
    match day {
        1 => {}
        2 => {}
        3 => {}
        4 => {}
        5 => {}
        6 => {}
        7 => {}
        8 => {}
        9 => {}
        10 => {}
        11 => {}
        12 => {}
        13 => {}
        14 => {}
        15 => {}
        16 => {}
        17 => {}
        18 => {}
        19 => {}
        20 => {}
        21 => {}
        22 => {}
        23 => {}
        24 => {}
        _ => {}
    }
}
