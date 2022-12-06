use crate::Solver;


pub struct Day2;

impl Solver for Day2 {
    fn part_1(&self, input: &str) -> String {
        todo!()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}

mod part1 {

    #[derive(Clone, Copy, Debug)]
    enum Rps {
        Rock,
        Paper,
        Scissors,
    }

    impl Into<i64> for Rps {
        fn into(self) -> i64 {
            match self {
                Rps::Rock => 1,
                Rps::Paper => 2,
                Rps::Scissors => 3,
            }
        }
    }

    impl Rps {
        fn battle(&self, other: Self) -> Decision {
            match self {
                Rps::Rock => match other {
                    Rps::Rock => Decision::Draw,
                    Rps::Paper => Decision::Loose,
                    Rps::Scissors => Decision::Win,
                },
                Rps::Paper => match other {
                    Rps::Rock => Decision::Win,
                    Rps::Paper => Decision::Draw,
                    Rps::Scissors => Decision::Loose,
                },
                Rps::Scissors => match other {
                    Rps::Rock => Decision::Loose,
                    Rps::Paper => Decision::Win,
                    Rps::Scissors => Decision::Draw,
                },
            }
        }
    }

    impl TryFrom<char> for Rps {
        type Error = &'static str;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value.to_ascii_lowercase() {
                'a' | 'x' => Ok(Rps::Rock),
                'b' | 'y' => Ok(Rps::Paper),
                'c' | 'z' => Ok(Rps::Scissors),
                _ => Err("not a valid Rps"),
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    enum Decision {
        Loose,
        Draw,
        Win,
    }

    impl Into<i64> for Decision {
        fn into(self) -> i64 {
            match self {
                Decision::Loose => 0,
                Decision::Draw => 3,
                Decision::Win => 6,
            }
        }
    }

    fn parse_line(line: &str) -> Option<(Rps, Rps)> {
        let mut it = line.chars();
        let them = it.next().unwrap().try_into().unwrap();
        let us = it.skip(1).next().unwrap().try_into().unwrap();
        Some((us, them))
    }

    pub(crate) fn answer(input: &str) -> i64 {
        let mut sum: i64 = 0;
        let input = input.lines();

        for (us, them) in input.filter_map(|l| parse_line(l)) {
            let val1: i64 = us.into();
            let val2: i64 = us.battle(them).into();
            sum += val1 + val2;
        }
        return sum;
    }
}

mod part2 {

    pub(crate) fn solve(input: &str) -> i64 {
        let lines = input.lines();
        let mut sum = 0;
        for l in lines {
            let b = l.as_bytes();
            let them = b[0];
            let res = (b[2] - b'X') * 3;

            let them = (them - b'A') as i64;

            let us = match res {
                0 => (them - 1).rem_euclid(3),
                3 => them,
                6 => (them + 1).rem_euclid(3),
                _ => panic!(),
            };

            sum += us as i64 + 1 + res as i64;
        }
        sum
    }
}

pub fn day_2() {
    let input = include_str!("../input/day_2/input");

    println!("Day 2.1: {:12}", part1::answer(input));
    println!("Day 2.2: {:12}", part2::solve(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "A Y\nB X\nC Z";
    #[test]
    fn test_part1() {
        assert_eq!(part1::answer(TEST), 15);
    }
}
