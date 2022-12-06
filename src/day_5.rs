// use std::collections::Vec;

use itertools::Itertools;
const INPUT: &str = include_str!("../input/day_5/input");

fn split_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (stack, moves) = input
        .split_once("\n\n")
        .expect("there should be an empty line!");
    let (stack, moves) = (stack.lines().collect(), moves.lines().collect());

    (stack, moves)
}

fn prepare_input(input: &str) -> (Stacks, Vec<&str>) {
    let (s, m) = split_input(input);
    let s = parse_stack(&s);
    (s, m)
}
#[derive(Debug, Default)]
struct Move {
    src: usize,
    dst: usize,
    cnt: usize,
}

impl TryFrom<&str> for Move {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let it = value
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<usize>().expect("failed to parse to int"));

        let (cnt, src, dst) = it.tuples().next().expect("something went wrong");

        Ok(Self {
            src: src - 1,
            dst: dst - 1,
            cnt,
        })
    }
}

#[derive(Debug, Default, Clone)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn new(stacks: Vec<Vec<char>>) -> Self {
        Self { stacks }
    }

    fn move_single(&mut self, order: &Move) {
        assert!(order.src < self.stacks.len() && order.dst < self.stacks.len());

        for _ in 0..order.cnt {
            let c = self.stacks[order.src]
                .pop()
                .expect("stack already empty");
            self.stacks[order.dst].push(c);
        }
    }

    fn move_multiple(&mut self, order: &Move) {
        let src = &mut self.stacks[order.src];
        let drained: Vec<_> = src.drain(src.len() - order.cnt..).collect();
        for c in drained {
            self.stacks[order.dst].push(c)
        }
    }

    fn _draw(&self) {
        self.stacks.iter().for_each(|s| {
            s.iter().for_each(|c| print!("{}", c));
            println!("");
        });
    }

    fn get_top(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s.last().expect("stack should not be empty"))
            .collect()
    }
}

fn parse_stack(input: &[&str]) -> Stacks {
    let mut it = input.iter().rev();
    let &last = it.next().expect("Empty input");

    let mut stacks: Vec<Vec<char>> = vec![];
    last.split_ascii_whitespace()
        .for_each(|_| stacks.push(Default::default()));

    for &l in it {
        l.chars()
            .chunks(4)
            .into_iter()
            .map(|c| c.skip(1).next().expect("expected another char"))
            .enumerate()
            .for_each(|(i, c)| {
                if !c.is_whitespace() {
                    stacks[i].push(c)
                }
            });
    }

    Stacks::new(stacks)
}

fn solve_part1(s: &mut Stacks, m: &[&str]) -> String {
    for &l in m {
        let cmd = Move::try_from(l).expect("Should be a valid move");
        s.move_single(&cmd);
    }

    s.get_top()
}

fn solve_part2(s: &mut Stacks, m: &[&str]) -> String {
    for &l in m {
        let cmd = Move::try_from(l).expect("Should be a valid move");
        s.move_multiple(&cmd);
    }
    s.get_top()
}

pub fn day_5() {
    let (mut s, m) = prepare_input(INPUT);
    let p1 = solve_part1(&mut s.clone(), &m);
    println!("Day 5.1: {:>12}", p1);
    let p2 = solve_part2(&mut s, &m);
    println!("Day 5.2: {:>12}", p2)
}



#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r"    [D]    
    [N] [C]    
    [Z] [M] [P]
     1   2   3 
    
    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2";

    #[test]
    fn test_split_input() {
        let (s, m) = split_input(TEST);
        s.iter().for_each(|l| println!("{}", l));
        println!("---------------------------------");
        println!("---------------------------------");
        m.iter().for_each(|l| println!("{}", l));
    }

    #[test]
    fn test_parse_stack() {
        let (stack, _) = split_input(TEST);
        parse_stack(&stack);
    }

    #[test]
    fn test_solve_part_1() {
        let (mut s, m) = prepare_input(TEST);
        let tops = solve_part1(&mut s, &m);
        assert_eq!(tops, "CMZ");
    }

    #[test]
    fn test_solve_part_2() {
        let (mut s, m) = prepare_input(TEST);
        let tops = solve_part2(&mut s, &m);
        assert_eq!(tops, "MCD")
    }
}
