use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some((x, y)),
                _ => None,
            })
        })
        .collect()
}

fn expaned_distance(galaxies: &Vec<(usize, usize)>, expand: i64) -> i64 {
    let (w, h) = galaxies
        .iter()
        .fold((0, 0), |(w, h), &(x, y)| (w.max(x + 1), h.max(y + 1)));

    let galaxy_cols: HashSet<usize> =
        galaxies.iter().map(|(x, _)| *x).collect();
    let galaxy_rows: HashSet<usize> =
        galaxies.iter().map(|(_, y)| *y).collect();

    let mut v = 0;
    let expanded_rows: Vec<i64> = (0..h)
        .map(|y| {
            v += if galaxy_rows.contains(&y) { 1 } else { expand };
            v
        })
        .collect();
    let mut v = 0;
    let expanded_cols: Vec<i64> = (0..w)
        .map(|x| {
            v += if galaxy_cols.contains(&x) { 1 } else { expand };
            v
        })
        .collect();

    let mut distance = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i..galaxies.len() {
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];
            // manhattan distance
            distance += (expanded_cols[x1] - expanded_cols[x2]).abs();
            distance += (expanded_rows[y1] - expanded_rows[y2]).abs();
        }
    }
    distance
}

pub fn part_one(input: &str) -> i64 {
    let galaxies = parse_input(input);
    expaned_distance(&galaxies, 2)
}

pub fn part_two(input: &str) -> i64 {
    let galaxies = parse_input(input);
    expaned_distance(&galaxies, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(11);
        let galaxies = parse_input(&input);
        assert_eq!(expaned_distance(&galaxies, 2), 374);
        assert_eq!(expaned_distance(&galaxies, 10), 1030);
        assert_eq!(expaned_distance(&galaxies, 100), 8410);
    }
}
