use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let input = include_str!("input.txt");
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Part 1: {}", part1(input));
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time elapsed: {:?}", end - start);

    println!("\n");

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Part 2: {}", part2(input));
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("Time elapsed: {:?}", end - start);
}

fn part1(_input: &str) -> i32 {
    0
}

fn part2(_input: &str) -> i32 {
    0
}
