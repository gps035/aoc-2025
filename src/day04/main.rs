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

fn part1(input: &str) -> i32 {
    let rows: Vec<Vec<bool>> = input
        .split_terminator("\n")
        .map(|line| line.chars().map(|char| char == '@').collect())
        .collect();

    rows.iter()
        .enumerate()
        .map(|(row_idx, row)| {
            let counts = row
                .iter()
                .enumerate()
                .map(|(col_idx, cell)| {
                    let mut other_rows = Vec::from([row_idx]);
                    if row_idx > 0 {
                        other_rows.push(row_idx - 1);
                    }
                    if (row_idx + 1) < rows.len() {
                        other_rows.push(row_idx + 1);
                    }
                    let mut other_cols = Vec::from([col_idx]);
                    if col_idx > 0 {
                        other_cols.push(col_idx - 1);
                    }
                    if (col_idx + 1) < row.len() {
                        other_cols.push(col_idx + 1);
                    }
                    let mut surrounding_indexes = Vec::new();
                    for r in &other_rows[..] {
                        for c in &other_cols {
                            if *r == row_idx && *c == col_idx {
                                continue;
                            }
                            surrounding_indexes.push((*r, *c))
                        }
                    }
                    let count = surrounding_indexes
                        .into_iter()
                        .filter(|(r, c)| rows[*r][*c])
                        .collect::<Vec<(usize, usize)>>();
                    count.len() < 4 && *cell
                })
                .collect::<Vec<bool>>();
            counts.into_iter().filter(|valid| *valid).count() as i32
        })
        .sum()
}

fn part2(_input: &str) -> i32 {
    0
}
