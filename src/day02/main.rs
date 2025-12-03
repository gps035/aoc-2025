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
