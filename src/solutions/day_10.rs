use core::fmt;
use std::collections::VecDeque;

use crate::Solver;

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> String {
        todo!()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}

struct Cpu {
    program: VecDeque<Instruction>,
    current: Option<(Instruction, usize)>,
    cycle: usize,
    reg_x: isize,
}

impl Cpu {
    fn new(mut program: VecDeque<Instruction>) -> Self {
        let current = program.pop_front().map(|i| (i, i.cycles()));
        Self {
            program,
            current,
            cycle: 0,
            reg_x: 1,
        }
    }

    fn fetch(&mut self) {
        self.current = self.program.pop_front().map(|i| (i, i.cycles()));
    }

    fn step(&mut self) -> bool {
        if let Some((cur, cycles)) = self.current.as_mut() {
            *cycles -= 1;
            if *cycles == 0 {
                match cur {
                    Instruction::Noop => {}
                    Instruction::Addx(acc) => self.reg_x += *acc,
                }
                self.fetch();
            }
            self.cycle += 1;
            true
        } else {
            false
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cycle={} x={} current={:?} ({} instructions left)",
            self.cycle,
            self.reg_x,
            self.current,
            self.program.len()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Instruction {
    Noop,
    Addx(isize),
}

impl Instruction {
    fn parse(i: &str) -> IResult<&str, Self> {
        {
            let noop = value(Instruction::Noop, tag("noop"));
            let addx = map(preceded(pair(tag("addx"), multispace1), parse_isize), |i| {
                Instruction::Addx(i)
            });
            alt((noop, addx))(i)
        }
    }

    fn cycles(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    combinator::{self, map, value},
    sequence::{pair, preceded, separated_pair},
    IResult,
};

fn parse_isize(i: &str) -> IResult<&str, isize> {
    combinator::map(nom::character::complete::i64, |n| n as _)(i)
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    const CASE_1: &str = "noop\naddx 3\naddx -5";

    #[test]
    fn test_cpu() {
        let program = CASE_1
            .lines()
            .map(|l| Instruction::parse(l).unwrap().1)
            .collect();
        let mut cpu = Cpu::new(program);

        assert!(cpu.step());
        assert_eq!((cpu.cycle, cpu.reg_x), (1, 1));

        assert!(cpu.step());
        assert_eq!((cpu.cycle, cpu.reg_x), (2, 1));
        assert!(cpu.step());
        assert_eq!((cpu.cycle, cpu.reg_x), (3, 4));

        assert!(cpu.step());
        assert_eq!((cpu.cycle, cpu.reg_x), (4, 4));
        assert!(cpu.step());
        assert_eq!((cpu.cycle, cpu.reg_x), (5, -1));
    }

    #[test_case("noop" => Instruction::Noop)]
    #[test_case("addx 3" => Instruction::Addx(3))]
    #[test_case("addx -5" => Instruction::Addx(-5))]
    fn test_parser(i: &str) -> Instruction {
        Instruction::parse(i).unwrap().1
    }
}
