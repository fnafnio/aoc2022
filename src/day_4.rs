use itertools::Itertools;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Assignment(i64, i64);

impl Assignment {
    fn fully_contained(&self, rhs: &Assignment) -> bool {
        self.0 >= rhs.0 && self.1 <= rhs.1 || self.0 <= rhs.0 && self.1 >= rhs.1
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

fn parse_line(l: &str) -> (Assignment, Assignment) {
    l.split(',')
        .map(|a| Assignment::try_from(a).unwrap())
        .sorted()
        .collect_tuple()
        .unwrap()
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|l| parse_line(l))
        .filter(|(x, y)| x.fully_contained(y))
        .count()
}
fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|l| parse_line(l))
        .filter(|(x, y)| x.overlap(y))
        .count()
}

pub fn day_4() {
    let result = part_1(INPUT);
    println!("Day 4.1: {:8}", result);
    let result = part_2(INPUT);
    println!("Day 4.2: {:8}", result);
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
        assert_eq!(part_1(&TEST), 2)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&TEST), 4)
    }
}
