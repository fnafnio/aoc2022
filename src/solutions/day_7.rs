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
struct Dir(Utf8PathBuf);

#[derive(Debug, PartialEq, Eq)]
struct File(usize, Utf8PathBuf);

use camino::Utf8PathBuf;
use nom::{
    self,
    branch::{alt, permutation},
    bytes::complete::{is_not, tag, take_while, take_while1},
    character::complete::{
        alphanumeric1, anychar, char, digit1, line_ending, multispace0, multispace1, one_of,
    },
    combinator::{map, map_parser, map_res},
    multi::{self, many1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

fn parse_dir(i: &str) -> IResult<&str, Dir> {
    map(preceded(tag("dir "), parse_path), |p| Dir(p))(i)
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    alt((
        map(parse_dir, |d| Entry::Dir((d))),
        map(parse_file, |f| Entry::File(f)),
    ))(i)
}

fn not_space(s: &str) -> IResult<&str, &str> {
    is_not(" \t\r\n")(s)
}

fn parse_file(i: &str) -> IResult<&str, File> {
    map(
        separated_pair(nom::character::complete::u64, multispace1, parse_path),
        |(size, path)| File(size as _, path),
    )(i)
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

fn line_cmd(i: &str) -> IResult<&str, Command> {
    let (i, cmd) = preceded(tag("$ "), alt((cmd_ls, cmd_cd)))(i)?;
    Ok((i, cmd))
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_cmd, |cmd| Line::Command(cmd)),
        map(parse_entry, |ent| Line::Entry(ent)),
    ))(i)
}

// fn line_entry(i:&str ) -> IResult<&str, Entry> {

// }
#[cfg(test)]
mod tests {
    use super::*;
    use assert_ok::assert_ok;
    use nom::{combinator::all_consuming, Finish};
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

    #[test]
    fn all_input() {
        let lines = TEST
            .lines()
            .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);
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
