use std::{collections::VecDeque, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, one_of},
    combinator,
    multi::{many1, separated_list0, separated_list1},
    number::complete,
    sequence::{delimited, preceded, terminated, tuple},
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
    combinator::map(number_list, |l| VecDeque::from(l))(i)
}

#[derive(Debug, PartialEq)]
enum Operation {
    Mul(Term, Term),
    Add(Term, Term),
}

fn parse_monkey(i: &str) -> IResult<&str, usize> {
    delimited(
        tag("Monkey "),
        combinator::map(nom::character::complete::u32, |u| u as usize),
        tag(":"),
    )(i)
}

impl Operation {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, _) = tag("Operation: new = ")(i)?;
        let (i, (t1, op, t2)) = tuple((
            Term::parse,
            delimited(multispace0, one_of("+*"), multispace0),
            Term::parse,
        ))(i)?;
        match op {
            '+' => Ok((i, Self::Add(t1, t2))),
            '*' => Ok((i, Self::Mul(t1, t2))),
            _ => {
                unreachable!()
            }
        }
    }
}

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
        let (i, number) = terminated(parse_monkey, multispace0)(i)?;
        println!("number: {number}");
        let (i, items) = terminated(starting_items, multispace0)(i)?;
        println!("items: {items:?}");
        let (i, op) = terminated(Operation::parse, multispace0)(i)?;
        println!("op: {op:?}");
        let (i, test) = terminated(parse_divisor, multispace0)(i)?;
        println!("test: {test}");
        let (i, yes) = terminated(parse_true, multispace0)(i)?;
        println!("yes: {yes}");
        let (i, no) = terminated(parse_false, multispace0)(i)?;
        println!("no: {no}");
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

fn parse_monkey_list(i: &str) -> IResult<&str, Vec<Monkey>> {
    many1(Monkey::parse)(i)
}

#[cfg(test)]
mod tests {
    use assert_ok::assert_ok;

    use super::*;

    const TEST_CASE: &str = r"Monkey 0:
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

    #[test]
    fn test_monkey() {
        // let i = TEST_CASE.split("\n\n").next().unwrap();
        let i = "Monkey 0:\nStarting items: 79, 98\nOperation: new = old * 19\nTest: divisible by 23\nIf true: throw to monkey 2\nIf false: throw to monkey 3\n";
        let (i, m) = assert_ok!(Monkey::parse(i));
        println!("Monkey -> {:?}", m);
    }

    #[test]
    fn test_parse_false() {
        let i = "If false: throw to monkey 3";
        let (i, u) = assert_ok!(parse_false(i));
    }

    #[test]
    fn test_starting_items() {
        let i = "Starting items: 79, 98";
        let (i, items) = assert_ok!(starting_items(i));
        println!("i: {}\n\n", i);
        println!("items: {:?}", items);
    }

    #[test]
    fn test_operation() {
        let i = "Operation: new = old * 19";
        let (i, o) = assert_ok!(Operation::parse(i));
    }

    #[test]
    fn test_monkey_number() {
        let i = TEST_CASE.lines().next().unwrap();
        let (i, u) = assert_ok!(parse_monkey(i));
        assert_eq!(u, 0);
    }

    #[test]
    fn test_all_monkeys() {
        let x = assert_ok::assert_ok!(parse_monkey_list(TEST_CASE));
        println!("{}\n", x.0);
        println!("{:?}", x.1);
    }
}
