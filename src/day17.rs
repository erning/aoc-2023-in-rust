use std::collections::BinaryHeap;
use std::collections::HashSet;

fn find_minimal_path(grid: &[Vec<i32>]) -> i32 {
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;

    let start = (0, 0);
    let end = (w - 1, h - 1);

    let mut visited = HashSet::<(i32, i32, i32, i32)>::new();
    let mut queue = BinaryHeap::<(i32, (i32, i32), (i32, i32))>::new();

    queue.push((0, (start), (-1, 0)));

    while let Some((heat_loss, (x, y), (direction, straight))) = queue.pop() {
        if (x, y) == end {
            return -heat_loss;
        }

        const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        for (nd, &(dx, dy)) in DIRS.iter().enumerate() {
            let new_direction = nd as i32;
            let (direction, straight) = if new_direction == direction {
                // same direction
                if straight >= 3 {
                    continue;
                }
                (new_direction, straight + 1)
            } else if new_direction % 2 == direction % 2 {
                // reverse direction
                continue;
            } else {
                // turning
                (new_direction, 1)
            };
            let (x, y) = (x + dx, y + dy);
            if x < 0 || x >= w || y < 0 || y >= h {
                // out of bound
                continue;
            }
            let heat_loss = heat_loss - grid[y as usize][x as usize];
            if visited.contains(&(x, y, direction, straight)) {
                continue;
            }
            visited.insert((x, y, direction, straight));
            queue.push((heat_loss, (x, y), (direction, straight)));
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
    find_minimal_path(&grid)
}

pub fn part_two(input: &str) -> usize {
    let grid = parse_input(input);
    0
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
