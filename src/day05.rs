#[allow(clippy::type_complexity)]
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
    for line in lines {
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

fn overlap(x: &(i64, i64), y: &(i64, i64)) -> Option<(i64, i64)> {
    if x > y {
        return overlap(y, x);
    }

    // considering the ranges are: [x1:x2) and [y1:y2)
    let (x1, x2) = x;
    let (y1, y2) = y;

    // assume that the ranges are well-formed (so that x1 < x2 and y1 < y2)
    assert!(x1 < x2);
    assert!(y1 < y2);

    // there exists some number C which is in both ranges
    // x1 < C < x2
    // y1 < C < y2
    if y1 < x2 {
        Some((y1.to_owned(), x2.min(y2).to_owned()))
    } else {
        None
    }
}

fn transform(
    sources: &Vec<(i64, i64)>,
    rules: &Vec<((i64, i64), i64)>,
) -> Vec<(i64, i64)> {
    let mut rv = vec![];

    for source in sources {
        let mut queue: Vec<(i64, i64)> = vec![source.to_owned()];
        while let Some(x) = queue.pop() {
            let mut found = false;
            for (y, delta) in rules {
                if let Some((a, b)) = overlap(&x, y) {
                    if x.0 < a {
                        queue.push((x.0, a));
                    }
                    rv.push((a + delta, b + delta));
                    if x.1 > b {
                        queue.push((b, x.1));
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                rv.push(x);
            }
        }
    }

    rv
}

pub fn part_two(input: &str) -> i64 {
    let (seeds, maps) = parse_input(input);

    // (begin, length) => (begin, end)
    let seeds: Vec<(i64, i64)> =
        seeds.chunks(2).map(|v| (v[0], v[0] + v[1])).collect();

    maps.iter()
        .map(|m| {
            // (destination, source, length) => (source begin, source end, delta);
            m.iter().map(|v| ((v.1, v.1 + v.2), v.0 - v.1)).collect()
        })
        .fold(seeds, |sources, rules| transform(&sources, &rules))
        .into_iter()
        .min()
        .unwrap()
        .0

    // let mut r = seeds;
    // for m in maps {
    //     let rules: Vec<((i64, i64), i64)> =
    //         m.iter().map(|v| ((v.1, v.1 + v.2), v.0 - v.1)).collect();
    //     r = transform(&r, &rules);
    // }
    // r.iter().min().unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(5);
        assert_eq!(part_one(&input), 35);
        assert_eq!(part_two(&input), 46);
    }

    #[test]
    fn overlap_test() {
        assert_eq!(overlap(&(10, 20), &(10, 20)), Some((10, 20)));

        assert_eq!(overlap(&(10, 20), &(30, 40)), None);
        assert_eq!(overlap(&(30, 40), &(10, 20)), None);

        assert_eq!(overlap(&(10, 20), &(15, 30)), Some((15, 20)));
        assert_eq!(overlap(&(15, 30), &(10, 20)), Some((15, 20)));

        assert_eq!(overlap(&(10, 20), &(20, 30)), None);
        assert_eq!(overlap(&(20, 30), &(10, 20)), None);

        assert_eq!(overlap(&(10, 20), &(10, 30)), Some((10, 20)));
        assert_eq!(overlap(&(10, 30), &(10, 20)), Some((10, 20)));
    }
}
