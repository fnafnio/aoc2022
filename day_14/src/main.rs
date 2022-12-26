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
    x: isize,
    y: isize,
}

impl Vec2 {
    fn as_index(&self, (width, height): (isize, isize)) -> usize {
        (self.y * width + self.x) as usize
    }

    fn from_index(index: usize, (width, height): (usize, usize)) -> Self {
        let x = index.rem_euclid(width) as isize;
        let y = index.div_floor(width) as isize;
        (x, y).into()
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(complete_isize, tag(","), complete_isize),
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

fn complete_isize(input: &str) -> IResult<&str, isize> {
    map(nom::character::complete::i64, |u| u as isize)(input)
}

impl From<(usize, usize)> for Vec2 {
    fn from((x, y): (usize, usize)) -> Self {
        let (x, y) = (x as isize, y as isize);
        Self { x, y }
    }
}
impl From<(isize, isize)> for Vec2 {
    fn from((x, y): (isize, isize)) -> Self {
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

        let offset = min - (20, 20).into();
        let dim = max - min + (40, 40).into();

        println!("{offset:?}");
        println!("{dim:?}");

        let grid = Vec::with_capacity(dim.x * dim.y);

        let mut s = Self { grid, dim, offset };

        for p in paths.iter() {
            s.set_path(&p, Cell::Rock)?;
        }

        Ok(s)
    }

    fn set_path(&mut self, path: &[Vec2], c: Cell) -> anyhow::Result<()> {
        // let mut it = path.iter();
        // let seed = it.next().ok_or(anyhow!("path empty"))?;
        // it.scan(seed, |prev, x| println!("{x}"));
        for (s, e) in path.iter().tuple_windows() {
            let diff = *e - *s;
            let s = *s - self.offset;
            let e = *e - self.offset;
            if diff.x != 0 && diff.y != 0 {
                return Err(anyhow!("only straight paths supported"));
            }

            if diff.x != 0 {
                // self.grid.as_mut_slice()[s.x+s.y..e.x+e.y*self.dim.x].fill(c);
                self.grid
                    .iter_mut()
                    .skip(s.y * self.dim.x + s.x)
                    .take(diff.x)
                    .for_each(|field| *field = c);
            }
            if diff.y != 0 {
                self.grid
                    .iter_mut()
                    .skip(s.y * self.dim.x + s.x)
                    .step_by(self.dim.x)
                    .for_each(|field| *field = c);
            }
        }
        Ok(())
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
