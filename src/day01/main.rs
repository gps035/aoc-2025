fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
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
            |(last_position, zero_count), movement| {
                let mut new_zero_count = zero_count;
                let mut new_position = last_position;
                for n in 0..movement.abs() {
                    if movement < 0 {
                        new_position -= 1
                    } else {
                        new_position += 1
                    }
                    if new_position == -1 {
                        new_position = 99
                    }
                    if new_position == 100 {
                        new_position = 0
                    }
                    if new_position == 0 {
                        new_zero_count += 1
                    }
                }
                (new_position, new_zero_count)
            },
        )
        .1
}
