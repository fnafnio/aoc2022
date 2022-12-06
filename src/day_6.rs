use itertools::Itertools;
use std::slice::Windows;

const INPUT: &str = include_str!("../input/day_6/input");

pub fn day_6() {
    let (marker, pos) = find_start_of_packet(INPUT).expect("There has to bee a marker in the input");
    println!("Day 6.1: {:6} Marker: {}", pos, marker)
}

fn find_start_of_packet(l: &str) -> Option<(&str, usize)> {
    for (i, t) in l.as_bytes().windows(4).enumerate() {
        if t.iter().unique().count() == 4 {
            let pos = i + 4;
            return Some((&l[i..pos], pos));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::find_start_of_packet;

    const TEST: &[(&str, usize)] = &[
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    #[test]
    fn first_test() {
        for &(l, r_pos) in TEST.iter() {
            let (_mark, pos) = find_start_of_packet(l).expect("there should be a marker");
            assert_eq!(r_pos, pos);
        }
    }
}
