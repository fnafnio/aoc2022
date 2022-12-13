use std::fmt::Display;

use crate::Solver;
use itertools::*;

pub struct Day;

// type Forrest = Ve

impl Solver for Day {
    fn part_1(&self, input: &str) -> String {
        let forest = parse_forest(input);
        let mut visible = prepare_visible(&forest);
        count_visible(&visible);

        {
            let f_it = forest.iter();
            let v_it = visible.iter_mut();
            search(f_it, v_it);
        }

        count_visible(&visible);

        {
            let f_it = forest.iter();
            let v_it = visible.iter_mut();

            search_rev(f_it, v_it);
        }
        count_visible(&visible);

        print_forest(&forest);

        let forest = transpose2(forest);
        let mut visible = transpose2(visible);

        {
            let f_it = forest.iter();
            let v_it = visible.iter_mut();
            search(f_it, v_it);
        }

        count_visible(&visible);

        {
            let f_it = forest.iter();
            let v_it = visible.iter_mut();

            search_rev(f_it, v_it);
        }
        print_forest(&forest);

        count_visible(&visible).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}

fn print_forest<T: Display>(f: &Vec<Vec<T>>) {
    for l in f {
        for t in l {
            print!("{}", t);
        }
        println!();
    }
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn search_rev(f_it: std::slice::Iter<Vec<u8>>, v_it: std::slice::IterMut<Vec<bool>>) {
    for (l_f, l_v) in f_it.zip(v_it) {
        let mut largest = 0;
        for (&t, v) in l_f.iter().rev().zip(l_v.iter_mut().rev()) {
            if t > largest {
                largest = t;
                *v = true;
            }
        }
    }
}

fn search(f_it: std::slice::Iter<Vec<u8>>, v_it: std::slice::IterMut<Vec<bool>>) {
    for (l_f, l_v) in f_it.zip(v_it) {
        let mut largest = 0;
        for (&t, v) in l_f.iter().zip(l_v.iter_mut()) {
            if t > largest {
                largest = t;
                *v = true;
            }
        }
    }
}

fn count_visible(v: &Vec<Vec<bool>>) -> usize {
    let r = v.iter().flat_map(|l| l.iter()).filter(|b| **b).count();
    println!("{:6} visible", r);
    r
}

fn prepare_visible(forest: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let l_len = forest.iter().next().unwrap().len();
    let mut visible: Vec<Vec<bool>> = Vec::with_capacity(forest.len());
    for _ in 0..forest.len() {
        let mut v = Vec::with_capacity(l_len);
        for _ in 0..l_len {
            v.push(false);
        }
        visible.push(v);
    }
    visible
}

fn parse_forest(input: &str) -> Vec<Vec<u8>> {
    let mut forest: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|b| *b).collect())
        .collect();
    forest
}

#[cfg(test)]
mod tests {
    use super::{Day, Solver};
    use assert_ok::assert_ok;
    const TEST: &str = r"30373
25512
65332
33549
35390";

    #[test]
    fn solver_part_1() {
        let result: String = Day.part_1(TEST);
        assert_eq!(&result, "21")
    }
}
