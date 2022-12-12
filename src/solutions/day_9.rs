use std::{ops::{Add, AddAssign, Sub, SubAssign}, collections::HashSet};

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
        series_of_motions(input).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Point { x: 0, y: 1 },
            Direction::Down => Point { x: 0, y: -1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Rope {
    head: Point,
    tail: Point,
}

impl Rope {
    pub fn step(&mut self, step: Direction) {
        self.step_head(step);
        self.step_tail(step);
    }

    fn step_head(&mut self, step: Direction) {
        self.head += step.into();
    }
    fn step_tail(&mut self, step: Direction) {
        let diff_abs = (self.head - self.tail).abs();
        if diff_abs.x > 1 {
            // move after in x direction
            match diff_abs.y {
                0 => self.tail += step.into(),
                1 => {
                    self.tail += step.into();
                    self.tail.y = self.head.y
                }
                _ => unimplemented!(),
            };
        } else if diff_abs.y > 1 {
            // we are too far away in y direction
            match diff_abs.x {
                0 => self.tail += step.into(),
                1 => {
                    self.tail += step.into();
                    self.tail.x = self.head.x
                }
                _ => unimplemented!(),
            }
        }
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

fn series_of_motions(input: &str) -> usize {
    let mut rope = Rope::default();
    let mut set = HashSet::new();
    let it = input.lines().map(|l| parse_movement(l).unwrap().1);
    for m in it {
        for _ in 0..m.1 {
            rope.step(m.0);
            set.insert(rope.tail);
        }
    }
    set.len()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use assert_ok::assert_ok;

    use super::{parse_movement, series_of_motions, Point, Rope};

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

    #[test]
    fn test_movement() {
        let set = series_of_motions();

        assert_eq!(, 13)
    }
}
