#[path = "../utilities.rs"]
mod utilities;

const START_INDICATOR: u8 = b'S';
const SPLITTER_INDICATOR: u8 = b'^';

fn main() {
    utilities::run_solution(include_str!("input.txt"), part1, part2);
}

fn get_lines(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

fn part1(input: &str) -> i64 {
    let lines = get_lines(input);
    let mut active: Vec<bool> = lines[0].iter().map(|&c| c == START_INDICATOR).collect();
    let mut splits = 0;

    for line in lines {
        let mut next_active = active.clone();
        for (i, &is_active) in active.iter().enumerate() {
            if is_active && line[i] == SPLITTER_INDICATOR {
                next_active[i - 1] = true;
                next_active[i + 1] = true;
                next_active[i] = false;
                splits += 1;
            }
        }
        active = next_active;
    }
    splits
}

fn part2(input: &str) -> i64 {
    let lines: Vec<&[u8]> = get_lines(input);
    let mut path_counts = vec![vec![0; lines[0].len()]; lines.len()];

    for row in (0..path_counts.len()).rev() {
        for col in 0..path_counts[row].len() {
            path_counts[row][col] = if row == lines.len() - 1 {
                1
            } else if lines[row][col] == SPLITTER_INDICATOR {
                path_counts[row + 1][col - 1] + path_counts[row + 1][col + 1]
            } else {
                path_counts[row + 1][col]
            };
        }
    }

    path_counts[0][lines[0]
        .iter()
        .position(|&pos| pos == START_INDICATOR)
        .expect("There is no starting indicator")]
}
