#![feature(int_roundings)]

const INPUT: &str = include_str!("../../input/day_14");

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    Ok(())
}

use std::fmt::Display;

use anyhow::anyhow;
use derive_more::{Add, AddAssign, Sub, SubAssign};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::{self, IResult};

#[derive(
    Clone, Copy, Debug, Add, AddAssign, Sub, SubAssign, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    // fn as_index(&self, (width, height): (isize, isize)) -> usize {
    //     (self.y * width + self.x) as usize
    // }

    fn as_index(&self, s: Vec2) -> usize {
        (self.y * s.x + self.x) as usize
    }

    fn from_index(index: usize, (width, height): (isize, isize)) -> Self {
        let x = (index as isize).rem_euclid(width) as isize;
        let y = (index as isize).div_floor(width) as isize;
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

// fn complete_usize(input: &str) -> IResult<&str, usize> {
//     map(nom::character::complete::u64, |u| u as usize)(input)
// }

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
#[repr(u8)]
enum Cell {
    Air = b' ',
    Rock = b'#',
    Sand = b'o',
}

impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        value as u8 as char
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Cell>,
    dim: Vec2,
    offset: Vec2,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid
            .chunks(self.dim.x as _)
            .map(|c| c.iter().map(|c| char::from(*c)).collect::<String>())
            .enumerate()
            .for_each(|(i, l)| writeln!(f, "{}:{}", i, l).unwrap());

        Ok(())
    }
}

impl Grid {
    fn from_paths(paths: &Vec<Vec<Vec2>>) -> anyhow::Result<Self> {
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

        let offset = (min.x - 4, 0).into();
        let dim = Vec2 {
            x: max.x - min.x + 12,
            y: max.y + 1,
        };

        let capacity = (dim.x * dim.y).abs() as usize;
        let mut grid = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            grid.push(Cell::Air)
        }

        Ok(Self { grid, dim, offset })
    }

    fn set_path(&mut self, path: &[Vec2], c: Cell) -> anyhow::Result<()> {
        for (s, e) in path.iter().tuple_windows() {
            println!("self:\n{self}");

            let Self { grid, dim, offset } = self;

            let (s, e) = [s, e]
                .into_iter()
                .sorted_by(|a, b| a.as_index(*dim).cmp(&b.as_index(*dim)))
                .tuples()
                .next()
                .unwrap();

            let diff = *e - *s;
            let s = *s - *offset;
            let e = *e - *offset;

            if diff.x != 0 && diff.y != 0 {
                return Err(anyhow!("only straight paths supported"));
            }

            let s_index = s.as_index(*dim);
            let e_index = e.as_index(*dim);

            if diff.x != 0 {
                grid.iter_mut()
                    .skip(s_index)
                    .take((diff.x.abs() as usize) + 1)
                    .for_each(|field| *field = c);
            } else if diff.y != 0 {
                println!("s_index: {s_index}");
                println!("dim.x: {}", dim.x);
                grid.iter_mut()
                    .enumerate()
                    .skip(s_index)
                    .step_by(dim.x as usize)
                    .take_while(|(i, f)| *i <= e_index)
                    .for_each(|(_, field)| *field = c);
            }
        }
        Ok(())
    }
}

fn path_input_parser(input: &str) -> Vec<Vec<Vec2>> {
    input.lines().map(|l| parse_path(l).unwrap().1).collect()
}

fn solve_part_1(input: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    use crate::Grid;
    use crate::{path_input_parser, Vec2};
    use assert_ok::assert_ok;

    #[test]
    fn test_parse_path() {
        let mut it = path_input_parser(INPUT).into_iter();
        assert_eq!(
            Some(vec![
                Vec2::from((498isize, 4)),
                Vec2::from((498isize, 6)),
                Vec2::from((496isize, 6)),
            ]),
            it.next()
        );
        assert_eq!(
            Some(vec![
                Vec2::from((503isize, 4)),
                Vec2::from((502isize, 4)),
                Vec2::from((502isize, 9)),
                Vec2::from((494isize, 9)),
            ]),
            it.next()
        )
    }

    #[test]
    fn test_creation() {
        let paths = path_input_parser(INPUT);
        let mut g = assert_ok!(Grid::from_paths(&paths));
        println!("offset {:?}", g.offset);
        println!("dim{:?}", g.dim);
        // println!("{g}");
        for p in paths {
            println!("{p:?}");
            println!("{g}");
            g.set_path(&p, crate::Cell::Rock);
        }
        println!("{g}");
    }
}
