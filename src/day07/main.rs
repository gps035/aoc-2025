use std::collections::HashSet;

#[path = "../utilities.rs"]
mod utilities;

fn main() {
    utilities::run_solution(include_str!("input.txt"), part1, part2);
}

fn get_start_column(line: &[u8]) -> usize {
    line.iter().position(|&r| r == b'S').unwrap()
}

fn get_lines(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

fn part1(input: &str) -> i64 {
    let lines = get_lines(input);
    let mut columns = HashSet::from([get_start_column(&lines[0])]);
    let mut splits = 0;
    for line in lines {
        let mut new_columns = HashSet::new();
        for beam in &columns {
            if line[*beam] == b'^' {
                new_columns.insert(beam - 1);
                new_columns.insert(beam + 1);
                splits += 1;
                continue;
            }
            new_columns.insert(*beam);
        }
        columns = new_columns;
    }
    splits
}

fn part2(input: &str) -> i64 {
    let lines: Vec<&[u8]> = get_lines(input);
    let rows = lines.len();
    let columns = lines[0].len();

    let mut paths_to_bottom = vec![vec![0; columns]; rows];

    for row in (0..rows).rev() {
        for col in 0..columns {
            let mut value = 1;
            if row < rows - 1 {
                let below = &paths_to_bottom[row + 1];
                value = below[col];
                if lines[row][col] == b'^' {
                    value = below[col - 1] + below[col + 1];
                }
            }

            paths_to_bottom[row][col] = value;
        }
    }

    paths_to_bottom[0][get_start_column(lines[0])]
}
