use std::{fs::File, io::Read, path::Path};

#[derive(Debug, Copy, Clone)]
pub struct IdRange {
    pub start: u64,
    pub end: u64,
}

fn get_input(path: &str) -> Vec<IdRange> {
    let path = Path::new(path);
    let mut file = File::open(path).expect("Unable to open file");
    let mut line = String::new();
    file.read_to_string(&mut line).expect("Unable to read file");
    line.trim_end()
        .split(",")
        .map(|id_range| {
            let (start, end) = id_range.split_at(id_range.find('-').unwrap());
            IdRange {
                start: start.parse().unwrap(),
                end: end[1..].parse().unwrap(),
            }
        })
        .collect()
}

fn is_invalid(id: &str) -> bool {
    let midpoint = id.len() / 2;
    let (left, right) = id.split_at(midpoint);
    left == right
}

fn is_invalid2(id: &str) -> bool {
    let primes = [2, 3, 5, 7];
    primes.iter().any(|&n| {
        let length = id.len();
        if length.is_multiple_of(n) {
            let piece = &id[..(length / n)];
            piece.repeat(n) == id
        } else {
            false
        }
    })
}

fn part1(input: Vec<IdRange>) -> u64 {
    input
        .iter()
        .flat_map(|id_range| {
            (id_range.start..=id_range.end)
                .filter(|id| is_invalid(&id.to_string()))
                .collect::<Vec<_>>()
        })
        .sum()
}

fn part2(input: Vec<IdRange>) -> u64 {
    input
        .iter()
        .flat_map(|id_range| {
            (id_range.start..=id_range.end)
                .filter(|id| is_invalid2(&id.to_string()))
                .collect::<Vec<_>>()
        })
        .sum()
}

fn main() {
    let input = get_input("puzzle.txt");
    println!("{:?}", part1(input.clone()));
    println!("{:?}", part2(input));
}
