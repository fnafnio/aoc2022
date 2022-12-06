use itertools::Itertools;

use crate::Solver;

pub struct Day4;

impl Solver for Day4 {
    fn part_1(&self, input: &str) -> String {
        solve_1(input).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        solve_2(input).to_string()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Assignment(i64, i64);

impl Assignment {
    fn fully_contained(&self, rhs: &Assignment) -> bool {
        self.contains(rhs.0) && self.contains(rhs.1) || rhs.contains(self.0) && rhs.contains(self.1)
    }

    fn contains(&self, rhs: i64) -> bool {
        self.0 <= rhs && self.1 >= rhs
    }

    fn overlap(&self, rhs: &Assignment) -> bool {
        self.contains(rhs.0) || self.contains(rhs.1)
    }
}

impl TryFrom<&str> for Assignment {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (low, high) = s.split_once('-').ok_or("error splitting string")?;
        let low = low.parse().map_err(|_| "error parsing to string")?;
        let high = high.parse().map_err(|_| "error parsing to string")?;
        Ok(Self(low, high))
    }
}

const INPUT: &str = include_str!("../input/day_4/input");
pub fn _day_4() {
    let result = solve_1(INPUT);
    println!("Day 4.1: {:12}", result);
    let result = solve_2(INPUT);
    println!("Day 4.2: {:12}", result);
}


fn parse_line(l: &str) -> (Assignment, Assignment) {
    l.split(',')
        .map(|a| Assignment::try_from(a).unwrap())
        .sorted()
        .collect_tuple()
        .unwrap()
}

fn solve_1(input: &str) -> usize {
    input
        .lines()
        .map(|l| parse_line(l))
        .filter(|(x, y)| x.fully_contained(y))
        .count()
}
fn solve_2(input: &str) -> usize {
    input
        .lines()
        .map(|l| parse_line(l))
        .filter(|(x, y)| x.overlap(y))
        .count()
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n";

    #[test]
    fn test_parsing() {
        TEST.lines().for_each(|s| {
            s.split(',')
                .for_each(|a| assert!(Assignment::try_from(a).is_ok()))
        })
    }
    #[test]
    fn test_ordering() {
        let mut ass: Vec<Assignment> = TEST
            .lines()
            .map(|l| l.split(','))
            .flatten()
            .map(|ass| ass.try_into().unwrap())
            .collect();

        ass.sort();
        let ordered = ass
            .iter()
            .skip(1)
            .zip(ass.iter())
            .map(|(l, h)| l.0 >= h.0)
            .any(|f| f);
        assert!(ordered)
    }

    #[test]
    fn test_part_1() {
        assert_eq!(solve_1(&TEST), 2)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_2(&TEST), 4)
    }
}
