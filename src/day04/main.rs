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

fn get_grid(input: &str) -> Vec<Vec<bool>> {
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

fn remove_accessible(mut grid: Vec<Vec<bool>>) -> (Vec<Vec<bool>>, i32) {
    let mut to_remove = Vec::new();
    for (row_idx, row) in grid.iter().enumerate() {
        for (cell_idx, cell) in row.iter().enumerate() {
            if ! *cell {
                continue
            }
            if NEIGHBOUR_OFFSETS
                .iter()
                .filter_map(|(row_offset, col_offset)| {
                    grid.get(row_idx.checked_add_signed(*row_offset)?)?
                        .get(cell_idx.checked_add_signed(*col_offset)?)
                })
                .filter(|&&c| c)
                .count() <4 {
                to_remove.push((row_idx, cell_idx))
            }
        }
    }
    for (row_idx, cell_idx) in to_remove.iter() {
        grid[*row_idx][*cell_idx]  =false
    }
    (grid, to_remove.len() as i32)
}

fn part1(input: &str) -> i32 {
    let grid = get_grid(input);
    let (_, removed_count) = remove_accessible(grid);
    removed_count
}

fn part2(input: &str) -> i32 {
    let mut grid = get_grid(input);
    let mut total = 0;
    loop {
        let (new, removed) = remove_accessible(grid);
        grid = new;
        if removed == 0 {
            break;
        }
        total += removed
    }
    total
}
