extern crate derive_more;
use derive_more::{Add, AddAssign, Sub, SubAssign};

use crate::Solver;

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> String {
        find(input).unwrap().to_string()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
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
    fn get_coord(&self, p: GridPoint) -> <&Cell> {
        let i = p.as_index(self.dim());
        &self.grid[i]
    }

    fn in_bounds(&self, p: GridPoint) -> bool {
        p.x < self.width && p.y < self.height
    }

    fn get_neighbours(&self, p: GridPoint) -> impl Iterator<Item = GridPoint> + '_ {
        let deltas: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        let height = self.get_coord(p).height();

        deltas.into_iter().filter_map(move |(dx, dy)| {
            Some(GridPoint {
                x: p.x.checked_add_signed(dx)?,
                y: p.y.checked_add_signed(dy)?,
            })
            .filter(|&s| self.in_bounds(s))
            .filter(|&s| {
                let b = self.get_coord(s).height();
                height.abs_diff(b) <= 1
            }).map(|c| {println!("gathered {c:?}"); c})
        })
    }

    fn walkable(&self, p: GridPoint, q: GridPoint) -> bool {
        let a = self.get_coord(p).height();
        let b = self.get_coord(q).height();

        a.abs_diff(b) <= 1
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

fn find(input: &str) -> Result<usize> {
    let g = Grid::parse(input).ok_or(anyhow!("failed to parse grid"))?;

    let start = g.start;
    let end = g.end;

    let result = bfs(
        &start,
        |&s| g.get_neighbours(s).collect::<Vec<GridPoint>>(),
        |&s| s == end,
    )
    .ok_or(anyhow!("No path found!"))?;

    Ok(result.len() - 1)
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
    fn test_find() {
        let result = assert_ok!(find(INPUT));
        assert_eq!(result, 31)
        // assert_eq!(find(input))
    }
}
