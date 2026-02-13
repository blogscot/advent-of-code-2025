#![feature(test)]

extern crate test;

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Eq)]
enum Light {
    On,
    Off,
}

fn parse_numbers(input: &str) -> Vec<usize> {
    input
        .trim_matches(['{', '}', '(', ')'])
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, Eq)]
struct Lights {
    state: Vec<Light>,
}

impl Lights {
    fn new(num: usize) -> Self {
        let all_off = vec![Light::Off; num];
        Self { state: all_off }
    }

    fn apply(&mut self, buttons: &[usize]) {
        use Light::*;
        for button in buttons {
            self.state[*button] = match self.state[*button] {
                On => Off,
                Off => On,
            }
        }
    }
}

fn parse_input(input: &str) -> (Vec<Light>, Vec<Vec<usize>>, Vec<usize>) {
    let mut splits: Vec<_> = input.split_whitespace().collect();
    let joltage = parse_numbers(splits.pop().unwrap());
    let lights = splits[0]
        .trim_matches(['[', ']'])
        .chars()
        .map(|ch| match ch {
            '#' => Light::On,
            '.' => Light::Off,
            _ => unreachable!("Invalid character"),
        })
        .collect::<Vec<_>>();

    let buttons = splits.iter().skip(1).map(|s| parse_numbers(s)).collect();
    (lights, buttons, joltage)
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut presses = 0;
            let (answer, buttons, _) = parse_input(line);
            buttons.iter().powerset().any(|set| {
                let mut lights = Lights::new(answer.len());
                presses = 0;
                set.iter().for_each(|btns| {
                    presses += 1;
                    lights.apply(btns);
                });
                lights.state == answer
            });
            presses
        })
        .sum::<u32>()
}

fn main() {
    let data = std::fs::read_to_string("puzzle.txt").unwrap();
    println!("{}", part1(&data));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        let data = std::fs::read_to_string("puzzle.txt").unwrap();
        assert_eq!(part1(&data), 578);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let data = std::fs::read_to_string("puzzle.txt").unwrap();
        b.iter(|| part1(&data));
    }
}
