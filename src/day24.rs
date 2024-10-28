use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Ray<T> {
    px: T,
    py: T,
    pz: T,
    vx: T,
    vy: T,
    vz: T,
}

fn parse_input<T>(input: &str) -> Vec<Ray<T>>
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
        .map(|v| Ray {
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
    fn general_equation(a: &Ray<f32>) -> (f32, f32, f32) {
        (a.vy, -a.vx, a.vx * a.py - a.vy * a.px)
    }

    fn get_intersection(a: &Ray<f32>, b: &Ray<f32>) -> Option<(f32, f32)> {
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
    let rays = parse_input(input);
    let (min, max) = if rays.len() <= 5 {
        (7.0, 27.0)
    } else {
        (200000000000000.0, 400000000000000.0)
    };
    for (i, a) in rays.iter().enumerate().take(rays.len() - 1) {
        for b in rays.iter().skip(i + 1) {
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

fn gaussian_elimination(matrix: &[Vec<f64>]) -> Vec<f64> {
    let mut m = matrix.to_vec();
    let h = m.len();
    let w = m[0].len();
    assert!(h == w - 1);

    for i in 0..h - 1 {
        let mut max_row = i;
        for j in i + 1..h {
            if m[j][i].abs() > m[max_row][i].abs() {
                max_row = j;
            }
        }
        m.swap(i, max_row);
        let a = m[i][i];
        assert!(a != 0.0);
        for j in i + 1..h {
            let b = m[j][i];
            if b == 0.0 {
                continue;
            }
            let c = a / b;
            for k in 0..w {
                m[j][k] = m[i][k] - m[j][k] * c;
            }
        }
    }

    let mut answer: Vec<f64> = vec![0.0; h];
    for i in (0..h).rev() {
        let mut v = m[i][w - 1];
        for (j, ans) in answer.iter().enumerate().skip(i + 1) {
            v -= m[i][j] * ans;
        }
        v /= m[i][i];
        answer[i] = v.round();
    }
    answer
}

pub fn part_two(input: &str) -> i64 {
    let rays: Vec<Ray<f64>> = parse_input(input);

    let xy: Vec<Vec<f64>> = rays
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

    let xz: Vec<Vec<f64>> = rays
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

    let yz: Vec<Vec<f64>> = rays
        .windows(2)
        .take(4)
        .map(|v| (v[0], v[1]))
        .map(|(a, b)| {
            vec![
                a.vz - b.vz,
                -a.vy + b.vy,
                -a.pz + b.pz,
                a.py - b.py,
                a.py * a.vz - a.pz * a.vy - b.py * b.vz + b.pz * b.vy,
            ]
        })
        .collect();

    let a1 = gaussian_elimination(&xy);
    let a2 = gaussian_elimination(&xz);
    let a3 = gaussian_elimination(&yz);

    assert!(a1[0] == a2[0]);
    assert!(a1[1] == a3[0]);
    assert!(a2[1] == a3[1]);

    a1[0] as i64 + a1[1] as i64 + a2[1] as i64
    // 194723518367339 + 181910661443432 + 150675954587450
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
