use std::fmt::Debug;

fn part1(input: &str) -> u64 {
    let mut lines = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let ops = lines.pop().unwrap();
    let nums = lines
        .iter()
        .map(|line| {
            line.iter()
                .map(|val| val.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    (0..ops.len())
        .map(|col| match ops[col] {
            "+" => (0..nums.len()).map(|row| nums[row][col]).sum(),
            "*" => (0..nums.len()).map(|row| nums[row][col]).product(),
            _ => 0,
        })
        .sum()
}

pub fn transpose<T: Debug + Clone>(v: Vec<Vec<T>>) -> impl Iterator<Item = Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();

    (0..len).map(move |_| {
        iters
            .iter_mut()
            .map(|n| n.next().unwrap())
            .collect::<Vec<T>>()
    })
}

pub fn get_column_numbers(nums: &[String]) -> Vec<u64> {
    let chars = nums
        .iter()
        .map(|num| num.chars().collect())
        .collect::<Vec<Vec<char>>>();

    transpose(chars)
        .map(|row| row.iter().collect::<String>())
        .map(|row| row.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines().collect::<Vec<&str>>();
    let last_line = lines.pop().unwrap();
    let op_chars = last_line.chars().collect::<Vec<char>>();

    let mut column_markers = op_chars
        .iter()
        .enumerate()
        .filter_map(|(index, op)| if *op != ' ' { Some(index) } else { None })
        .collect::<Vec<usize>>();
    column_markers.push(op_chars.len() + 1);

    let line_segments = lines
        .iter()
        .map(|line| {
            column_markers
                .iter()
                .zip(column_markers.clone().iter().skip(1))
                .map(|(&a, &b)| line[a..b - 1].to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<String>>>();

    let ops = last_line
        .split_whitespace()
        .map(|op| op.to_string())
        .collect::<Vec<String>>();

    transpose(line_segments)
        .map(|row| get_column_numbers(&row))
        .zip(ops)
        .map(|(row, op)| match op.as_str() {
            "+" => row.iter().sum(),
            "*" => row.iter().product(),
            _ => 0,
        })
        .sum()
}

fn main() {
    let input = include_str!("../puzzle.txt");

    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}
