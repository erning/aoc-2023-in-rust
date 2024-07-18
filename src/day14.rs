fn tilt(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let h = grid[0].len();
    let w = grid.len();
    let mut tilted: Vec<Vec<char>> = grid.to_owned();
    for y in 1..h {
        for x in 0..w {
            if tilted[y][x] == 'O' {
                for i in (0..y).rev() {
                    if tilted[i][x] != '.' {
                        break;
                    }
                    tilted[i + 1][x] = '.';
                    tilted[i][x] = 'O';
                }
            }
        }
    }
    tilted
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|row| row.chars().collect()).collect()
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let tilted = tilt(&grid);
    let h = tilted.len();
    tilted
        .iter()
        .map(|row| row.iter().filter(|&&ch| ch == 'O').count())
        .enumerate()
        .map(|(i, v)| ((h - i), v))
        .map(|(i, v)| i * v)
        .sum()
}

pub fn part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(14);
        assert_eq!(part_one(&input), 136);
        assert_eq!(part_two(&input), 0);
    }
}
