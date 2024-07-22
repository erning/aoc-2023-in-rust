use std::collections::BinaryHeap;
use std::collections::HashSet;

fn find_minimal_path(grid: &[Vec<i32>], steps_range: (i32, i32)) -> i32 {
    const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;
    let start = (0, 0);
    let end = (w - 1, h - 1);

    // (x, y, direction, moving steps of the direction)
    let mut visited = HashSet::<(i32, i32, i32, i32)>::new();
    // (amount of heat loss, (x, y), (direction, steps))
    let mut queue = BinaryHeap::<(i32, (i32, i32), (i32, i32))>::new();

    queue.push((0, (start), (-1, 0)));

    while let Some((amount, (x, y), (direction, steps))) = queue.pop() {
        if steps >= steps_range.0 && (x, y) == end {
            return -amount;
        }

        for (nd, &(dx, dy)) in DIRS.iter().enumerate() {
            let next_direction = nd as i32;
            let next_steps = if direction < 0 {
                // starting
                1
            } else if next_direction == direction {
                // same direction
                if steps >= steps_range.1 {
                    continue;
                }
                steps + 1
            } else if next_direction % 2 == direction % 2 {
                // reverse direction
                continue;
            } else {
                // turning
                if steps < steps_range.0 {
                    continue;
                }
                1
            };
            let (x, y) = (x + dx, y + dy);
            if x < 0 || x >= w || y < 0 || y >= h {
                // out of bound
                continue;
            }
            let next_amount = amount - grid[y as usize][x as usize];
            if !visited.insert((x, y, next_direction, next_steps)) {
                continue;
            }
            queue.push((next_amount, (x, y), (next_direction, next_steps)));
        }
    }
    0
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    let grid = parse_input(input);
    find_minimal_path(&grid, (1, 3))
}

pub fn part_two(input: &str) -> i32 {
    let grid = parse_input(input);
    find_minimal_path(&grid, (4, 10))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn examnple2() {
        let input = "111111111111\n\
            999999999991\n\
            999999999991\n\
            999999999991\n\
            999999999991";
        assert_eq!(part_two(&input), 71);
    }

    #[test]
    fn example() {
        let input = read_example(17);
        assert_eq!(part_one(&input), 102);
        assert_eq!(part_two(&input), 94);
    }
}
