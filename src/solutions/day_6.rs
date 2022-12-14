use crate::Solver;
use itertools::Itertools;

const START_OF_PACKET_LEN: usize = 4;
const START_OF_MSG_LEN: usize = 14;

pub struct Day;

impl Solver for Day {
    fn part_1(&self, input: &str) -> String {
        let (_, s) = find_marker(input, START_OF_PACKET_LEN)
            .expect("There has to bee a marker in the input");
        s.to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let (_, s) =
            find_marker(input, START_OF_MSG_LEN).expect("There has to bee a marker in the input");
        s.to_string()
    }
}

fn find_marker(l: &str, size: usize) -> Option<(&str, usize)> {
    for (i, t) in l.as_bytes().windows(size).enumerate() {
        if t.iter().unique().count() == size {
            let pos = i + size;
            return Some((&l[i..pos], pos));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{find_marker, START_OF_MSG_LEN, START_OF_PACKET_LEN};

    const TEST: &[(&str, usize, usize)] = &[
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    #[test]
    fn test_sop() {
        for &(l, so_packet, so_msg) in TEST.iter() {
            let (_mark, pos) =
                find_marker(l, START_OF_PACKET_LEN).expect("there should be a marker");
            assert_eq!(so_packet, pos);
            let (_mark, pos) = find_marker(l, START_OF_MSG_LEN).expect("there should be a marker");
            assert_eq!(so_msg, pos);
        }
    }
}
