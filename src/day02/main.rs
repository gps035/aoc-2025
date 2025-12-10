#[path = "../utilities.rs"]
mod utilities;

fn main() {
    utilities::run_solution(include_str!("input.txt"), part1, part2);
}

fn all_ids(input: &str) -> impl Iterator<Item = i64> {
    input
        .split(",")
        .map(|range| range.split_once("-").unwrap())
        .map(|(start, end)| (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap()))
        .flat_map(|(start, end)| start..=end)
}

fn part1(input: &str) -> i64 {
    all_ids(input)
        .filter(|id| {
            let s = id.to_string();
            let mid = s.len() / 2;
            s[..mid] == s[mid..]
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    all_ids(input)
        .filter(|id| {
            let s = id.to_string();
            let length = s.len();
            for n in 1..=(length / 2) {
                let expected = s[..n].repeat(length / n);
                if expected == s {
                    return true;
                }
            }
            false
        })
        .sum()
}
