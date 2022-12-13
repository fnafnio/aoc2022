use nom::{
    bytes::complete::tag,
    combinator,
    multi::{many1, separated_list0, separated_list1},
    number::complete,
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

fn parse_number(i: &str) -> IResult<&str, isize> {
    combinator::map(nom::character::complete::i64, |n| n as _)(i)
}

fn starting_items(i: &str) -> IResult<&str, Vec<usize>> {
    let number_list = separated_list0(
        tag(", "),
        combinator::map(complete::u64, |v| v::parse().unwrap()),
    );

    number_list(i)
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
