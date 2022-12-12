use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign},
};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::{self, all_consuming},
    sequence::separated_pair,
    IResult,
};

use crate::Solver;

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> String {
        series_of_motions(input, 1).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}

#[derive(Debug)]
struct Grid {
    x: usize,
    y: usize,
    field: Vec<String>,
}

impl Grid {
    fn new(x: usize, y: usize) -> Self {
        // let field: Vec<char> = vec!['.'; x * y];
        let field = vec![str::repeat(".", x * y)];
        Self { x, y, field }
    }

    fn set_coord(&mut self, p: &Point, new: char) -> Option<()> {
        let line: &mut String = self
            .field
            .get_mut(usize::try_from(p.y).expect("index is negative"))
            .unwrap();

        line.rep

        None

        // if let Some(c) = self.field.get_mut(self.x * p.y as usize + p.x as usize) {
        //     *c = new;
        // }
    }

    fn set_relative(&mut self, p: &Point, rel: &Point, new: char) {
        let set = *p + *rel;
        self.set_coord(&set, new);
    }

    fn draw(&self) -> String {
        let mut result = String::with_capacity((self.x * self.y) as _);
        self.field
            .iter()
            .chunks(self.x as _)
            .into_iter()
            .for_each(|chunk| {
                let mut s: String = chunk.collect();
                result.push_str(&s);
                result.push('\n');
            });
        result
    }
}

fn draw_field(r: &Rope, ll: &Point, ur: &Point) {
    let span = (*ur - *ll).abs();
    let mut g = Grid::new(span.x.try_into().unwrap(), span.y.try_into().unwrap());

    for (knot, new) in r.knots.iter().zip('0'..'9') {
        g.set_coord(knot, new);
    }
    print!("{}", g.draw());
    println!("");
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

    fn step_head(&mut self, step: Direction) {
        *self += step.into();
    }

    fn step_tail(&mut self, step: Direction, head: &Point) {
        let diff_abs = (*head - *self).abs();
        if diff_abs.x > 1 {
            // move after in x direction
            match diff_abs.y {
                0 => *self += step.into(),
                1 => {
                    *self += step.into();
                    self.y = head.y
                }
                _ => unimplemented!(),
            };
        } else if diff_abs.y > 1 {
            // we are too far away in y direction
            match diff_abs.x {
                0 => *self += step.into(),
                1 => {
                    *self += step.into();
                    self.x = head.x
                }
                _ => unimplemented!(),
            }
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

#[derive(Clone, Debug)]
struct Rope {
    knots: Vec<Point>,
}

impl Default for Rope {
    fn default() -> Self {
        Self::new_with_len(1)
    }
}

const LL: Point = Point { x: -5, y: -5 };
const UR: Point = Point { x: 15, y: 15 };

impl Rope {
    fn new_with_len(l: usize) -> Self {
        assert!(l > 0);

        let mut rope = Self {
            knots: vec![Default::default(); l + 1],
        };
        rope
    }

    pub fn step(&mut self, step: Direction) {
        let mut it = self.knots.iter_mut();

        let head = it.next().unwrap();
        head.step_head(step);
        let mut prev = *head;

        for t in it {
            t.step_tail(step, &prev);
            prev = *t;
        }
    }

    fn get_head_mut(&mut self) -> &mut Point {
        self.knots.first_mut().expect("Tails vector is empty!")
    }

    fn get_head(&self) -> &Point {
        self.knots.first().expect("Tails vector is empty!")
    }

    fn get_tail(&self) -> &Point {
        self.knots.last().expect("Tails vector is empty!")
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
struct Movement {
    dir: Direction,
    steps: usize,
}

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
        |(dir, range)| Movement { dir, steps: range },
    )(i)
}

fn series_of_motions(input: &str, length: usize) -> usize {
    let mut rope = Rope::new_with_len(length);
    let mut set = HashSet::new();

    let it = input
        .lines()
        .map(|l| all_consuming(parse_movement)(l).unwrap().1);

    for m in it {
        for _ in 0..m.steps {
            rope.step(m.dir);
            set.insert(rope.get_tail().clone());
        }
    }
    set.len()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use test_case::test_case;

    use assert_ok::assert_ok;

    use super::{parse_movement, series_of_motions, Point, Rope};

    const TEST_SHORT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const TEST_LONG: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_parser() {
        for line in TEST_SHORT.lines() {
            let l = assert_ok!(parse_movement(line));
            println!("{:?}", l);
        }
    }

    #[test_case(TEST_SHORT => 13)]
    #[test_case(TEST_LONG => 88)]
    fn short_rope(input: &str) -> usize {
        series_of_motions(input, 1)
    }

    #[test_case(TEST_SHORT => 1)]
    #[test_case(TEST_LONG => 36)]

    fn long_rope(input: &str) -> usize {
        series_of_motions(input, 9)
    }
}
