use std::collections::HashSet;
use std::collections::VecDeque;

struct Garden {
    start: (i32, i32),
    rocks: HashSet<(i32, i32)>,
    w: i32,
    h: i32,
}

fn parse_input(input: &str) -> Garden {
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
    w += 1;
    h += 1;
    Garden { start, rocks, w, h }
}

fn count_plots(garden: &Garden, steps: usize) -> usize {
    let mut plots: Vec<(i32, i32)> = Vec::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: VecDeque<(usize, (i32, i32))> = VecDeque::new();
    queue.push_back((steps, garden.start));
    while let Some((steps, (x, y))) = queue.pop_front() {
        if !visited.insert((x, y)) {
            continue;
        }
        if steps % 2 == 0 {
            plots.push((x, y));
        }
        if steps == 0 {
            continue;
        }
        for (x, y) in [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
            let cx = ((x % garden.w) + (garden.w)) % garden.w;
            let cy = ((y % garden.h) + (garden.h)) % garden.h;
            if garden.rocks.contains(&(cx, cy)) {
                continue;
            }
            queue.push_back((steps - 1, (x, y)));
        }
    }
    plots.len()
}

pub fn part_one(input: &str) -> usize {
    let garden = parse_input(input);
    count_plots(&garden, 64)
}

pub fn part_two(input: &str) -> usize {
    let garden = parse_input(input);
    count_plots(&garden, 2000)
    // count_plots(&garden, 26501365)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(21);
        let parsed = parse_input(&input);
        assert_eq!(count_plots(&parsed, 6), 16);
        assert_eq!(count_plots(&parsed, 10), 50);
        assert_eq!(count_plots(&parsed, 50), 1594);
        assert_eq!(count_plots(&parsed, 100), 6536);
        assert_eq!(count_plots(&parsed, 500), 167004);
        assert_eq!(count_plots(&parsed, 1000), 668697);
        assert_eq!(count_plots(&parsed, 5000), 16733044);
    }
}
