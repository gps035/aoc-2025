#[path = "../utilities.rs"]
mod utilities;

fn main() {
    utilities::run_solution(include_str!("input.txt"), part1, part2);
}

fn part1(input: &str) -> i64 {
    let mut lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    lines
        .pop()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(index, op)| {
            let numbers = lines.iter().map(|line| line[index].parse::<i64>().unwrap());
            return if *op == "*" {
                numbers.product::<i64>()
            } else {
                numbers.sum()
            };
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let lines: Vec<&[u8]> = input.split_terminator("\n").map(str::as_bytes).collect();
    let mut sum = 0;
    let mut numbers = Vec::new();
    for index in (0..lines[0].len()).rev() {
        let column: String = lines.iter().map(|line| line[index] as char).collect();
        let op = column.chars().last().unwrap();

        let Ok(num) = column.trim_end_matches(['*', '+']).trim().parse::<i64>() else {
            continue;
        };
        numbers.push(num);
        if op == '*' {
            sum += numbers.iter().product::<i64>();
            numbers.clear();
            continue;
        }
        if op == '+' {
            sum += numbers.iter().sum::<i64>();
            numbers.clear();
            continue;
        }
    }
    sum
}
