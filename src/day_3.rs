use std::collections::HashSet;

use itertools::Itertools;

use crate::Solver;

pub struct Day3;

impl Solver for Day3 {
    fn part_1(&self, input: &str) -> String {
        todo!()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}


const INPUT: &str = include_str!("../input/day_3/input");

fn part_1() -> i64 {
    INPUT.lines().map(|l| compartment_check(l)).sum()
}

fn part_2() -> i64 {
    INPUT
        .lines()
        .chunks(3)
        .into_iter()
        .map(|s| s.collect::<Vec<&str>>())
        .map(|s| duplicate(&s).unwrap())
        .map(|c| calc_priority(c))
        .sum()
}

fn duplicate(g: &[&str]) -> Option<char> {
    if g.len() < 2 {
        return None;
    }

    let mut common: HashSet<_> = g[0].chars().collect();
    for &elf in &g[1..] {
        common = elf.chars().filter(|c| common.contains(c)).collect();
        if common.len() == 1 {
            break;
        }
    }
    common.iter().next().cloned()
}

pub fn day_3() {
    let result_1 = part_1();
    println!("Day 3.1: {:12}", result_1);
    let result_2 = part_2();
    println!("Day 3.2: {:12}", result_2);
}

fn compartment_check(l: &str) -> i64 {
    let (left, right) = l.split_at(l.len().div_ceil(2));

    let dup = duplicate(&[left, right]).unwrap();
    calc_priority(dup)
}

fn calc_priority(c: char) -> i64 {
    if c.is_lowercase() {
        (c as u8 - b'a' + 1) as i64
    } else {
        (c as u8 - b'A' + 27) as i64
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::{calc_priority, compartment_check, duplicate};

    const TEST: &str = include_str!("../input/day_3/test");

    #[test]
    fn part1() {
        let sum: i64 = TEST.lines().map(|l| compartment_check(l)).sum();
        assert_eq!(sum, 157);
    }

    #[test]
    fn part2() {
        let sum: i64 = TEST
            .lines()
            .chunks(3)
            .into_iter()
            .map(|s| s.collect::<Vec<&str>>())
            .map(|s| duplicate(&s).unwrap())
            .map(|c| calc_priority(c))
            .sum();
        assert_eq!(sum, 70)
    }
}
