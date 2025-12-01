fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let start_position: i32 = 50;
    input
        .split_terminator("\n")
        .map(|line| {
            let turns = line[1..].parse::<i32>().unwrap();
            if line.starts_with('L') { -turns } else { turns }
        })
        .fold(
            (start_position, 0),
            |(last_position, zero_count), movement| {
                let new_position = (last_position + movement).rem_euclid(100);
                (new_position, zero_count + (new_position == 0) as i32)
            },
        )
        .1
}

fn part2(_input: &str) -> i32 {
    0
}
