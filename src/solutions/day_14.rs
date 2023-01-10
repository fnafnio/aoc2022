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
        let mut g = Grid::parse(input).unwrap();
        while g.drop_sand() {}
        g.dropped.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let mut g = Grid::parse(input).unwrap();
        g.add_floor();
        while g.drop_sand() {}
        g.dropped.to_string()
    }
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
    // fn as_index(&self, (width, height): (isize, isize)) -> usize {
    //     (self.y * width + self.x) as usize
    // }

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
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid
            .chunks(self.dim.x as _)
            .map(|c| c.iter().skip((500 - self.dim.x) as usize).take(self.dim.x as usize * 2).map(|c| char::from(*c)).collect::<String>())
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
}

impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        value as u8 as char
    }
}

impl Grid {
    fn get_cell(&self, v: PosG) -> Option<&Cell> {
        let index = v.as_index(self.dim);
        // &self.grid[index]
        self.grid.get(index)
    }

    fn get_cell_mut(&mut self, v: PosG) -> Option<&mut Cell> {
        let index = v.as_index(self.dim);
        self.grid.get_mut(index)
        // &mut self.grid[index]
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
        self.current = SAND_START;
        self.dropped = 0;
    }

    fn drop_sand(&mut self) -> bool {
        let (a, b, c) =
            if let Some(x) = self.get_cell_slice(self.current + (0isize, 1isize).into(), 3, -1) {
                x.iter().tuples().next().unwrap()
            } else {
                return false;
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
                if self.current == SAND_START {
                    return false;
                }
                self.current = self.backtrack.pop().unwrap_or(SAND_START);
                *self.get_cell_mut(self.current).unwrap() = Cell::Sand;
                self.dropped += 1;
            }
        }
        true
    }
    fn from_paths(paths: Vec<Vec<PosG>>) -> anyhow::Result<Self> {
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

        let y = max.y + 2;
        let dim = PosG {
            // x: max.x - min.x + 2 * y,
            x: 1000,
            y: y,
        };

        let capacity = (dim.x * dim.y).abs() as usize;
        let mut grid = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            grid.push(Cell::Air)
        }

        let mut grid = Self {
            grid,
            dim,
            paths,
            backtrack: vec![],
            current: SAND_START,
            dropped: 0,
        };
        grid.draw_paths()?;
        Ok(grid)
    }

    fn add_floor(&mut self) {
        for _ in 0..self.dim.x {
            self.grid.push(Cell::Air)
        }
        for _ in 0..self.dim.x {
            self.grid.push(Cell::Rock)
        }
        self.dim.y += 2;
    }

    fn draw_paths(&mut self) -> anyhow::Result<()> {
        let Self {
            grid, dim, paths, ..
        } = self;
        paths
            .iter()
            .map(|p| {
                let path: &[PosG] = &p;
                for (s, e) in path.iter().tuple_windows() {
                    let (s, e) = [s, e]
                        .into_iter()
                        .sorted_by(|a, b| a.as_index(*dim).cmp(&b.as_index(*dim)))
                        .tuples()
                        .next()
                        .unwrap();

                    let diff = *e - *s;

                    let s_index = s.as_index(*dim);
                    let e_index = e.as_index(*dim);

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
    use super::{parse_path, Grid, PosG};
    use assert_ok::assert_ok;

    fn path_input_parser(input: &str) -> Vec<Vec<PosG>> {
        input.lines().map(|l| parse_path(l).unwrap().1).collect()
    }

    #[test]
    fn test_parse_path() {
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
    fn test_part_1() {
        let mut g = Grid::parse(INPUT).unwrap();
        println!("{}", g);
        g.draw_paths().unwrap();
        println!("{}", g);
        let mut i = 0;
        while g.drop_sand() {
            i += 1;
        }
        println!("{}", g);

        println!("{:?}", g.dropped);
        assert_eq!(25, g.dropped)
    }
    #[test]
    fn test_part_2() {
        let mut g = Grid::parse(INPUT).unwrap();
        println!("{}", g);
        g.add_floor();
        g.draw_paths().unwrap();
        println!("{}", g);
        let mut i = 0;
        while g.drop_sand() {
            i += 1;
        }
        println!("{}", g);

        println!("{:?}", g.dropped);
        assert_eq!(93, g.dropped);
    }
    // #[test]
    // fn test_part_1_2() {
    //     let s = include_str!("../../../input/day_14");
    //     let mut g = Grid::parse(s).unwrap();
    //     println!("{}", g);
    //     g.draw_paths().unwrap();
    //     println!("{}", g);
    //     let mut i = 0;
    //     while g.drop_sand() {
    //         i += 1;
    //     }
    //     println!("{}", g);

    //     println!("{:?}", g.dropped);
    //     assert_eq!(25, g.dropped)
    // }
    // #[test]
    // fn test_part_2_2() {
    //     let s = include_str!("../../../input/day_14");
    //     let mut g = Grid::parse(s).unwrap();
    //     println!("{}", g);
    //     g.add_floor();
    //     g.draw_paths().unwrap();
    //     println!("{}", g);
    //     let mut i = 0;
    //     while g.drop_sand() {
    //         i += 1;
    //     }
    //     println!("{}", g);

    //     println!("{:?}", g.dropped);
    //     assert_eq!(93, g.dropped);
    // }
}
