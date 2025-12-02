fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    input
        .split(",")
        .map(|range| range.split_once("-").unwrap())
        .map(|(start, end)| (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap()))
        .flat_map(|(start, end)| start..end)
        .filter(|id| {
            let s = id.to_string();
            let mid = s.len() / 2;
            s[..mid] == s[mid..]
        })
        .sum()
}

fn part2(_input: &str) -> i32 {
    0
}
