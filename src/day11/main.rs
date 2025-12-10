use std::time::{Instant};

fn main() {
    let input = include_str!("input.txt");
    let start = Instant::now();
    println!("Part 1: {}", part1(input));
    let end = Instant::now();
    println!("Time elapsed: {:?}", end - start);

    println!("\n");

    let start = Instant::now();
    println!("Part 2: {}", part2(input));
    let end = Instant::now();
    println!("Time elapsed: {:?}", end - start);
}

fn part1(_input: &str) -> i32 {
    0
}

fn part2(_input: &str) -> i32 {
    0
}
