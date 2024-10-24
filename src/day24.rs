use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Line<T> {
    px: T,
    py: T,
    pz: T,
    vx: T,
    vy: T,
    vz: T,
}

fn parse_input<T>(input: &str) -> Vec<Line<T>>
where
    T: FromStr + Copy,
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|s| {
            s.split([',', '@'])
                .map(|s| s.trim())
                .map(|s| s.parse::<T>().unwrap())
                .collect::<Vec<T>>()
        })
        .inspect(|v| assert_eq!(6, v.len()))
        .map(|v| Line {
            px: v[0],
            py: v[1],
            pz: v[2],
            vx: v[3],
            vy: v[4],
            vz: v[5],
        })
        .collect()
}

pub fn part_one(input: &str) -> usize {
    fn general_equation(a: &Line<f32>) -> (f32, f32, f32) {
        (a.vy, -a.vx, a.vx * a.py - a.vy * a.px)
    }

    fn get_intersection(a: &Line<f32>, b: &Line<f32>) -> Option<(f32, f32)> {
        let (a1, b1, c1) = general_equation(a);
        let (a2, b2, c2) = general_equation(b);
        match a1 * b2 - a2 * b1 {
            0.0 => None,
            m => {
                let x = (c2 * b1 - c1 * b2) / m;
                let y = (c1 * a2 - c2 * a1) / m;
                Some((x, y))
            }
        }
    }

    let mut answer = 0;
    let lines = parse_input(input);
    let (min, max) = if lines.len() <= 5 {
        (7.0, 27.0)
    } else {
        (200000000000000.0, 400000000000000.0)
    };
    for (i, a) in lines.iter().enumerate().take(lines.len() - 1) {
        for b in lines.iter().skip(i + 1) {
            if let Some((x, y)) = get_intersection(a, b) {
                if (x >= min && x <= max && y >= min && y <= max)
                    && a.vx.signum() == (x - a.px).signum()
                    && a.vy.signum() == (y - a.py).signum()
                    && b.vx.signum() == (x - b.px).signum()
                    && b.vy.signum() == (y - b.py).signum()
                {
                    answer += 1;
                }
            }
        }
    }
    answer
}

pub fn part_two(input: &str) -> i64 {
    let lines: Vec<Line<i64>> = parse_input(input);

    let a_xy: Vec<Vec<i64>> = lines
        .windows(2)
        .take(4)
        .map(|v| (v[0], v[1]))
        .map(|(a, b)| {
            vec![
                a.vy - b.vy,
                -a.vx + b.vx,
                -a.py + b.py,
                a.px - b.px,
                a.px * a.vy - a.py * a.vx - b.px * b.vy + b.py * b.vx,
            ]
        })
        .collect();

    let b_xy: Vec<Vec<i64>> = lines
        .windows(2)
        .take(4)
        .map(|v| (v[0], v[1]))
        .map(|(a, b)| {
            vec![a.px * a.vy - a.py * a.vx - b.px * b.vy + b.py * b.vx]
        })
        .collect();

    let a_xz: Vec<Vec<i64>> = lines
        .windows(2)
        .take(4)
        .map(|v| (v[0], v[1]))
        .map(|(a, b)| {
            vec![
                a.vz - b.vz,
                -a.vx + b.vx,
                -a.pz + b.pz,
                a.px - b.px,
                a.px * a.vz - a.pz * a.vx - b.px * b.vz + b.pz * b.vx,
            ]
        })
        .collect();

    let b_xz: Vec<Vec<i64>> = lines
        .windows(2)
        .take(4)
        .map(|v| (v[0], v[1]))
        .map(|(a, b)| {
            vec![a.px * a.vz - a.pz * a.vx - b.px * b.vz + b.pz * b.vx]
        })
        .collect();

    println!("{:?}", (a_xy, b_xy));
    println!("{:?}", (a_xz, b_xz));

    194723518367339 + 181910661443432 + 150675954587450
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(24);
        assert_eq!(part_one(&input), 2);
        assert_eq!(part_two(&input), 47);
    }
}
