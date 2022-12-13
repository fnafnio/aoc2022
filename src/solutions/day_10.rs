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
    cycle: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Instruction {
    Noop,
    Addx(isize),
}

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::multispace1,
        combinator::{self, map},
        sequence::{pair, preceded, separated_pair},
        IResult,
    };


    fn parse_isize(i: &str) -> IResult<&str, isize> {
        combinator::map(nom::character::complete::i64, |n| n as _)(i)
    }

    fn command(i: &str) -> IResult<&str, Instruction> {
        let noop = map(tag("noop"), |_| Instruction::Noop);
        let addx = map(preceded(pair(tag("addx"), multispace1), parse_isize), |i| {
            Instruction::Addx(i)
        });
        alt((noop, addx))(i)
        // Ok((i, Instruction::Noop))
    }

#[cfg(test)]
mod tests {

    use test_case::test_case;
    use super::*;

    #[test_case("noop" => Instruction::Noop)]
    #[test_case("addx 3" => Instruction::Addx(3))]
    #[test_case("addx -5" => Instruction::Addx(-5))]
    fn test_parser(input: &str) -> Instruction {
        command(input).unwrap().1
    }


}
