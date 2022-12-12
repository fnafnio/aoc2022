use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator,
    sequence::separated_pair,
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

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Movement(Direction, usize);

fn parse_direction(i: &str) -> IResult<&str, Direction> {
    use Direction::*;
    let up = combinator::map(tag("U"), |_| Up);
    let down = combinator::map(tag("D"), |_| Down);
    let left = combinator::map(tag("L"), |_| Left);
    let right = combinator::map(tag("R"), |_| Right);

    alt((up, down, left, right))(i)
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    combinator::map(nom::character::complete::u32, |n| n as _)(i)
}

fn parse_movement(i: &str) -> IResult<&str, Movement> {
    combinator::map(
        separated_pair(parse_direction, multispace1, parse_number),
        |(dir, range)| Movement(dir, range),
    )(i)
}

#[cfg(test)]
mod tests {
    use assert_ok::assert_ok;

    use super::parse_movement;

    const TEST: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_parser() {
        for line in TEST.lines() {
            let l = assert_ok!(parse_movement(line));
            println!("{:?}", l);
        }
    }
}
