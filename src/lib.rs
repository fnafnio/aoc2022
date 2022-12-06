#![feature(int_roundings)]

use day_1::Day1;
use day_2::Day2;
use day_3::Day3;
use day_4::Day4;
use day_5::Day5;
use day_6::Day6;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;

#[derive(Debug)]
pub enum Part {
    Part1,
    Part2,
}

impl TryFrom<usize> for Part {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Part::Part1),
            2 => Ok(Part::Part2),
            _ => Err("Part can only be 1 or 2"),
        }
    }
}

pub struct Days {
    solvers: Vec<Box<dyn Solver>>,
}

impl Days {
    pub fn new() -> Self {
        let mut solvers = Vec::new();
        Self { solvers }
    }
}

pub trait Solver {
    fn part_1(&self, input: &str) -> String;
    fn part_2(&self, input: &str) -> String;

    fn run_part(&self, input: &str, part: Part) -> String {
        match part {
            Part::Part1 => self.part_1(input),
            Part::Part2 => self.part_2(input),
        }
    }
}

const SOLVERS: &[&dyn Solver] = &[&Day1, &Day2, &Day3, &Day4, &Day5, &Day6];

pub fn run_solver(day: usize, input: &str, part: Part) -> String {
    assert!(day < SOLVERS.len() && day > 0);
    let day = day - 1;

    SOLVERS[day].run_part(input, part)
}
