use std::ops::{Deref, DerefMut};

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
    File(usize),
    Dir(Directory),
    Command(Command),
}

#[derive(Debug)]
enum Command {
    Cd(String),
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
struct Directory {
    name: String,
    entries: Vec<Entry>,
}

impl PartialEq for Directory {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            entries: vec![],
        }
    }
}

impl Deref for Directory {
    type Target = Vec<Entry>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl DerefMut for Directory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}

#[derive(Debug, PartialEq, Eq)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
    }
}

use nom::{
    self,
    branch::{alt, permutation},
    bytes::complete::{is_not, tag, take_while},
    character::complete::{
        alphanumeric1, anychar, char, digit1, line_ending, multispace0, multispace1, one_of,
    },
    combinator::{map, map_parser, map_res},
    multi::{self, many1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

fn parse_dir(i: &str) -> IResult<&str, Directory> {
    let (i, name) = preceded(tag("dir "), not_space)(i)?;
    Ok((i, Directory::new(name)))
}

fn not_space(s: &str) -> IResult<&str, &str> {
    is_not(" \t\r\n")(s)
}

fn parse_file(i: &str) -> IResult<&str, File> {
    let (i, size) = map_res(digit1, |s: &str| s.parse::<usize>())(i)?;
    let (i, _) = multispace1(i)?;
    let (i, name) = not_space(i)?;

    Ok((i, File::new(name, size)))
}

fn parse_cmd_cd(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("ls")(i)?;
    Ok((i, Command::Ls))
}

fn cmd_cd(i: &str) -> IResult<&str, Command> {
    let (i, (_, s)) = separated_pair(tag("cd"), multispace0, not_space)(i)?;
    Ok((i, Command::Cd(s.to_string())))
}
fn cmd_ls(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("ls")(i)?;
    Ok((i, Command::Ls))
}

fn parse_cmd(i: &str) -> IResult<&str, Command> {
    alt((cmd_cd, cmd_ls))(i)
}

fn parse_cmd_line(i: &str) -> IResult<&str, Command> {
    let (i, cmd) = preceded(tag("$ "), alt((cmd_ls, cmd_cd)))(i)?;
    Ok((i, cmd))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_ok::assert_ok;
    use test_case::test_case;

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
    7214296 k";

    #[test_case("$ ls", Command::Ls)]
    #[test_case("$ cd a", Command::Cd("a".to_string()))]
    #[test_case("$ cd e", Command::Cd("e".to_string()))]
    #[test_case("$ cd ..", Command::Cd("..".to_string()))]
    #[test_case("$ cd d", Command::Cd( "d".to_string()))]

    fn test_parse_cmd(i: &str, cmd: Command) {
        let (_, b) = assert_ok!(parse_cmd_line(i));
        dbg!(&b);
        assert_eq!(b, cmd)
    }

    #[test_case("29116 f", 29116, "f"; )]
    #[test_case("2557 g", 2557, "g"; )]
    #[test_case("4060174 j", 4060174, "j"; )]
    #[test_case("7214296 k", 7214296, "k"; )]
    #[test_case("14848514 b.txt", 14848514, "b.txt"; )]
    #[test_case("8504156 c.dat", 8504156, "c.dat"; )]
    #[test_case("62596 h.lst", 62596, "h.lst"; )]
    #[test_case("8033020 d.log", 8033020, "d.log"; )]
    #[test_case("5626152 d.ext", 5626152, "d.ext"; )]
    fn test_parse_file(i: &str, size: usize, name: &str) {
        let (s, f) = assert_ok!(parse_file(i));
        // assert_eq!(f, File::new(name, size));
        assert_eq!(name, f.name);
        assert_eq!(size, f.size);
        assert!(s.is_empty());
    }

    #[test_case("dir a", "a")]
    #[test_case("dir d", "d")]
    #[test_case("dir e", "e")]
    fn test_parse_dir(i: &str, r: &str) {
        let (s, dir) = assert_ok!(parse_dir(i));
        assert_eq!(dir, Directory::new(r));
        assert!(s.is_empty());
    }
}
