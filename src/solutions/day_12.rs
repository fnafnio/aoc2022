extern crate derive_more;
use derive_more::{Add, AddAssign, Sub, SubAssign};
use itertools::Itertools;

use crate::Solver;

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> String {
        find_s_to_e(input).unwrap().to_string()
    }

    fn part_2(&self, input: &str) -> String {
        find_e_to_low(input).unwrap().to_string()
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Cell>,
    width: usize,
    height: usize,
    start: GridPoint,
    end: GridPoint,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Start,
    End,
    Any(u8),
}

impl Cell {
    fn height(&self) -> u8 {
        match self {
            Cell::Start => 0,
            Cell::End => 25,
            Cell::Any(x) => *x,
        }
    }
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'S' => Self::Start,
            b'E' => Self::End,
            x => Self::Any(x - b'a'),
        }
    }
}

#[derive(Clone, Copy, Debug, Add, AddAssign, Sub, SubAssign, PartialEq, Eq, Hash)]
struct GridPoint {
    x: usize,
    y: usize,
}

impl GridPoint {
    fn as_index(&self, (width, height): (usize, usize)) -> usize {
        (self.y * width + self.x) as usize
    }

    fn from_index(index: usize, (width, height): (usize, usize)) -> Self {
        let x = index.rem_euclid(width) as usize;
        let y = index.div_floor(width) as usize;
        (x, y).into()
    }
}

impl From<(usize, usize)> for GridPoint {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl Grid {
    fn get_coord(&self, p: GridPoint) -> Option<&Cell> {
        let i = p.as_index(self.dim());
        self.grid.get(i)
    }

    fn get_coord_mut(&mut self, p: GridPoint) -> Option<&mut Cell> {
        let i = p.as_index(self.dim());
        self.grid.get_mut(i)
    }

    fn in_bounds(&self, p: GridPoint) -> bool {
        p.x < self.width && p.y < self.height
    }

    fn is_border(&self, p: GridPoint) -> bool {
        p.x == 0 || p.x == self.width - 1 || p.y == 0 || p.y == self.height - 1
    }

    fn print(&self) {
        self.grid
            .iter()
            .map(|c| char::from(c.height() + b'a'))
            .chunks(self.width)
            .into_iter()
            .for_each(|l| {
                let s: String = l.collect();
                println!("{s}")
            })
    }

    fn get_neighbours(&self, p: GridPoint) -> impl Iterator<Item = GridPoint> + '_ {
        let deltas: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        let cur_height = self.get_coord(p).expect("current out of bounds!").height();

        deltas.into_iter().filter_map(move |(dx, dy)| {
            Some(GridPoint {
                x: p.x.checked_add_signed(dx)?,
                y: p.y.checked_add_signed(dy)?,
            })
            .filter(|&s| self.in_bounds(s))
            .filter(|&s| {
                let other_height = self.get_coord(s).expect("neighbour out of bounds").height();
                other_height + 2 > cur_height
            })
        })
    }

    fn dim(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn parse(input: &str) -> Option<Self> {
        let width = input.lines().next().unwrap().len() as _;
        let height = input.lines().count() as _;
        let grid: Vec<Cell> = input
            .lines()
            .flat_map(|l| l.bytes())
            .map(|c| Cell::from(c))
            .collect();

        let start = grid.iter().position(|&c| c == Cell::Start).unwrap();
        let end = grid.iter().position(|&c| c == Cell::End).unwrap();

        Some(Self {
            grid,
            width,
            height,
            start: GridPoint::from_index(start, (width, height)),
            end: GridPoint::from_index(end, (width, height)),
        })
    }
}

use pathfinding::{directed::bfs::bfs, grid};

use anyhow::{anyhow, Result};

fn find_s_to_e(input: &str) -> Result<usize> {
    let g = Grid::parse(input).ok_or(anyhow!("failed to parse grid"))?;

    let start = g.start;
    let end = g.end;

    let result = bfs(
        &start,
        |&p| g.get_neighbours(p).collect::<Vec<GridPoint>>(),
        |&p| p == end,
    )
    .ok_or(anyhow!("No path found!"))?;

    Ok(result.len() - 1)
}

fn find_e_to_low(input: &str) -> Result<usize> {
    let g = Grid::parse(input).ok_or(anyhow!("failed to parse grid"))?;

    // we search from highest point to lowest point at the border
    let start = g.end;

    let result = bfs(
        &start,
        |&p| g.get_neighbours(p).collect::<Vec<GridPoint>>(),
        |&p| {
            g.is_border(p)
                && match g.get_coord(p) {
                    Some(o) => o.height() == 0,
                    None => false,
                }
        },
    )
    .ok_or(anyhow!("No path found!"))?;

    Ok(result.len() - 1)
    // Err(anyhow!("How did we end up here?"))
}

#[cfg(test)]
mod tests {

    use super::*;
    use assert_ok::assert_ok;

    const INPUT: &str = "Sabqponm\n\
                         abcryxxl\n\
                         accszExk\n\
                         acctuvwj\n\
                         abdefghi\n";

    #[test]
    fn test_parse() {
        let g = Grid::parse(INPUT).unwrap();
    }

    #[test]
    fn test_get_neighbours() {
        let g = Grid::parse(INPUT).unwrap();
        let v: Vec<GridPoint> = g.get_neighbours(g.start).collect();
        let correct = vec![GridPoint::from((0usize, 1usize)), (1, 0).into()];
        dbg!(correct);
        dbg!(v);
    }

    #[test]
    fn test_find_s_to_e() {
        let result = assert_ok!(find_s_to_e(INPUT));
        assert_eq!(result, 31)
    }

    #[test]
    fn test_find_e_to_low() {
        let result = assert_ok!(find_e_to_low(INPUT));
        assert_eq!(result, 29)
    }
}
