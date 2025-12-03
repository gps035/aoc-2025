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

fn parse_input(input: &str) -> impl Iterator<Item = i32> {
    input.split_terminator("\n").map(|line| {
        let turns = line[1..].parse::<i32>().unwrap();
        if line.starts_with('L') { -turns } else { turns }
    })
}

fn part1(input: &str) -> i32 {
    let start_position: i32 = 50;
    parse_input(input)
        .fold(
            (start_position, 0),
            |(last_position, zero_count), movement| {
                let new_position = (last_position + movement).rem_euclid(100);
                (new_position, zero_count + (new_position == 0) as i32)
            },
        )
        .1
}

fn part2(input: &str) -> i32 {
    let start_position: i32 = 50;
    parse_input(input)
        .fold(
            (start_position, 0),
            |(mut position, mut zero_count), movement| {
                for _ in 0..movement.abs() {
                    position = (position + movement.signum()).rem_euclid(100);
                    zero_count += (position == 0) as i32
                }
                (position, zero_count)
            },
        )
        .1
}
