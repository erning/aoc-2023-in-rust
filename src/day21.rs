use std::collections::HashSet;

struct Parsed {
    start: (i32, i32),
    rocks: HashSet<(i32, i32)>,
    w: i32,
    h: i32,
}

fn parse_input(input: &str) -> Parsed {
    let mut start: (i32, i32) = (0, 0);
    let mut rocks: HashSet<(i32, i32)> = HashSet::new();
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    input
        .lines()
        .enumerate()
        .map(|(x, s)| (x as i32, s))
        .for_each(|(x, s)| {
            s.chars().enumerate().map(|(y, c)| (y as i32, c)).for_each(
                |(y, c)| {
                    match c {
                        '#' => {
                            rocks.insert((x, y));
                        }
                        'S' => {
                            start = (x, y);
                        }
                        _ => {}
                    };
                    h = h.max(y)
                },
            );
            w = w.max(x)
        });
    Parsed { start, rocks, w, h }
}

fn garden_plots(parsed: &Parsed, steps: usize) -> usize {
    let mut plots: HashSet<(i32, i32)> = HashSet::new();
    plots.insert(parsed.start);
    for _ in 0..steps {
        let mut new_plots: HashSet<(i32, i32)> = HashSet::new();
        for (x, y) in plots.iter() {
            for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                let (x, y) = (x + dx, y + dy);
                if x < 0 || x >= parsed.w || y < 0 || y >= parsed.h {
                    continue;
                }
                if parsed.rocks.contains(&(x, y)) {
                    continue;
                }
                new_plots.insert((x, y));
            }
        }
        plots = new_plots;
    }
    plots.len()
}

pub fn part_one(input: &str) -> usize {
    let parsed = parse_input(input);
    garden_plots(&parsed, 64)
}

pub fn part_two(input: &str) -> usize {
    let parsed = parse_input(input);
    // garden_plots(&parsed, 26501365)
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(21);
        let parsed = parse_input(&input);
        assert_eq!(garden_plots(&parsed, 6), 16);
        assert_eq!(garden_plots(&parsed, 10), 50);
        assert_eq!(garden_plots(&parsed, 50), 1594);
        assert_eq!(garden_plots(&parsed, 100), 6536);
        assert_eq!(garden_plots(&parsed, 500), 167004);
        assert_eq!(garden_plots(&parsed, 1000), 668697);
        assert_eq!(garden_plots(&parsed, 5000), 16733044);
    }
}
