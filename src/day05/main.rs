#[path = "../utilities.rs"]
mod utilities;

fn main() {
    utilities::run_solution(include_str!("input.txt"), part1, part2);
}

fn get_fresh_ranges(ranges: &str) -> Vec<(i64, i64)> {
    ranges
        .split_terminator("\n")
        .map(|range| range.split_once("-").unwrap())
        .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
        .collect()
}

fn part1(input: &str) -> i64 {
    let (ranges, list) = input.split_once("\n\n").unwrap();
    let fresh_ranges = get_fresh_ranges(ranges);
    list.split_terminator("\n")
        .map(|id| id.parse::<i64>().unwrap())
        .filter(|id| {
            fresh_ranges
                .iter()
                .any(|(start, end)| id >= start && id <= end)
        })
        .count() as i64
}

fn part2(input: &str) -> i64 {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let mut fresh_ranges = get_fresh_ranges(ranges);
    fresh_ranges.sort_by(|(a, _), (b, _)| a.cmp(b));
    let mut combined_ranges = Vec::with_capacity(fresh_ranges.len());
    for (start, end) in fresh_ranges {
        if combined_ranges.is_empty() {
            combined_ranges.push((start, end));
            continue;
        }
        let (last_start, last_end) = combined_ranges.pop().unwrap();
        if start > last_end {
            combined_ranges.push((last_start, last_end));
            combined_ranges.push((start, end));
            continue;
        }
        if end < last_end {
            combined_ranges.push((last_start, last_end));
            continue;
        }
        combined_ranges.push((last_start, end));
    }
    combined_ranges
        .iter()
        .fold(0, |acc, (start, end)| acc + 1 + (end - start))
}
