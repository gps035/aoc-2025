#[path = "../utilities.rs"]
mod utilities;

fn main() {
    utilities::run_solution(include_str!("input.txt"), part1, part2);
}

fn parse_input(input: &str) -> impl Iterator<Item = i32> {
    input.split_terminator("\n").map(|line| {
        let turns = line[1..].parse::<i32>().unwrap();
        if line.starts_with('L') { -turns } else { turns }
    })
}

fn part1(input: &str) -> i64 {
    let start_position: i32 = 50;
    parse_input(input)
        .fold(
            (start_position, 0),
            |(last_position, zero_count), movement| {
                let new_position = (last_position + movement).rem_euclid(100);
                (new_position, zero_count + (new_position == 0) as i64)
            },
        )
        .1
}

fn part2(input: &str) -> i64 {
    let start_position: i32 = 50;
    parse_input(input)
        .fold(
            (start_position, 0),
            |(mut position, mut zero_count), movement| {
                for _ in 0..movement.abs() {
                    position = (position + movement.signum()).rem_euclid(100);
                    zero_count += (position == 0) as i64
                }
                (position, zero_count)
            },
        )
        .1
}
