#![allow(unused)]
#![feature(test)]

extern crate test;

use itertools::Itertools;
use z3::{Optimize, ast::Int};

#[derive(Debug)]
struct Machine {
    goal: u32,
    button_group: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl From<&str> for Machine {
    fn from(input: &str) -> Self {
        let mut splits = input.split_whitespace().collect::<Vec<&str>>();
        let joltages = splits.pop().map(Machine::parse_numbers).unwrap();

        let goal = splits[0]
            .to_string()
            .trim_matches(['[', ']'])
            .chars()
            .rev()
            .fold(
                0,
                |acc, ch| {
                    if ch == '#' { (acc << 1) + 1 } else { acc << 1 }
                },
            );

        let button_group = splits
            .iter()
            .skip(1)
            .map(|s| Machine::parse_numbers(s))
            .collect::<Vec<_>>();

        Self {
            goal,
            button_group,
            joltages,
        }
    }
}

impl Machine {
    fn parse_numbers(input: &str) -> Vec<usize> {
        input
            .trim_matches(['{', '}', '(', ')'])
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    }

    fn press(lights: &mut u32, buttons: &[usize]) {
        for button in buttons {
            *lights ^= 1 << button;
        }
    }

    fn solve(&self) -> u64 {
        let opt = Optimize::new();
        let presses: Vec<Int> = (0..self.button_group.len())
            .map(|i| Int::new_const(format!("p_{i}")))
            .collect();

        presses.iter().for_each(|p| opt.assert(&p.ge(0u32)));

        self.joltages.iter().enumerate().for_each(|(j, joltage)| {
            let eqs = self
                .button_group
                .iter()
                .enumerate()
                .filter_map(|(b, buttons)| {
                    if buttons.contains(&j) {
                        Some(&presses[b])
                    } else {
                        None
                    }
                })
                .sum::<Int>();

            opt.assert(&eqs.eq(Int::from_u64(*joltage as u64)));
        });

        let total = presses.iter().sum::<Int>();

        opt.minimize(&total);
        opt.check(&[]);
        let model = opt.get_model().unwrap();
        model.eval(&total, true).unwrap().as_u64().unwrap()
    }
}

fn part1(data: &str) -> u32 {
    data.lines()
        .map(|line| {
            let machine = Machine::from(line);
            let mut presses = 0;
            // find the first match if any
            machine.button_group.iter().powerset().any(|set| {
                let mut lights = 0;
                presses = 0;
                set.iter().for_each(|buttons| {
                    presses += 1;
                    Machine::press(&mut lights, buttons);
                });
                lights == machine.goal
            });
            presses
        })
        .sum()
}

fn part2(data: &str) -> u64 {
    data.lines().map(|line| Machine::from(line).solve()).sum()
}

fn main() {
    let data = std::fs::read_to_string("puzzle.txt").unwrap();
    println!("{}", part1(&data));
    println!("{}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn parse_lights() {
        let input = "[...#.]";
        let expected = 0b00010;
        assert_eq!(
            expected,
            input.trim_matches(['[', ']']).chars().fold(0, |acc, ch| {
                if ch == '#' { (acc << 1) + 1 } else { acc << 1 }
            })
        );
    }

    #[test]
    fn test_press_all_the_buttons() {
        let mut lights = 0;
        let buttons = vec![vec![0], vec![0, 1], vec![0, 1, 2]];
        let wanted = 5;
        for button in &buttons {
            Machine::press(&mut lights, button);
        }
        assert_eq!(lights, wanted);
    }

    #[test]
    fn test_find_actual_solution() {
        let mut lights = 0;
        let buttons = [vec![2, 4], vec![0, 1, 2], vec![1, 2, 3, 4]];
        let expected = [0b10100, 0b10011, 0b01101];
        for (button, wanted) in buttons.iter().zip(&expected) {
            Machine::press(&mut lights, button);
            assert_eq!(lights, *wanted);
        }
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let data = std::fs::read_to_string("puzzle.txt").unwrap();
        b.iter(|| part1(&data));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let data = std::fs::read_to_string("puzzle.txt").unwrap();
        b.iter(|| part2(&data));
    }
}
