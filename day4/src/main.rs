fn is_movable(grid: &[Vec<char>], width: usize, height: usize, x: usize, y: usize) -> bool {
    if grid[y][x] != '@' {
        return false;
    }
    let deltas: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    deltas
        .iter()
        .map(|&(xd, yd)| {
            let new_x = x as i32 + xd;
            let new_y = y as i32 + yd;
            (0..width).contains(&(new_x as usize))
                && (0..height).contains(&(new_y as usize))
                && grid[new_y as usize][new_x as usize] == '@'
        })
        .filter(|&x| x)
        .count()
        < 4
}

fn part1(grid: &[Vec<char>], width: usize, height: usize) -> usize {
    (0..height)
        .flat_map(|y| (0..width).map(move |x| is_movable(grid, width, height, x, y)))
        .filter(|&x| x)
        .count()
}

fn remove_movable(grid: &[Vec<char>], width: usize, height: usize) -> (u32, Vec<Vec<char>>) {
    let mut removed: u32 = 0;
    let mut new_grid = grid.to_vec();
    (0..height).for_each(|y| {
        (0..width).for_each(|x| {
            if is_movable(grid, width, height, x, y) {
                new_grid[y][x] = '.';
                removed += 1;
            }
        })
    });
    (removed, new_grid)
}

fn part2(grid: &[Vec<char>], width: usize, height: usize) -> u32 {
    let mut total: u32 = 0;
    let mut updated = grid.to_vec();
    loop {
        let (removed, new_grid) = remove_movable(&updated, width, height);
        total += removed;
        updated = new_grid;
        if removed == 0 {
            return total;
        }
    }
}

fn main() {
    let data = include_str!("../puzzle.txt");
    let grid = data
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let height = grid.len();
    let width = grid[0].len();

    println!("{:?}", part1(&grid, width, height));
    println!("{:?}", part2(&grid, width, height));
}
