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

fn part2(_input: &str) -> i64 {
    0
}
