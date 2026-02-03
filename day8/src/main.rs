type Connection = (i64, JBox, JBox);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct JBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JBox {
    fn new(txt: &str) -> JBox {
        let points = txt
            .splitn(3, ",")
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let (x, y, z) = (points[0], points[1], points[2]);
        JBox { x, y, z }
    }

    fn distance(&self, other: &JBox) -> i64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        i64::isqrt(x * x + y * y + z * z)
    }

    fn get_distances(&self, others: &[JBox]) -> Vec<(i64, JBox)> {
        others
            .iter()
            .map(|&pos| (self.distance(&pos), pos))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Circuit {
    jboxes: Vec<JBox>,
}

impl Circuit {
    fn new(connection: &Connection) -> Self {
        let &(_, jbox1, jbox2) = connection;
        Circuit {
            jboxes: vec![jbox1, jbox2],
        }
    }
    fn add(&mut self, jbox: &JBox) {
        self.jboxes.push(*jbox);
    }
    fn contains(self, jbox: &JBox) -> bool {
        self.jboxes.contains(jbox)
    }
    fn merge(&mut self, other: &Self) {
        self.jboxes.extend(&other.jboxes);
    }
}

fn find_connections(positions: &[JBox]) -> impl Iterator<Item = Connection> {
    let mut pos = positions.to_vec();
    let mut min_distances = vec![];
    while pos.len() > 1 {
        let position = pos.pop().unwrap();
        position
            .get_distances(&pos)
            .into_iter()
            .for_each(|(min_distance, min_position)| {
                min_distances.push((min_distance, position, min_position));
            })
    }
    min_distances.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    min_distances.into_iter()
}

fn search_circuits(circuits: &mut Vec<Circuit>, connection: &Connection) {
    let (_, jbox1, jbox2) = connection;
    let jbox1_position = circuits.iter().position(|c| c.clone().contains(jbox1));
    let jbox2_position = circuits.iter().position(|c| c.clone().contains(jbox2));
    match (jbox1_position, jbox2_position) {
        (None, None) => circuits.push(Circuit::new(connection)),
        // One match found, add the other
        (Some(pos1), None) => circuits[pos1].add(jbox2),
        (None, Some(pos2)) => circuits[pos2].add(jbox1),
        (Some(pos1), Some(pos2)) => {
            // Both matches found, merge circuits if different
            if pos1 != pos2 {
                // merge circuits
                let circuit2 = circuits[pos2].clone();
                circuits[pos1].merge(&circuit2);
                circuits.remove(pos2);
            }
            // Both matches found, do nothing
        }
    }
}

fn part1(positions: &[JBox], num_pairs: usize) -> usize {
    let mut circuits: Vec<Circuit> = vec![];
    let mut connections = find_connections(positions)
        .take(num_pairs)
        .collect::<Vec<_>>();
    while let Some(connection) = connections.pop() {
        search_circuits(&mut circuits, &connection);
    }
    let mut lengths = circuits
        .iter()
        .map(|c| c.jboxes.len())
        .collect::<Vec<usize>>();
    lengths.sort_unstable_by(|a, b| b.cmp(a));
    lengths.iter().take(3).product()
}

fn max_length(circuits: &[Circuit]) -> Option<usize> {
    circuits.iter().map(|c| c.jboxes.len()).max()
}

fn part2(positions: &[JBox]) -> i64 {
    let mut circuits: Vec<Circuit> = vec![];
    let mut connections = find_connections(positions);

    loop {
        let connection = connections.next().unwrap();
        search_circuits(&mut circuits, &connection);
        if max_length(&circuits).unwrap() == positions.len() {
            let (_, jbox1, jbox2) = connection;
            return jbox1.x * jbox2.x;
        }
    }
}

fn main() {
    let data = include_str!("../puzzle.txt");
    let num_pairs = 1000;
    let positions = data.lines().map(JBox::new).collect::<Vec<_>>();

    println!("{}", part1(&positions, num_pairs));
    println!("{}", part2(&positions));
}
