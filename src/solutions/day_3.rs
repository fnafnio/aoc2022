use std::collections::HashSet;

use itertools::Itertools;

use crate::Solver;

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> String {
        solve_1(input).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        solve_2(input).to_string()
    }
}



fn solve_1(input: &str) -> usize {
    input.lines().map(|l| compartment_check(l)).sum()
}

fn solve_2(input: &str) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|s| s.collect::<Vec<&str>>())
        .map(|s| duplicates(&s).unwrap())
        .map(|c| calc_priority(c))
        .sum()
}

fn duplicates(g: &[&str]) -> Option<char> {
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

fn compartment_check(l: &str) -> usize {
    let (left, right) = l.split_at(l.len().div_ceil(2));

    let dup = duplicates(&[left, right]).unwrap();
    calc_priority(dup)
}

fn calc_priority(c: char) -> usize {
    if c.is_lowercase() {
        (c as u8 - b'a' + 1) as usize
    } else {
        (c as u8 - b'A' + 27) as usize
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use test_case::test_case;

    use super::{calc_priority, compartment_check, duplicates};

    // const TEST: &str = include_str!("../input/day_3/test");
    const TEST: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n";
#[test_case(16, "vJrwpWtwJgWrhcsFMMfFFhFp")]
#[test_case(38, "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")]
#[test_case(42, "PmmdzqPrVvPwwTWBwg")]
#[test_case(22, "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")]
#[test_case(20, "ttgJtRGJQctTZtZT")]
#[test_case(19, "CrZsJsPPZsGzwwsLwLmpwMDw")]
    fn test_compartment_check(prio: usize, line: &str) {
        assert_eq!(prio, compartment_check(line))
    }

    #[test]
    fn part1() {
        let sum: usize = TEST.lines().map(|l| compartment_check(l)).sum();
        assert_eq!(sum, 157);
    }

    #[test]
    fn part2() {
        let sum: usize = TEST
            .lines()
            .chunks(3)
            .into_iter()
            .map(|s| s.collect::<Vec<&str>>())
            .map(|s| duplicates(&s).unwrap())
            .map(|c| calc_priority(c))
            .sum();
        assert_eq!(sum, 70)
    }
}
