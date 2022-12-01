// include_str!("../input/puzzle.md");

mod aoc_types;

fn main() {
    day_1();

    // for i in guys {
    //     println!("{:?}", i)
    // }
}

fn day_1() {
    let input = include_str!("../input/day_1/input");
    let mut guys: Vec<aoc_types::Elf> = input
        .split("\n\n")
        .map(|guy| aoc_types::Elf::from(guy))
        .collect();
    guys.sort_by(|a, b| b.cmp(a));

    let big_3: i64 = guys
        .iter()
        .take(3)
        .enumerate()
        .map(|(nr, e)| {
            println!("Biggest {}: {}", nr, e.sum);
            e.sum
        })
        .fold(0, |acc, x| acc + x);
    println!("total: {}", big_3);
}
