fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    input
        .split_terminator("\n")
        .map(|bank| {
            let mut initial = (0, '0');
            for (i, c) in bank.chars().enumerate() {
                if initial.1 == '9' || i == bank.len() - 1 {
                    break;
                }
                if c > initial.1 {
                    initial = (i, c);
                }
            }
            let mut secondary = '0';
            for (i, c) in bank[(initial.0 + 1)..].chars().enumerate() {
                if c > secondary {
                    secondary = c;
                }
            }
            String::from_iter([initial.1, secondary])
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

fn part2(_input: &str) -> i32 {
    0
}
