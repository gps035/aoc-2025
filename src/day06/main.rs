#[path = "../utilities.rs"]
mod utilities;

fn main() {
    utilities::run_solution(include_str!("input.txt"), part1, part2);
}

fn part1(input: &str) -> i64 {
    let mut lines = input
        .split_terminator("\n")
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let operators = lines.pop().unwrap();
    operators
        .iter()
        .enumerate()
        .map(|(index, op)| {
            let numbers = lines
                .iter()
                .map(|line| line[index])
                .map(|item| item.parse::<i64>().unwrap());
            if *op == "*" {
                return numbers.product::<i64>();
            }
                return numbers.sum();
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let lines = input
        .split_terminator("\n")
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>();
    let mut sums = Vec::new();
    let mut current_numbers = Vec::new();
    for index in (0..lines[0].len()).rev() {
         let mut column = String::from_utf8(lines.iter().map(|line| {
            (*line)[index]
        }).collect::<Vec<u8>>()).unwrap();
        if column.ends_with("*") {
            column.pop();
            current_numbers.push(column.trim().parse().unwrap());
            sums.push(current_numbers.iter().product::<i64>());
            current_numbers.clear();
            continue
        }
        if column.ends_with("+") {
            column.pop();
            current_numbers.push(column.trim().parse().unwrap());
            sums.push(current_numbers.iter().sum::<i64>());
            current_numbers.clear();
            continue
        }
        if column.trim().is_empty() {
            continue
        }
        current_numbers.push(column.trim().parse().unwrap());
    }
    sums.iter().sum()
}
