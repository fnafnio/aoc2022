#![feature(int_roundings)]

const INPUT: &str = include_str!("../../input/day_14");

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    Ok(())
}

use anyhow::anyhow;
use derive_more::{Add, AddAssign, Sub, SubAssign};
use itertools::Itertools;
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
    fn move_me(&mut self, trans: &Vec2) {
        
    }

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

fn parse_path(input: &str) -> IResult<&str, Vec<Vec2>> {
    nom::multi::separated_list1(tag(" -> "), Vec2::parse)(input)
}

fn complete_usize(input: &str) -> IResult<&str, usize> {
    map(nom::character::complete::u64, |u| u as usize)(input)
}

impl From<(usize, usize)> for Vec2 {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Air,
    Rock,
    Sand,
}

impl Into<char> for Cell {
    fn into(self) -> char {
        match self {
            Cell::Air => ' ',
            Cell::Rock => '#',
            Cell::Sand => 'o',
        }
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Cell>,
    dim: Vec2,
    offset: Vec2,
}

impl Grid {
    fn from_paths(paths: Vec<Vec<Vec2>>) -> anyhow::Result<Self> {
        let (xmin, xmax) = paths
            .iter()
            .flat_map(|p| p.iter())
            .map(|v| v.x)
            .minmax()
            .into_option()
            .ok_or(anyhow!("failed to find minmax"))?;
            
        let (ymin, ymax) = paths
            .iter()
            .flat_map(|p| p.iter())
            .map(|v| v.y)
            .minmax()
            .into_option()
            .ok_or(anyhow!("failed to find minmax"))?;
            
        let min = Vec2::from((xmin, ymin));
        let max = Vec2::from((xmax, ymax));

        let offset = min -


        Err(anyhow!("damn"))
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
    use crate::parse_path;
    use crate::Vec2;

    #[test]
    fn test_parse_path() {
        let mut it = INPUT.lines().map(|l| parse_path(l).unwrap().1);
        assert_eq!(
            Some(vec![
                Vec2::from((498, 4)),
                Vec2::from((498, 6)),
                Vec2::from((496, 6)),
            ]),
            it.next()
        );
        assert_eq!(
            Some(vec![
                Vec2::from((503, 4)),
                Vec2::from((502, 4)),
                Vec2::from((502, 9)),
                Vec2::from((494, 9)),
            ]),
            it.next()
        )
    }
}
