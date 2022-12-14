use std::ops::Rem;

use itertools::Itertools;
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
        solve_rounds(input, 20, 3).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        solve_rounds(input, 10000, 1).to_string()
    }
}

fn starting_items(i: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("Starting items: "), item_list)(i)
}

fn item_list(i: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(tag(", "), nom::character::complete::u64)(i)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    Mul(Term, Term),
    Add(Term, Term),
}

fn parse_monkey(i: &str) -> IResult<&str, usize> {
    delimited(
        tag("Monkey "),
        combinator::map(nom::character::complete::u64, |u| u as usize),
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

    fn eval(self, old: u64) -> u64 {
        match self {
            Operation::Mul(x, y) => x.eval(old) * y.eval(old),
            Operation::Add(x, y) => x.eval(old) + y.eval(old),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Term {
    Old,
    Val(u64),
}

impl Term {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            combinator::map(tag("old"), |_| Self::Old),
            combinator::map(nom::character::complete::u64, |u| Self::Val(u)),
        ))(i)
    }

    fn eval(self, old: u64) -> u64 {
        match self {
            Term::Old => old,
            Term::Val(v) => v,
        }
    }
}

fn parse_usize(i: &str) -> IResult<&str, usize> {
    combinator::map(nom::character::complete::u64, |u| u as usize)(i)
}

fn parse_divisor(i: &str) -> IResult<&str, u64> {
    preceded(tag("Test: divisible by "), nom::character::complete::u64)(i)
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
    inspected: u64,
    items: Vec<u64>,
    op: Operation,
    test: u64,
    yes: usize,
    no: usize,
}

impl Monkey {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, number) = terminated(parse_monkey, multispace0)(i)?;
        let (i, items) = terminated(starting_items, multispace0)(i)?;
        let (i, op) = terminated(Operation::parse, multispace0)(i)?;
        let (i, test) = terminated(parse_divisor, multispace0)(i)?;
        let (i, yes) = terminated(parse_true, multispace0)(i)?;
        let (i, no) = terminated(parse_false, multispace0)(i)?;

        Ok((
            i,
            Self {
                inspected: 0,
                items,
                op,
                test,
                yes,
                no,
            },
        ))
    }

    fn catch(&mut self, item: u64) {
        self.items.push(item);
    }

    fn inspect_items(&mut self, worry_div: u64) -> Vec<(usize, u64)> {
        assert_ne!(worry_div, 0);

        let mut v = vec![];
        for &i in self.items.iter() {
            let new = (self.op.eval(i) / worry_div);
            let m = self.test(new);
            v.push((m, new));
            self.inspected += 1;
        }
        self.items.clear();
        v
    }

    fn test(&self, item: u64) -> usize {
        match item.rem(self.test) == 0 {
            true => self.yes,
            false => self.no,
        }
    }
}

fn solve_rounds(i: &str, rounds: usize, worry_div: u64) -> u64 {
    let (_, mut monkeys) = parse_monkey_list(i).expect("something wrong with parsing");
    let mut items: Vec<(usize, u64)> = vec![];
    let divisor_product = monkeys.iter().map(|m| m.test).product();

    dbg!(divisor_product);

    for _ in 0..rounds {
        do_round(&mut monkeys, worry_div, divisor_product);
    }

    let (x, y) = monkeys
        .iter()
        .sorted_by(|b, a| a.inspected.cmp(&b.inspected))
        .tuples()
        .next()
        .unwrap();
    x.inspected * y.inspected
}

fn do_round(monkeys: &mut Vec<Monkey>, worry_div: u64, divisor_product: u64) {
    for i in 0..monkeys.len() {
        let items = {
            let monkey = &mut monkeys[i];
            monkey.inspect_items(worry_div)
        };
        for (m, item) in items.into_iter() {
            monkeys[m].catch(item.rem(divisor_product));
        }
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
    fn test_solver_part1() {
        assert_eq!(solve_rounds(TEST_CASE), 10605)
    }

    #[test]
    fn test_monkey() {
        // let i = TEST_CASE.split("\n\n").next().unwrap();
        let i = "Monkey 0:\nStarting items: 79, 98\nOperation: new = old * 19\nTest: divisible by 23\nIf true: throw to monkey 2\nIf false: throw to monkey 3\n";
        let (i, m) = assert_ok!(Monkey::parse(i));
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
    }
}
