#![feature(int_roundings)]

use std::fmt::Display;
use std::time::Instant;

use anyhow::anyhow;
use derive_more::{Add, AddAssign, Sub, SubAssign};
use egui::{Color32, Pos2, Rect, Sense, Stroke, Vec2};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::{self, Finish, IResult};

const SAND_START: PosG = PosG { x: 500, y: 1 };
const INPUT: &str = include_str!("../../input/day_14");
const _INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let g = Grid::parse(INPUT).unwrap();
    println!("{g}");
    println!("{:?}", g.dim);

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1600.0, 1200.0)),
        ..Default::default()
    };

    eframe::run_native("AoC 2022 — Day 9", options, Box::new(|_cc| Box::new(g)));

    Ok(())
}

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

// fn complete_usize(input: &str) -> IResult<&str, usize> {
//     map(nom::character::complete::u64, |u| u as usize)(input)
// }

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

impl From<PosG> for Pos2 {
    fn from(PosG { x, y }: PosG) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
        }
    }
}

impl From<PosG> for Vec2 {
    fn from(PosG { x, y }: PosG) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
        }
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

#[derive(Debug)]
struct Grid {
    grid: Vec<Cell>,
    dim: PosG,
    paths: Vec<Vec<PosG>>,
    backtrack: Vec<PosG>,
    current: PosG,
    last: f64,
    speed: f64,
    running: bool,
    dropped: usize,
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

impl eframe::App for Grid {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.add(
                egui::Slider::new(&mut self.speed, 0.01..=100.0)
                    .text("Speed")
                    .logarithmic(true),
            );
            ui.add(egui::Checkbox::new(&mut self.running, "Simulation Running"));
            ui.add(egui::Label::new(format!("Dropped: {}", self.dropped)));
            if ui.add(egui::Button::new("Reset")).clicked() {
                self.reset_sand();
            };
        });

        if self.running {
            self.last += self.speed / 60.0;

            while self.last > 1.0 {
                self.last -= 1.0;
                if !self.drop_sand() {
                    self.running = false;
                    break;
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter_size = ui.min_size();
            let scale: Vec2 = painter_size / Vec2::from(self.dim);
            // let scale = egui::vec2(2.0, 4.0);
            let side = scale * Vec2 { x: 1.0, y: 1.0 };

            // ui.label(format!("painter_size: {painter_size:?}",));
            // ui.label(format!("scale: {scale:?}",));
            // ui.label(format!("side: {side:?}",));
            // ui.label(format!("self.dim: {:?}", self.dim));
            ui.label(format!("speed: {}", self.speed));

            // let painter_size = egui::vec2(1500.0, self.dim.y as f32 * SIDE);
            let (res, painter) = ui.allocate_painter(painter_size, Sense::hover());

            let center =
                res.rect.center().to_vec2() * Vec2 { x: 0.5, y: 0.0 } + Vec2 { x: 0.0, y: 6.0 };
            let to_panel_pos = |pos: Pos2| {
                (egui::vec2(pos.x as f32 * side.x, pos.y as f32 * side.y) + center).to_pos2()
            };

            for x in 0..self.dim.x {
                for y in 0..self.dim.y {
                    let dot = PosG { x, y };

                    let cell = *self.get_cell(dot).unwrap();

                    let color = match cell {
                        Cell::Air => {
                            continue;
                        }
                        Cell::Rock => Color32::LIGHT_GRAY,
                        Cell::Sand => Color32::DARK_RED,
                    };

                    let dotf: Pos2 = dot.into();

                    let rect = Rect::from_center_size(to_panel_pos(dotf), side);
                    painter.rect_filled(rect, egui::Rounding::none(), color);
                }
            }
        });
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
        self.last = 0.0;
    }

    fn drop_sand(&mut self) -> bool {
        self.dropped += 1;
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
                self.current = self.backtrack.pop().unwrap_or(SAND_START);
                *self.get_cell_mut(self.current).unwrap() = Cell::Sand;
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
            x: max.x - min.x + 2 * y,
            y,
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
            last: 0.0,
            speed: 1.0,
            running: false,
            dropped: 0,
        };
        grid.draw_paths()?;
        Ok(grid)
    }

    fn draw_paths(&mut self) -> anyhow::Result<()> {
        let Self {
            grid, dim, paths, ..
        } = self;
        paths
            .iter()
            .map(|p| {
                let path: &[PosG] = &p;
                let c = crate::Cell::Rock;
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
                                .for_each(|field| *field = c);
                        }
                        (0, _y) => {
                            grid.iter_mut()
                                .enumerate()
                                .skip(s_index)
                                .step_by(dim.x as usize)
                                .take_while(|(i, _f)| *i <= e_index)
                                .for_each(|(_, field)| *field = c);
                        }
                        (_, _) => {
                            return Err(anyhow!("only straight paths supported"));
                        }
                    }
                    if diff.x != 0 {
                    } else if diff.y != 0 {
                        // println!("s_index: {s_index}");
                        // println!("dim.x: {}", dim.x);
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

fn path_input_parser(input: &str) -> Vec<Vec<PosG>> {
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
    use crate::{path_input_parser, PosG};
    use assert_ok::assert_ok;

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
    fn test_creation() {
        let paths = path_input_parser(INPUT);
        let mut g = assert_ok!(Grid::from_paths(paths));
        println!("dim{:?}", g.dim);
        // println!("{g}");
        for p in paths {
            println!("{p:?}");
            println!("{g}");
            assert_ok!(g.set_path(&p, crate::Cell::Rock));
        }
        println!("{g}");
    }
}
