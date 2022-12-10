use std::collections::HashMap;

use crate::Solver;

pub struct Day7;

impl Solver for Day7 {
    fn part_1(&self, input: &str) -> String {
        todo!()
    }

    fn part_2(&self, input: &str) -> String {
        todo!()
    }
}

#[derive(Debug)]
enum Entry {
    File(File),
    Dir(Dir),
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

#[derive(Debug, Clone)]
enum Command {
    Cd(Utf8PathBuf),
    Ls,
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Cd(l0), Self::Cd(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[derive(Debug)]
struct Dir(Utf8PathBuf);

#[derive(Debug, PartialEq, Eq)]
struct File(usize, Utf8PathBuf);

use camino::Utf8PathBuf;
use nom::{
    self,
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{multispace0, multispace1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair, terminated},
    Finish, IResult,
};

const LEGAL_CHARS: &str = "abcdefghijklmnopqrstuvwxyz./";

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(take_while1(|c: char| LEGAL_CHARS.contains(c)), Into::into)(i)
}

fn parse_dir(i: &str) -> IResult<&str, Dir> {
    map(preceded(tag("dir "), parse_path), |p| Dir(p))(i)
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    alt((
        map(parse_dir, |d| Entry::Dir(d)),
        map(parse_file, |f| Entry::File(f)),
    ))(i)
}

fn parse_file(i: &str) -> IResult<&str, File> {
    map(
        separated_pair(nom::character::complete::u64, multispace1, parse_path),
        |(size, path)| File(size as _, path),
    )(i)
}

fn parse_cmd(i: &str) -> IResult<&str, Command> {
    let cd = map(terminated(tag("ls"), multispace0), |_| Command::Ls);
    let ls = map(
        separated_pair(tag("cd"), multispace0, parse_path),
        |(_, p)| Command::Cd(p),
    );
    alt((cd, ls))(i)
}

fn line_cmd(i: &str) -> IResult<&str, Command> {
    let (i, cmd) = preceded(tag("$ "), parse_cmd)(i)?;
    Ok((i, cmd))
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(line_cmd, |cmd| Line::Command(cmd)),
        map(parse_entry, |ent| Line::Entry(ent)),
    ))(i)
}

fn crawler(input: &str) -> HashMap<Utf8PathBuf, Entry> {
    let mut pwd = Utf8PathBuf::new();

    let lines = input
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    lines
        .filter_map(|l| dbg!(match l {
            Line::Command(Command::Ls) | Line::Entry(Entry::Dir(_)) => None,
            Line::Command(Command::Cd(path)) => match path.as_str() {
                "/" => {
                    pwd = Utf8PathBuf::from("/");
                    Some((pwd.clone(), Entry::Dir(Dir(pwd.clone()))))
                }
                ".." => {
                    pwd.pop();
                    Some(((pwd.clone()), Entry::Dir(Dir(pwd.clone()))))
                }
                p => {
                    pwd.push(p);
                    Some((path.clone(), Entry::Dir(Dir(pwd.clone()))))
                }
            },
            Line::Entry(Entry::File(file)) => {
                let p = pwd.join(file.1.clone());
                Some((p, Entry::File(file)))
            },
        })
        
    )
        .collect()
}

fn total_size() {}

fn solve_part_1() {}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_ok::assert_ok;

    const TEST: &str = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_crawler() {
        crawler(TEST);
    }

    #[test]
    fn test_stuff() {
        let (rest, cmd) = assert_ok!(parse_line("$ cd .."));
        println!("{:?}", (rest, cmd));
        let (rest, cmd) = assert_ok!(parse_cmd("cd /"));
        println!("{:?}", (rest, cmd));
        let (rest, cmd) = assert_ok!(line_cmd("$ cd /"));
        println!("{:?}", (rest, cmd));
        let (rest, cmd) = assert_ok!(parse_line("$ ls "));
        println!("{:?}", (rest, cmd));
    }

    #[test]
    fn all_input() {
        let lines = TEST.lines().map(|l| {
            // println!("{}", l);
            all_consuming(parse_line)(l).finish().unwrap().1
        });

        for line in lines {
            println!("{:?}", line)
        }
    }

    // #[test_case("$ ls", Command::Ls)]
    // #[test_case("$ cd a", Command::Cd("a".to_string()))]
    // #[test_case("$ cd e", Command::Cd("e".to_string()))]
    // #[test_case("$ cd ..", Command::Cd("..".to_string()))]
    // #[test_case("$ cd d", Command::Cd( "d".to_string()))]

    // fn test_parse_cmd(i: &str, cmd: Command) {
    //     let (_, b) = assert_ok!(cmd_line(i));
    //     dbg!(&b);
    //     assert_eq!(b, cmd)
    // }

    // #[test_case("29116 f", 29116, "f"; )]
    // #[test_case("2557 g", 2557, "g"; )]
    // #[test_case("4060174 j", 4060174, "j"; )]
    // #[test_case("7214296 k", 7214296, "k"; )]
    // #[test_case("14848514 b.txt", 14848514, "b.txt"; )]
    // #[test_case("8504156 c.dat", 8504156, "c.dat"; )]
    // #[test_case("62596 h.lst", 62596, "h.lst"; )]
    // #[test_case("8033020 d.log", 8033020, "d.log"; )]
    // #[test_case("5626152 d.ext", 5626152, "d.ext"; )]
    // fn test_parse_file(i: &str, size: usize, name: &str) {
    //     let (s, f) = assert_ok!(parse_file(i));
    //     // assert_eq!(f, File::new(name, size));
    //     assert_eq!(name, f.name);
    //     assert_eq!(size, f.size);
    //     assert!(s.is_empty());
    // }

    // #[test_case("dir a", "a")]
    // #[test_case("dir d", "d")]
    // #[test_case("dir e", "e")]
    // fn test_parse_dir(i: &str, r: &str) {
    //     let (s, dir) = assert_ok!(parse_dir(i));
    //     assert_eq!(dir, Dir::new(r));
    //     assert!(s.is_empty());
    // }
}
