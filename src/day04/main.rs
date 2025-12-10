#[path = "../utilities.rs"]
mod utilities;

fn main() {
    utilities::run_solution(include_str!("input.txt"), part1, part2);
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

fn remove_accessible(mut grid: Vec<Vec<bool>>) -> (Vec<Vec<bool>>, i64) {
    let mut removed = 0;
    for row_idx in 0..grid.len() {
        for cell_idx in 0..grid[row_idx].len() {
            if !grid[row_idx][cell_idx] {
                continue;
            }
            let neighbours = NEIGHBOUR_OFFSETS
                .iter()
                .filter_map(|(row_offset, col_offset)| {
                    grid.get(row_idx.checked_add_signed(*row_offset)?)?
                        .get(cell_idx.checked_add_signed(*col_offset)?)
                })
                .filter(|&&c| c)
                .count();
            if neighbours >= 4 {
                continue;
            }
            grid[row_idx][cell_idx] = false;
            removed += 1;
        }
    }
    (grid, removed)
}

fn part1(input: &str) -> i64 {
    let grid = get_grid(input);
    let (_, removed_count) = remove_accessible(grid);
    removed_count
}

fn part2(input: &str) -> i64 {
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
