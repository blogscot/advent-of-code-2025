use std::{collections::HashSet, fmt::Debug};

fn combinations(vec: &[Tile]) -> Vec<(Tile, Tile)> {
    let mut output = vec![];
    for i in 0..(vec.len() - 1) {
        for j in (i + 1)..vec.len() {
            output.push((vec[i], vec[j]));
        }
    }
    output
}

fn area((a, b): (&Tile, &Tile)) -> u64 {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Tile(u64, u64);

impl Tile {
    fn new(x: u64, y: u64) -> Tile {
        Tile(x, y)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Axis {
    Vertical = 0,
    Horizontal = 1,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Edge {
    along: u64, // Vertical edge has shared x value, horizontal shared y
    start: u64,
    end: u64,
}

struct Part2 {
    tiles: Vec<Tile>,
    vertical_edges: HashSet<Edge>,
    horizontal_edges: HashSet<Edge>,
}

impl Part2 {
    fn new(tiles: &[Tile]) -> Self {
        Part2 {
            tiles: tiles.to_vec(),
            vertical_edges: find_edges(tiles, Axis::Vertical),
            horizontal_edges: find_edges(tiles, Axis::Horizontal),
        }
    }

    fn area_contains_no_edges(&self, t1: &Tile, t2: &Tile) -> bool {
        let (min_x, max_x) = (t1.0.min(t2.0), t1.0.max(t2.0));
        let (min_y, max_y) = (t1.1.min(t2.1), t1.1.max(t2.1));

        for edge in &self.vertical_edges {
            if min_x < edge.along && edge.along < max_x && edge.end > min_y && edge.start < max_y {
                return false;
            }
        }
        for edge in &self.horizontal_edges {
            if min_y < edge.along && edge.along < max_y && edge.end > min_x && edge.start < max_x {
                return false;
            }
        }
        true
    }

    fn solve(&self) -> u64 {
        combinations(&self.tiles)
            .iter()
            .filter(|(t1, t2)| self.area_contains_no_edges(t1, t2))
            .map(|(t1, t2)| area((t1, t2)))
            .max()
            .unwrap()
    }
}

/// Returns all the edges that are along the given axis
fn find_edges(tiles: &[Tile], axis: Axis) -> HashSet<Edge> {
    use Axis::*;
    combinations(tiles)
        .iter()
        .filter(|(t1, t2)| match axis {
            Vertical => t1.0 == t2.0,
            Horizontal => t1.1 == t2.1,
        })
        .map(|(t1, t2)| {
            let (along, start, end) = match axis {
                Vertical => (t1.0, t1.1.min(t2.1), t1.1.max(t2.1)),
                Horizontal => (t1.1, t1.0.min(t2.0), t1.0.max(t2.0)),
            };
            Edge { along, start, end }
        })
        .collect::<HashSet<_>>()
}

fn part1(tiles: &[Tile]) -> u64 {
    combinations(tiles)
        .iter()
        .map(|(t1, t2)| area((t1, t2)))
        .max()
        .unwrap()
}

fn main() {
    let data = std::fs::read_to_string("puzzle.txt").unwrap();
    let tiles = data
        .lines()
        .map(|line| line.split(",").collect::<Vec<&str>>())
        .map(|v| Tile::new(v[0].parse::<u64>().unwrap(), v[1].parse::<u64>().unwrap()))
        .collect::<Vec<_>>();

    println!("{:?}", part1(&tiles));
    let part2 = Part2::new(&tiles);
    println!("{:?}", part2.solve());
}
