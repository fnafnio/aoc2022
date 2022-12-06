#![feature(int_roundings)]
use std::{fmt::Display, ops::Deref};

use color_eyre::eyre;
use eyre::{anyhow, Error};

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

pub enum ParsingErrors {
    InvalidDay(String),
    InvalidPart(String),
}

#[derive(Debug, Clone, Copy)]
pub enum Part {
    Part1 = 1,
    Part2 = 2,
}

impl TryFrom<usize> for Part {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Part::Part1),
            2 => Ok(Part::Part2),
            _ => Err(anyhow!("Part can only be 1 or 2")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Day(usize);

impl Day {
    pub fn index(&self) -> usize {
        self.0 - 1
    }
}

impl TryFrom<usize> for Day {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Err(anyhow!("So, day 0 you say?")),
            x @ 1..=25 => Ok(Day(x)),
            _ => Err(anyhow!("Missed Christmas this year?")),
        }
    }
}

impl std::ops::Deref for Day {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
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

pub fn run_solver(day: Day, part: Part, input: &str) -> String {
    // assert!(day < SOLVERS.len() && day > 0);
    // let day = day - 1;

    SOLVERS[day.index()].run_part(input, part)
}
