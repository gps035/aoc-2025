fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn pick_batteries(input: &str, battery_count: usize) -> impl Iterator<Item = i64> {
    input.lines().map(move |line| {
        let bank = line.as_bytes();
        let mut start = 0;
        let mut result: i64 = 0;
        for remaining in (0..battery_count).rev() {
            let mut selected_idx = start;
            for i in start..(bank.len() - remaining) {
                if bank[i] <= bank[selected_idx] {
                    continue;
                }
                selected_idx = i;
            }
            result *= 10;
            result += (bank[selected_idx] - b'0') as i64;
            start = selected_idx + 1;
        }
        result
    })
}

fn part1(input: &str) -> i64 {
    pick_batteries(input, 2).sum()
}

fn part2(input: &str) -> i64 {
    pick_batteries(input, 12).sum()
}
