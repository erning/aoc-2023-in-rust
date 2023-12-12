use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.trim().lines().map(|s| s.chars().collect()).collect()
}

// return tuple (number, x, y, width)
fn parse_numbers(grid: &Vec<Vec<char>>) -> Vec<(u32, usize, usize, usize)> {
    let h = grid.len();
    let w = grid[0].len();

    let mut numbers: Vec<(u32, usize, usize, usize)> = vec![];
    for y in 0..h {
        let mut n = 0;
        let mut s = 0;
        for x in 0..w {
            let ch = grid[y][x];
            if ch.is_numeric() {
                n = n * 10 + ch.to_digit(10).unwrap();
                s += 1;
            } else {
                if s != 0 {
                    numbers.push((n, x - s, y, s));
                    n = 0;
                    s = 0;
                }
            }
        }
        if s != 0 {
            numbers.push((n, w - s, y, s));
        }
    }
    numbers
}

pub fn part_one(input: &str) -> u32 {
    let grid = parse_input(input);
    let h = grid.len();
    let w = grid[0].len();
    let numbers = parse_numbers(&grid);

    let mut sum = 0;
    for &(n, x, y, s) in numbers.iter() {
        let is_symbol = |x: usize, y: usize| -> bool {
            grid[y][x] != '.' && !grid[y][x].is_numeric()
        };
        let x1 = x;
        let x2 = x + s;

        let is_adjacent_to_symbol = || {
            // north
            if y > 0 {
                for i in x1..x2 {
                    if is_symbol(i, y - 1) {
                        return true;
                    }
                }
            }
            // south
            if y < h - 1 {
                for i in x1..x2 {
                    if is_symbol(i, y + 1) {
                        return true;
                    }
                }
            }
            // west
            if x1 > 0 && is_symbol(x1 - 1, y) {
                return true;
            }
            // east
            if x2 < w && is_symbol(x2, y) {
                return true;
            }
            // NW
            if y > 0 && x1 > 0 && is_symbol(x1 - 1, y - 1) {
                return true;
            }
            // NE
            if y > 0 && x2 < w && is_symbol(x2, y - 1) {
                return true;
            }
            // SW
            if y < h - 1 && x1 > 0 && is_symbol(x1 - 1, y + 1) {
                return true;
            }
            // SE
            if y < h - 1 && x2 < w && is_symbol(x2, y + 1) {
                return true;
            }
            return false;
        };

        if is_adjacent_to_symbol() {
            sum += n
        }
    }
    sum
}

pub fn part_two(input: &str) -> u32 {
    let grid = parse_input(input);
    let h = grid.len();
    let w = grid[0].len();
    let numbers = parse_numbers(&grid);

    let mut stars: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for &(n, x, y, s) in numbers.iter() {
        let is_star = |x: usize, y: usize| -> bool { grid[y][x] == '*' };
        let mut insert_star = |x: usize, y: usize| {
            if let Some(v) = stars.get_mut(&(x, y)) {
                v.push(n);
            } else {
                stars.insert((x, y), vec![n]);
            }
        };
        let x1 = x;
        let x2 = x + s;
        // north
        if y > 0 {
            for i in x1..x2 {
                if is_star(i, y - 1) {
                    insert_star(i, y - 1);
                }
            }
        }
        // south
        if y < h - 1 {
            for i in x1..x2 {
                if is_star(i, y + 1) {
                    insert_star(i, y + 1);
                }
            }
        }
        // west
        if x1 > 0 && is_star(x1 - 1, y) {
            insert_star(x1 - 1, y);
        }
        // east
        if x2 < w && is_star(x2, y) {
            insert_star(x2, y);
        }
        // NW
        if y > 0 && x1 > 0 && is_star(x1 - 1, y - 1) {
            insert_star(x1 - 1, y - 1);
        }
        // NE
        if y > 0 && x2 < w && is_star(x2, y - 1) {
            insert_star(x2, y - 1);
        }
        // SW
        if y < h - 1 && x1 > 0 && is_star(x1 - 1, y + 1) {
            insert_star(x1 - 1, y + 1);
        }
        // SE
        if y < h - 1 && x2 < w && is_star(x2, y + 1) {
            insert_star(x2, y + 1);
        }
    }

    stars
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(3);
        assert_eq!(part_one(&input), 4361);
        assert_eq!(part_two(&input), 467835);
    }
}
