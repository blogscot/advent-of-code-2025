// Finds the first occurence of the max value in a subrange
fn find_max(input: &[u32], start: usize, end: usize) -> (u32, usize) {
    let mut index = start;
    let mut max = input[index];
    for i in start..end {
        if input[i] > max {
            max = input[i];
            index = i;
        }
    }
    (max, index)
}

fn calculate_joltage(bank: &str) -> usize {
    let digits = bank
        .split("")
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<u32>>();

    let mut index = 0;
    let block_size = 12;
    (0..block_size)
        .reduce(|joltage, i| {
            let (max, ind) = find_max(&digits, index, digits.len() - (block_size - i - 1));
            index = ind + 1;
            joltage * 10 + max as usize
        })
        .unwrap()
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits = line
                .split("")
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<u32>>();
            let (max, ind) = find_max(&digits, 0, digits.len() - 1);
            let (next_max, _) = find_max(&digits, ind + 1, digits.len());
            max * 10 + next_max
        })
        .sum::<u32>()
}

fn part2(input: &str) -> usize {
    input.lines().map(|bank| calculate_joltage(bank)).sum()
}

fn main() {
    let input = include_str!("../puzzle.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}
