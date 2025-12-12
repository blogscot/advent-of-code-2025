fn part1(ingredients_iter: impl Iterator<Item = u64>, ranges: &[(u64, u64)]) -> usize {
    ingredients_iter
        .map(|ingredient| {
            ranges
                .iter()
                .any(|&(start, end)| (start..=end).contains(&ingredient))
        })
        .filter(|&x| x)
        .count()
}

fn streamline(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    ranges
        .iter()
        .skip(1)
        .fold(vec![ranges[0]], |mut acc, current| {
            let previous: (u64, u64) = acc.pop().unwrap();
            let (start1, end1) = previous;
            let &(start2, end2) = current;
            // if previous is a subset of current
            if start1 == start2 && end1 <= end2 {
                let result = (start2, end2);
                acc.push(result);
                acc
            }
            // if previous is a superset of current
            else if start1 <= start2 && end1 >= end2 {
                let result = (start1, end1);
                acc.push(result);
                acc
            }
            // if previous overlaps with current
            else if end1 >= start2 {
                let result = (start1, end2);
                acc.push(result);
                acc
            } else {
                [acc, vec![previous, *current]].concat()
            }
        })
}

fn part2(ranges: &mut [(u64, u64)]) -> u64 {
    ranges.sort_by(|a, b| match a.0.cmp(&b.0) {
        std::cmp::Ordering::Equal => a.1.cmp(&b.1),
        x => x,
    });

    streamline(ranges)
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum::<u64>()
}

fn main() {
    let data = include_str!("../puzzle.txt");
    let (ranges, ingredients) = data.split_once("\n\n").expect("Error opening file");
    let mut ranges = ranges
        .split("\n")
        .filter_map(|x| x.split_once("-"))
        .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
        .collect::<Vec<_>>();

    let ingredients_iter = ingredients.lines().map(|x| x.parse::<u64>().unwrap());

    println!("{:?}", part1(ingredients_iter, &ranges));
    println!("{:?}", part2(&mut ranges));
}
