use std::fmt::Display;

use crate::Solver;
use anyhow::anyhow;
use derive_more::{Add, AddAssign, Sub, SubAssign};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::{self, Finish, IResult};
pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> String {
        solve_part_1(input, false).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        solve_part_2(input, false).to_string()
    }
}

fn solve_part_2(input: &str, print: bool) -> usize {
    let mut g = Grid::parse(input).unwrap();
    g.add_floor();
    while g.drop_sand() != Drop::TheStart {}
    if print {
        println!("{}", g);
    }
    g.dropped
}

fn solve_part_1(input: &str, print: bool) -> usize {
    let mut g = Grid::parse(input).unwrap();
    while g.drop_sand() != Drop::TheAbyss {}
    if print {
        println!("{}", g);
    }
    g.dropped
}

const SAND_START: PosG = PosG { x: 500, y: 0 };
#[derive(
    Clone, Copy, Debug, Add, AddAssign, Sub, SubAssign, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
struct PosG {
    x: isize,
    y: isize,
}

impl PosG {
    fn as_index(&self, dim: PosG) -> usize {
        (self.y * dim.x + self.x) as usize
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

fn parse_path(input: &str) -> IResult<&str, Vec<PosG>> {
    nom::multi::separated_list1(tag(" -> "), PosG::parse)(input)
}

fn complete_isize(input: &str) -> IResult<&str, isize> {
    map(nom::character::complete::i64, |u| u as isize)(input)
}

impl From<(usize, usize)> for PosG {
    fn from((x, y): (usize, usize)) -> Self {
        let (x, y) = (x as isize, y as isize);
        Self { x, y }
    }
}

impl From<(isize, isize)> for PosG {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

impl From<PosG> for (isize, isize) {
    fn from(PosG { x, y }: PosG) -> Self {
        (x, y)
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Cell>,
    dim: PosG,
    paths: Vec<Vec<PosG>>,
    backtrack: Vec<PosG>,
    current: PosG,
    dropped: usize,
    offset: isize,
    sand_start: PosG,
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

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
enum Cell {
    Air = b' ',
    Rock = b'#',
    Sand = b'o',
    Start = b'+',
}

impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        value as u8 as char
    }
}

#[derive(PartialEq, Debug)]
enum Drop {
    TheAbyss,
    TheStart,
    Dropping,
}

impl Grid {
    fn get_cell(&self, v: PosG) -> Option<&Cell> {
        let index = v.as_index(self.dim);
        self.grid.get(index)
    }

    fn get_cell_mut(&mut self, v: PosG) -> Option<&mut Cell> {
        let index = v.as_index(self.dim);
        self.grid.get_mut(index)
    }

    fn get_cell_slice(&self, v: PosG, l: usize, offset: isize) -> Option<&[Cell]> {
        let index = (v.as_index(self.dim) as isize + offset) as usize;
        // &self.grid[index..index + l]
        self.grid.get(index..index + l)
    }

    fn reset_sand(&mut self) {
        self.grid
            .iter_mut()
            .filter(|c| **c == Cell::Sand)
            .for_each(|c| *c = Cell::Air);
        self.backtrack.clear();
        self.current = self.sand_start;
        self.dropped = 0;
    }

    fn drop_sand(&mut self) -> Drop {
        let (a, b, c) =
            if let Some(x) = self.get_cell_slice(self.current + (0isize, 1isize).into(), 3, -1) {
                x.iter().tuples().next().unwrap()
            } else {
                return Drop::TheAbyss;
            };

        use Cell::*;
        match (a, b, c) {
            (_, Air, _) => {
                // free fall
                *self.get_cell_mut(self.current).unwrap() = Cell::Air;
                self.current.y += 1;
                *self.get_cell_mut(self.current).unwrap() = Cell::Sand;
            }
            (Air, Rock | Sand, _) => {
                // drop left
                *self.get_cell_mut(self.current).unwrap() = Cell::Air;
                self.backtrack.push(self.current);
                self.current += (-1isize, 1isize).into();
                *self.get_cell_mut(self.current).unwrap() = Cell::Sand;
            }
            (Rock | Sand, Rock | Sand, Air) => {
                // drop right
                *self.get_cell_mut(self.current).unwrap() = Cell::Air;
                self.backtrack.push(self.current);
                self.current += (1isize, 1isize).into();
                *self.get_cell_mut(self.current).unwrap() = Cell::Sand;
            }
            (Rock | Sand, Rock | Sand, Rock | Sand) => {
                // bottom out
                self.dropped += 1;

                if self.current == self.sand_start {
                    return Drop::TheStart;
                }
                self.current = self.backtrack.pop().unwrap_or(self.sand_start);
                *self.get_cell_mut(self.current).unwrap() = Cell::Sand;
            }
            (_, _, _) => {
                unimplemented!()
            }
        }
        Drop::Dropping
    }
    fn from_paths(mut paths: Vec<Vec<PosG>>) -> anyhow::Result<Self> {
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

        let min = PosG::from((xmin, ymin));
        let max = PosG::from((xmax, ymax));

        let y = max.y + 2; // this way we can already fit the floor
        let x = max.x - min.x + 2 * y; // this would make it bigger than necessary I guess, max to the floor should be y. we'll see

        let offset = min.x - y;

        let dim = PosG { x, y };

        for p in paths.iter_mut() {
            for e in p.iter_mut() {
                e.x -= offset;
            }
        }

        let capacity = (dim.x * dim.y).abs() as usize;
        let grid = vec![Cell::Air; capacity];

        let sand_start = SAND_START - PosG { x: offset, y: 0 };
        let mut grid = Self {
            grid,
            dim,
            paths,
            backtrack: vec![],
            current: sand_start,
            dropped: 0,
            offset,
            sand_start,
        };
        grid.draw_paths()?;
        Ok(grid)
    }

    fn add_floor(&mut self) {
        for _ in 0..self.dim.x {
            self.grid.push(Cell::Rock)
        }
    }

    fn draw_paths(&mut self) -> anyhow::Result<()> {
        let Self {
            grid, dim, paths, ..
        } = self;
        paths
            .iter()
            .map(|p| {
                let path: &[PosG] = &p;
                for (start, end) in path.iter().tuple_windows() {
                    let (p_start, p_end) = [start, end]
                        .into_iter()
                        .sorted_by(|a, b| a.as_index(*dim).cmp(&b.as_index(*dim)))
                        .tuples()
                        .next()
                        .unwrap();

                    let diff = *p_end - *p_start;
                    let x = 5;
                    // let y = x.sig

                    let s_index = p_start.as_index(*dim);

                    let e_index = p_end.as_index(*dim);

                    match diff.into() {
                        (0, 0) => {
                            anyhow::bail!("path length 0");
                        }
                        (x, 0) => {
                            grid.iter_mut()
                                .skip(s_index)
                                .take((x.abs() as usize) + 1)
                                .for_each(|field| *field = Cell::Rock);
                        }
                        (0, _y) => {
                            grid.iter_mut()
                                .enumerate()
                                .skip(s_index)
                                .step_by(dim.x as usize)
                                .take_while(|(i, _f)| *i <= e_index)
                                .for_each(|(_, field)| *field = Cell::Rock);
                        }
                        (_, _) => {
                            return Err(anyhow!("only straight paths supported"));
                        }
                    }
                }
                Ok(())
            })
            .all(|l| l.is_ok());
        *self.get_cell_mut(self.sand_start).unwrap() = Cell::Start;
        Ok(())
    }

    fn parse(input: &str) -> anyhow::Result<Self> {
        let p = input
            .lines()
            .filter_map(|l| parse_path(l).finish().ok().map(|(_, p)| p))
            .collect();

        Grid::from_paths(p)
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    use super::{parse_path, solve_part_1, solve_part_2, Day, Drop, Grid, PosG};
    use assert_ok::assert_ok;

    fn path_input_parser(input: &str) -> Vec<Vec<PosG>> {
        input.lines().map(|l| parse_path(l).unwrap().1).collect()
    }

    #[test]
    fn path_parser() {
        let mut it = path_input_parser(INPUT).into_iter();
        assert_eq!(
            Some(vec![
                PosG::from((498isize, 4)),
                PosG::from((498isize, 6)),
                PosG::from((496isize, 6)),
            ]),
            it.next()
        );
        assert_eq!(
            Some(vec![
                PosG::from((503isize, 4)),
                PosG::from((502isize, 4)),
                PosG::from((502isize, 9)),
                PosG::from((494isize, 9)),
            ]),
            it.next()
        )
    }

    #[test]
    fn part_1() {
        let mut g = Grid::parse(INPUT).unwrap();
        println!("{}", g);
        g.draw_paths().unwrap();
        println!("{}", g);
        let mut i = 0;

        while g.drop_sand() != Drop::TheAbyss {
            i += 1;
        }
        println!("{}", i);
        println!("{}", g);

        println!("{:?}", g.dropped);
        assert_eq!(24, g.dropped)
    }
    #[test]
    fn part_2() {
        let mut g = Grid::parse(INPUT).unwrap();
        println!("{}", g);
        g.add_floor();
        g.draw_paths().unwrap();
        println!("{}", g);
        let mut i = 0;
        while g.drop_sand() != Drop::TheStart {
            i += 1;
        }
        println!("{}", g);

        println!("{:?}", g.dropped);
        assert_eq!(93, g.dropped);
    }

    #[test]
    fn part_1_solver() {
        assert_eq!(solve_part_1(INPUT, true), 24);
        let input = include_str!("../../input/day_14");
        assert_eq!(solve_part_1(input, true), 825);
    }
    #[test]
    fn part_2_solver() {
        assert_eq!(solve_part_2(INPUT, true), 93);
        let input = include_str!("../../input/day_14");
        assert_eq!(solve_part_2(input, true), 26729);
    }
}
