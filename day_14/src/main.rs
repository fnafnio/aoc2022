#![feature(int_roundings)]

const INPUT: &str = include_str!("../../input/day_14");

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    Ok(())
}

use anyhow::anyhow;
use derive_more::{Add, AddAssign, Sub, SubAssign};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::{self, IResult};
#[derive(Clone, Copy, Debug, Add, AddAssign, Sub, SubAssign, PartialEq, Eq, Hash)]
struct Vec2 {
    x: usize,
    y: usize,
}

impl Vec2 {
    fn as_index(&self, (width, height): (usize, usize)) -> usize {
        (self.y * width + self.x) as usize
    }

    fn from_index(index: usize, (width, height): (usize, usize)) -> Self {
        let x = index.rem_euclid(width) as usize;
        let y = index.div_floor(width) as usize;
        (x, y).into()
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(complete_usize, tag(","), complete_usize),
            |(x, y)| Self { x, y },
        )(input)
    }
}

fn complete_usize(input: &str) -> IResult<&str, usize> {
    map(nom::character::complete::u64, |u| u as usize)(input)
}

impl From<(usize, usize)> for Vec2 {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    const TEST: &str = "4     5  5
9     0  0
4     0  3
0 ......+...
1 ..........
2 ..........
3 ..........
4 ....#...##
5 ....#...#.
6 ..###...#.
7 ........#.
8 ........#.
9 #########.";

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
}
