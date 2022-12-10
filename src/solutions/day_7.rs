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

impl Entry {
    fn is_file(&self) -> bool {
        match self {
            Entry::File(_) => true,
            Entry::Dir(_) => false,
        }
    }

    // fn get_file(&self) -> Option<File> {
    //     match self {
    //         Entry::File(f) => Some(*f.clone()),
    //         Entry::Dir(_) => None,
    //     }
    // }

    fn is_dir(&self) -> bool {
        !self.is_file()
    }
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
struct File(Utf8PathBuf, usize);

use camino::{Utf8Path, Utf8PathBuf};
use itertools::Itertools;
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
        |(size, path)| File(path, size as _),
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

type EntryTree = HashMap<Utf8PathBuf, Entry>;
type DirSizeTree = HashMap<Utf8PathBuf, usize>;

fn crawler(input: &str) -> EntryTree {
    let mut pwd: Utf8PathBuf = Utf8PathBuf::from("/");

    let lines = input
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    lines
        .filter_map(|l| {
            match l {
                Line::Command(Command::Ls) | Line::Entry(Entry::Dir(_)) => None,
                Line::Command(Command::Cd(path)) => match path.as_str() {
                    "/" => {
                        // // pwd = Utf8PathBuf::from("/");
                        // let tmp = Utf8PathBuf::from("/").canonicalize().unwrap();
                        // println!("Canonical form: {:?}", tmp);
                        // // pwd = Utf8PathBuf::from(&mp);
                        // Some((pwd.clone(), Entry::Dir(Dir(pwd.clone()))))
                        Some((pwd.clone(), Entry::Dir(Dir(pwd.clone()))))
                    }
                    ".." => {
                        pwd.pop();
                        Some(((pwd.clone()), Entry::Dir(Dir(pwd.clone()))))
                    }
                    p => {
                        pwd.push(p);
                        // pwd = Utf8PathBuf::from(pwd.canonicalize().unwrap());
                        Some((pwd.clone(), Entry::Dir(Dir(pwd.clone()))))
                    }
                },
                Line::Entry(Entry::File(file)) => {
                    let p = pwd.join(file.0.clone());
                    let file = File(p.clone(), file.1);
                    let file = Entry::File(file);
                    Some((p, file))
                }
            }
        })
        .collect()
}

fn update_parent_size(f: &File, tree: &mut DirSizeTree) {
    for a in f.0.ancestors().skip(1) {
        println!("ancestor: {:8>}", a);
        if let Some(dir) = tree.get_mut(a) {
            *dir += f.1;
        } else {
            tree.insert(a.into(), f.1);
        }
    }
}

fn total_size(map: &EntryTree) -> DirSizeTree {
    let mut dt: DirSizeTree = Default::default();
    map.iter()
        .filter_map(|(_, f)| match f {
            Entry::File(f) => Some(f),
            Entry::Dir(_) => None,
        })
        // .map(|(_, f)| f.get_file().unwrap())
        .for_each(|f| update_parent_size(&f, &mut dt));

    dt
}

fn solve_part_1() {}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_ok::assert_ok;
    use itertools::Itertools;

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
        let entries = crawler(TEST);
        entries
            .iter()
            .sorted_by(|b, a| b.0.cmp(a.0))
            .for_each(|(p, e)| println!("{} @ {:?}", p, e));

        let dt = total_size(&entries);
        for x in dt.iter().sorted_by(|x, y| x.1.cmp(y.1)) {
            println!("{:?}", x)
        }

        let result: usize = dt
            .values()
            .filter(|&&size| dbg!(size) < 100_000)
            .sum1()
            .unwrap();
        assert_eq!(dbg!(result), 95437)
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
