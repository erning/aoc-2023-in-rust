fn parse_input(input: &str) -> (Vec<i64>, Vec<Vec<(i64, i64, i64)>>) {
    fn parse_seeds(line: &str) -> Vec<i64> {
        line.split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
    }
    let mut lines = input.lines();
    let seeds = parse_seeds(lines.next().unwrap());

    let mut maps: Vec<Vec<(i64, i64, i64)>> = vec![];
    let mut m: Vec<(i64, i64, i64)> = vec![];
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            continue;
        }
        if line.ends_with("map:") {
            if !m.is_empty() {
                maps.push(m);
                m = vec![];
            }
            continue;
        }
        let v: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        m.push((v[0], v[1], v[2]));
    }
    if !m.is_empty() {
        maps.push(m);
    }
    (seeds, maps)
}

pub fn part_one(input: &str) -> i64 {
    let (seeds, maps) = parse_input(input);
    seeds
        .iter()
        .map(|&seed| {
            maps.iter().fold(seed, |acc, m| {
                match m
                    .iter()
                    // convert from (destination, source, length) to
                    // (source begin, source end, delta to destination)
                    .map(|v| (v.1, v.1 + v.2, v.0 - v.1))
                    .find(|v| acc >= v.0 && acc <= v.1)
                {
                    Some(v) => acc + v.2,
                    None => acc,
                }
            })
        })
        .min()
        .unwrap()
}

pub fn part_two(input: &str) -> i64 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(5);
        assert_eq!(part_one(&input), 35);
        assert_eq!(part_two(&input), 0);
    }
}
