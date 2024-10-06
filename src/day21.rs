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
    input.lines().enumerate().for_each(|(x, s)| {
        s.chars().enumerate().for_each(|(y, c)| {
            match c {
                '#' => {
                    rocks.insert((x as i32, y as i32));
                }
                'S' => {
                    start = (x as i32, y as i32);
                }
                _ => {}
            };
            h = h.max(y as i32)
        });
        w = w.max(x as i32)
    });
    Parsed { start, rocks, w, h }
}

fn garden_plots(parsed: Parsed, steps: usize) -> usize {
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
    garden_plots(parsed, 64)
}

pub fn part_two(input: &str) -> u32 {
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
        let count = garden_plots(parsed, 6);
        assert_eq!(count, 16);

        // In exactly 6 steps, he can still reach 16 garden plots.
        // In exactly 10 steps, he can reach any of 50 garden plots.
        // In exactly 50 steps, he can reach 1594 garden plots.
        // In exactly 100 steps, he can reach 6536 garden plots.
        // In exactly 500 steps, he can reach 167004 garden plots.
        // In exactly 1000 steps, he can reach 668697 garden plots.
        // In exactly 5000 steps, he can reach 16733044 garden plots.
    }
}
