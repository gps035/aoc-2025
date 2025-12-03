fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn pick_batteries(input: &str, battery_count: usize) -> impl Iterator<Item = i64> {
    input
        .split_terminator("\n")
        .map(move |bank| {
            let mut next_possible_index = 0;
            let mut chosen = vec!['0'; battery_count];
            for x in 0..chosen.len() {
                let start = next_possible_index;
                let end = bank.len() - (chosen.len() - x);
                for (i,c) in bank[start..=end].chars().enumerate() {
                    if c > chosen[x] {
                        next_possible_index = start +i+1;
                        chosen[x] = c;
                    }
                }
            }
            String::from_iter(chosen)
                .parse::<i64>()
                .unwrap()
        })
}

fn part1(input: &str) -> i64 {
    return pick_batteries(input, 2).sum()
}

fn part2(input: &str) -> i64 {
    return pick_batteries(input, 12).sum()
}
