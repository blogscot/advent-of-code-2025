use std::{fs::File, io::Read, path::Path};

#[derive(Debug)]
struct Rotation {
    direction: char,
    value: i32,
}

#[derive(Debug)]
struct Dial {
    position: i32,
}

impl Dial {
    pub fn turn(&mut self, rotation: &Rotation) {
        match rotation.direction {
            'R' => self.position = (self.position + rotation.value) % 100,
            'L' => self.position = (self.position + 100 - rotation.value) % 100,
            _ => unreachable!(),
        }
    }
    pub fn rotate(&mut self, rotation: &Rotation) -> usize {
        let zeros = match rotation.direction {
            'R' => (self.position + 1..=self.position + rotation.value)
                .filter(|&pos| pos % 100 == 0)
                .count(),
            'L' => (self.position - rotation.value..self.position)
                .rev()
                .filter(|&pos| pos % 100 == 0)
                .count(),
            _ => unreachable!(),
        };
        self.turn(rotation);
        zeros
    }
}

fn get_input(path: &str) -> Vec<Rotation> {
    let path = Path::new(path);
    let mut file = File::open(path).expect("Unable to open file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Unable to read file");
    buf.lines()
        .map(|line| {
            let (direction, value) = line.split_at(1);
            Rotation {
                direction: direction.chars().next().unwrap(),
                value: value.parse().unwrap(),
            }
        })
        .collect()
}

fn part1(rotations: &[Rotation]) -> u32 {
    let mut dial = Dial { position: 50 };
    let mut password = 0;

    rotations.iter().for_each(|rotation| {
        dial.turn(rotation);
        if dial.position == 0 {
            password += 1;
        }
    });
    password
}

fn part2(rotations: &[Rotation]) -> usize {
    let mut dial = Dial { position: 50 };
    rotations.iter().map(|rotation| dial.rotate(rotation)).sum()
}

fn main() {
    let rotations = get_input("puzzle.txt");
    println!("{}", part1(&rotations));
    println!("{}", part2(&rotations));
}
