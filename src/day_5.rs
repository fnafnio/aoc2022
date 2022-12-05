use std::collections::VecDeque;

use itertools::Itertools;

const TEST: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

fn split_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    // let it = input.lines();
    // let stack: Vec<&str> = it.clone().take_while(|s| !s.is_empty()).collect();
    //
    // let moves = it.skip(stack.len() + 1).collect();

    let (stack, moves) = input
        .split_once("\n\n")
        .expect("there should be an empty line!");
    let (stack, moves) = (stack.lines().collect(), moves.lines().collect());

    (stack, moves)
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
        println!("{}", value);
        let mut it = value
            .split(" ")
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<usize>().expect("failed to parse to int") - 1);

        let (cnt, src, dst) = it.tuples().next().expect("something went wrong");
        // let cnt: usize = it
        //     .next()
        //     .expect("command too short")
        //     .parse()
        //     .map_err(|_| "cnt not an usize")?;
        // let src: usize = it
        //     .next()
        //     .expect("cnt command too short")
        //     .parse()
        //     .map_err(|_| "not an usize")?;
        // let dst: usize = it
        //     .next()
        //     .expect("cnt command too short")
        //     .parse()
        //     .map_err(|_| "not an usize")?;

        Ok(Self { src, dst, cnt })
    }
}

#[derive(Debug, Default)]
struct Stacks {
    stacks: Vec<VecDeque<char>>,
}

impl Stacks {
    fn new(stacks: Vec<VecDeque<char>>) -> Self {
        Self { stacks }
    }

    fn follow_order(&mut self, order: &Move) {
        assert!(order.src < self.stacks.len() && order.dst < self.stacks.len());

        for i in 0..order.cnt {
            let c = self.stacks[order.src]
                .pop_back()
                .expect("stack already empty");
            self.stacks[order.dst].push_back(c);
        }
    }

    fn draw(&self) {
        self.stacks.iter().for_each(|s| {
            s.iter().for_each(|c| print!("{}", c));
            println!("");
        });
    }
}

fn parse_stack(input: &[&str]) -> Stacks {
    let mut it = input.iter().rev();
    let &last = it.next().expect("Empty input");

    let mut stacks: Vec<VecDeque<char>> = vec![];
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
                    stacks[i].push_back(c)
                }
            });
    }
    // fun_name(&stacks);

    Stacks::new(stacks)
}

fn solve_part1(input: &str) {
    let (s, m) = split_input(input);
    let mut s = parse_stack(&s);
    // let mut s = Stacks::new(s);
    println!("---------------------------------");
    s.draw();
    println!("---------------------------------");
    for l in m {
        let cmd = Move::try_from(l).expect("Should be a valid move");
        s.follow_order(&cmd);
        println!("{:?}", cmd);
        s.draw();
        println!("---------------------------------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_run_stuff() {
        solve_part1(TEST);
    }
}
