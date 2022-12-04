use itertools::Itertools;
use std::collections::BinaryHeap;

pub(crate) fn day_1() {
    let input = include_str!("../input/day_1/input");
    let lines = input.lines().map(|l| l.parse::<i64>().unwrap_or(0));

    let x = lines.group_by(|&i| i != 0);

    let mut w: BinaryHeap<i64> = x
        .into_iter()
        .filter(|(a, _b)| *a)
        .map(|(_a, b)| b.sum())
        .collect();

    let mut sum = 0;
    for i in 0..3 {
        let val = w.pop().unwrap();
        sum += val;
        if i == 0 {
            println!("Day 1.1: {:8}", val);
        }
    }
    println!("Day 1.2: {:8}", sum);
}