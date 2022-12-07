use itertools::Itertools;
use std::collections::BinaryHeap;

use crate::Solver;

pub struct Day1;

impl Solver for Day1 {
    fn part_1(&self, input: &str) -> String {
        solve_1(input).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        solve_2(input).to_string()
    }
}

fn build_heap(input: &str) -> BinaryHeap<usize> {
    input
        .lines()
        .group_by(|&l| !l.is_empty())
        .into_iter()
        .filter(|(a, _b)| *a)
        .filter_map(|(_a, b)| b.filter_map(|l| l.parse::<usize>().ok()).sum1())
        .collect()
}

fn solve_1(input: &str) -> usize {
    let mut heap = build_heap(input);
    heap.pop().expect("Heap should not be empty")
}

fn solve_2(input: &str) -> usize {
    let mut heap = build_heap(input);
    heap.drain_sorted().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::{solve_1, solve_2};

    const TEST: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(TEST), 24000);
    }
    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(TEST), 45000);
    }
}
