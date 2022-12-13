use std::{collections::VecDeque, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete::one_of, streaming::multispace0},
    combinator,
    multi::{many1, separated_list0, separated_list1},
    number::complete,
    sequence::{preceded, tuple},
    IResult,
};

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

fn starting_items(i: &str) -> IResult<&str, VecDeque<u32>> {
    preceded(tag("Starting items: "), item_list)(i)
}

fn item_list(i: &str) -> IResult<&str, VecDeque<u32>> {
    let number_list = separated_list0(tag(", "), nom::character::complete::u32);
    combinator::map(number_list, |l| l.into())(i)
}

#[derive(Debug, PartialEq)]
enum Operation {
    Mul(Term, Term),
    Add(Term, Term),
}

fn parse_monkey(i: &str) -> IResult<&str, usize> {
    preceded(
        tag("Monkey: "),
        combinator::map(nom::character::complete::u32, |u| u as usize),
    )(i)
}

impl Operation {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, _) = tag("Operation: new = ")(i)?;
        let (i, (t1, op, t2)) = tuple((Term::parse, one_of("+*"), Term::parse))(i)?;
        match op {
            '+' => Ok((i, Self::Add(t1, t2))),
            '*' => Ok((i, Self::Mul(t1, t2))),
            _ => {
                unreachable!()
            }
        }
    }
}

// impl FromStr for Operation {
//     type Err;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         todo!()
//     }
// }

#[derive(Debug, PartialEq)]
enum Term {
    Old,
    Val(u32),
}

impl Term {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            combinator::map(tag("old"), |_| Self::Old),
            combinator::map(nom::character::complete::u32, |u| Self::Val(u)),
        ))(i)
    }
}

fn parse_usize(i: &str) -> IResult<&str, usize> {
    combinator::map(nom::character::complete::u32, |u| u as usize)(i)
}

fn parse_divisor(i: &str) -> IResult<&str, u32> {
    preceded(tag("Test: divisible by "), nom::character::complete::u32)(i)
}

fn parse_true(i: &str) -> IResult<&str, usize> {
    preceded(
        multispace0,
        preceded(tag("If true: throw to monkey "), parse_usize),
    )(i)
}
fn parse_false(i: &str) -> IResult<&str, usize> {
    preceded(
        multispace0,
        preceded(tag("If false: throw to monkey "), parse_usize),
    )(i)
}

#[derive(Debug, PartialEq)]
struct Monkey {
    items: VecDeque<u32>,
    op: Operation,
    test: u32,
    yes: usize,
    no: usize,
}

impl Monkey {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, number) = parse_monkey(i)?;
        let (i, items) = starting_items(i)?;
        let (i, op) = Operation::parse(i)?;
        let (i, test) = parse_divisor(i)?;
        let (i, yes) = parse_true(i)?;
        let (i, no) = parse_false(i)?;
        Ok((
            i,
            Self {
                items,
                op,
                test,
                yes,
                no,
            },
        ))
    }
}

#[cfg(test)]
mod tests {

    const TEST_CASE: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1";
}
