use std::time::Instant;

pub(crate) fn run_solution(input: &str, part1: fn(&str) -> i64, part2: fn(&str) -> i64) {
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
