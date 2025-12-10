use std::collections::{HashMap, HashSet};

#[path = "../utilities.rs"]
mod utilities;

fn main() {
    utilities::run_solution(include_str!("input.txt"), part1, part2);
}

fn get_start_column(lines: &Vec<&[u8]>) -> usize {
    lines[0].iter().position(|&r| r == b'S').unwrap()
}

fn get_lines(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

fn part1(input: &str) -> i64 {
    let lines = get_lines(input);
    let mut columns = HashSet::from([get_start_column(&lines)]);
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
    let lines = get_lines(input);

    fn get_paths_to_bottom(
        row: usize,
        col: usize,
        lines: &[&[u8]],
        memo: &mut HashMap<(usize, usize), i64>,
    ) -> i64 {
        if lines.len() - 1 == row {
            return 1;
        }

        if let Some(&known) = memo.get(&(row, col)) {
            return known;
        }

        if lines[row][col] == b'^' {
            let left = get_paths_to_bottom(row + 1, col - 1, lines, memo);
            let right = get_paths_to_bottom(row + 1, col + 1, lines, memo);
            let result = left + right;
            memo.insert((row, col), result);
            return result;
        }

        let result = get_paths_to_bottom(row + 1, col, lines, memo);
        memo.insert((row, col), result);
        result
    }
    get_paths_to_bottom(0, get_start_column(&lines), &lines, &mut (HashMap::new()))
}
