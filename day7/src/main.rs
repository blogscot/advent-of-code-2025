pub fn part1(grid: &[&str]) -> u32 {
    let mut line: Vec<bool> = grid
        .iter()
        .take(1)
        .flat_map(|line| line.chars().map(|ch| ch == 'S'))
        .collect();

    let mut splits = 0;
    grid.iter().skip(1).for_each(|row| {
        row.chars().enumerate().for_each(|(i, ch)| {
            if line[i] && ch == '^' {
                splits += 1;
                line[i] = false;
                line[i - 1] = true;
                line[i + 1] = true;
            }
        });
    });
    splits
}

pub fn part2(grid: &[&str]) -> u64 {
    let line = grid.first().unwrap();
    let mut counts = line
        .chars()
        .map(|ch| if ch == 'S' { 1 } else { 0 })
        .collect::<Vec<u64>>();

    grid.iter().for_each(|row| {
        row.chars().enumerate().for_each(|(i, ch)| {
            if counts[i] > 0 && ch == '^' {
                counts[i - 1] += counts[i];
                counts[i + 1] += counts[i];
                counts[i] = 0;
            }
        });
    });
    counts.iter().sum()
}

fn main() {
    let data = include_str!("../puzzle.txt");
    let grid = data.lines().collect::<Vec<&str>>();

    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}
