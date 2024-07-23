fn parse_input(input: &str) -> Vec<(usize, i32)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.split_whitespace().collect();
            (
                match v[0] {
                    "R" => 0,
                    "D" => 1,
                    "L" => 2,
                    "U" => 3,
                    _ => panic!(),
                },
                v[1].parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn parse_input_hex(input: &str) -> Vec<(usize, i32)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.split_whitespace().collect();
            (
                usize::from_str_radix(&v[2][7..8], 16).unwrap(),
                i32::from_str_radix(&v[2][2..7], 16).unwrap(),
            )
        })
        .collect()
}

// The last hexadecimal digit encodes the direction to dig:
// 0 means R, 1 means D, 2 means L, and 3 means U.
const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn part_one(input: &str) -> usize {
    let plan = parse_input(input);

    // build grid
    let (mut x0, mut y0) = (0, 0);
    let (mut x1, mut y1) = (0, 0);
    let (mut x, mut y) = (0, 0);
    let mut trench: Vec<(i32, i32)> = vec![(x, y)];
    for &(d, s) in plan.iter() {
        let (dx, dy) = DIRS[d];
        for _ in 0..s {
            x += dx;
            y += dy;
            trench.push((x, y));
        }
        (x0, y0) = (x0.min(x), y0.min(y));
        (x1, y1) = (x1.max(x), y1.max(y));
    }
    let w = x1 - x0 + 1;
    let h = y1 - y0 + 1;
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; w as usize]; h as usize];
    for item in trench.iter_mut() {
        let (x, y) = ((item.0 - x0) as usize, (item.1 - y0) as usize);
        grid[y][x] = '#';
    }

    let mut flood = |x: i32, y: i32| {
        let mut queue: Vec<(i32, i32)> = vec![(x, y)];

        if grid[y as usize][x as usize] != '.' {
            return;
        }
        grid[y as usize][x as usize] = '+';

        while let Some((x, y)) = queue.pop() {
            for (dx, dy) in DIRS {
                let (x, y) = (x + dx, y + dy);
                if x < 0 || x >= w || y < 0 || y >= h {
                    continue;
                }
                if grid[y as usize][x as usize] != '.' {
                    continue;
                }
                grid[y as usize][x as usize] = '+';
                queue.push((x, y));
            }
        }
    };

    // flood from edges
    for x in 0..w {
        flood(x, 0);
        flood(x, h - 1);
    }
    for y in 0..h {
        flood(0, y);
        flood(w - 1, y);
    }
    // grid.iter().for_each(|row| {
    //     row.iter().for_each(|ch| print!("{}", ch));
    //     println!();
    // });
    grid.iter()
        .map(|row| row.iter().filter(|&&ch| ch != '+').count())
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let plan = parse_input_hex(input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(18);
        assert_eq!(part_one(&input), 62);
        assert_eq!(part_two(&input), 0);
    }
}
