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

fn get_rows(input: &str) -> Vec<Vec<bool>> {
    input
        .split_terminator("\n")
        .map(|line| line.chars().map(|char| char == '@').collect())
        .collect()
}

const NEIGHBOUR_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    // (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn remove_accessible(rows: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    rows.iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, cell)| {
                    (
                        *cell,
                        NEIGHBOUR_OFFSETS
                            .iter()
                            .filter_map(|(row_offset, col_offset)| {
                                rows.get(row_idx.checked_add_signed(*row_offset)?)?
                                    .get(col_idx.checked_add_signed(*col_offset)?)
                            })
                            .filter(|&&c| c)
                            .count(),
                    )
                })
                .map(|(cell, surrounding)| cell && surrounding >= 4)
                .collect()
        })
        .collect()
}

fn count_paper(rows: &Vec<Vec<bool>>) -> i32 {
    rows.iter()
        .map(|row| row.iter().filter(|cell| **cell).count() as i32)
        .sum()
}

fn part1(input: &str) -> i32 {
    let rows = get_rows(input);
    let paper_count = count_paper(&rows);
    let remaining = remove_accessible(&rows);
    paper_count - count_paper(&remaining)
}

fn part2(input: &str) -> i32 {
    let rows = get_rows(input);
    let initial = count_paper(&rows);
    let mut remaining = rows;
    let mut remaining_count = initial;
    loop {
        let last = remaining_count;
        remaining = remove_accessible(&remaining);
        remaining_count = count_paper(&remaining);
        if remaining_count == last {
            break;
        }
    }
    initial - remaining_count
}
