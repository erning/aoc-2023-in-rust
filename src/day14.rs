use std::collections::HashMap;

fn tilt_north(grid: &mut [Vec<char>]) {
    let h = grid[0].len();
    let w = grid.len();
    for y in 1..h {
        for x in 0..w {
            if grid[y][x] == 'O' {
                for i in (0..y).rev() {
                    if grid[i][x] != '.' {
                        break;
                    }
                    grid[i + 1][x] = '.';
                    grid[i][x] = 'O';
                }
            }
        }
    }
}

fn tilt_south(grid: &mut [Vec<char>]) {
    let h = grid[0].len();
    let w = grid.len();
    for y in (0..h).rev() {
        for x in 0..w {
            if grid[y][x] == 'O' {
                for i in y + 1..h {
                    if grid[i][x] != '.' {
                        break;
                    }
                    grid[i - 1][x] = '.';
                    grid[i][x] = 'O';
                }
            }
        }
    }
}

fn tilt_west(grid: &mut [Vec<char>]) {
    let h = grid[0].len();
    let w = grid.len();
    for x in 1..w {
        #[allow(clippy::needless_range_loop)]
        for y in 0..h {
            if grid[y][x] == 'O' {
                for i in (0..x).rev() {
                    if grid[y][i] != '.' {
                        break;
                    }
                    grid[y][i + 1] = '.';
                    grid[y][i] = 'O';
                }
            }
        }
    }
}

fn tilt_east(grid: &mut [Vec<char>]) {
    let h = grid[0].len();
    let w = grid.len();
    for x in (0..w).rev() {
        #[allow(clippy::needless_range_loop)]
        for y in 0..h {
            if grid[y][x] == 'O' {
                for i in x + 1..w {
                    if grid[y][i] != '.' {
                        break;
                    }
                    grid[y][i - 1] = '.';
                    grid[y][i] = 'O';
                }
            }
        }
    }
}

fn spin(grid: &mut [Vec<char>]) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

fn total_load(grid: &[Vec<char>]) -> usize {
    let h = grid.len();
    grid.iter()
        .map(|row| row.iter().filter(|&&ch| ch == 'O').count())
        .enumerate()
        .map(|(i, v)| ((h - i), v))
        .map(|(i, v)| i * v)
        .sum()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|row| row.chars().collect()).collect()
}

pub fn part_one(input: &str) -> usize {
    let mut grid = parse_input(input);
    tilt_north(&mut grid);
    total_load(&grid)
}

pub fn part_two(input: &str) -> usize {
    const CYCLES: usize = 1000000000;

    let mut cache: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let mut grid = parse_input(input);

    let mut i = 0;
    let mut looped: Option<usize> = None;

    while i < CYCLES {
        i += 1;
        spin(&mut grid);
        if let Some(v) = cache.get(&grid) {
            looped = Some(*v);
            break;
        }
        cache.insert(grid.clone(), i);
    }

    if let Some(j) = looped {
        let w = (CYCLES - i) % (i - j);
        for _ in 0..w {
            spin(&mut grid);
        }
    }

    total_load(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(14);
        assert_eq!(part_one(&input), 136);
        assert_eq!(part_two(&input), 64);
    }
}
