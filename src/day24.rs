#[derive(Debug, Clone, Copy)]
struct Line {
    px: f32,
    py: f32,
    pz: f32,
    vx: f32,
    vy: f32,
    vz: f32,
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|s| {
            s.split([',', '@'])
                .map(|s| s.trim())
                .map(|s| s.parse::<f32>().unwrap())
                .collect::<Vec<f32>>()
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
    fn general_equation(a: &Line) -> (f32, f32, f32) {
        (a.vy, -a.vx, a.vx * a.py - a.vy * a.px)
    }

    fn get_intersection(a: &Line, b: &Line) -> Option<(f32, f32)> {
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
                if x < min || x > max || y < min || y > max {
                    continue;
                }
                if (a.vx < 0.0 && x > a.px) || (a.vx > 0.0 && x < a.px) {
                    continue;
                }
                if (a.vy < 0.0 && y > a.py) || (a.vy > 0.0 && y < a.py) {
                    continue;
                }
                if (b.vx < 0.0 && x > b.px) || (b.vx > 0.0 && x < b.px) {
                    continue;
                }
                if (b.vy < 0.0 && y > b.py) || (b.vy > 0.0 && y < b.py) {
                    continue;
                }
                answer += 1;
            }
        }
    }
    answer
}

pub fn part_two(input: &str) -> u32 {
    let lines = parse_input(input);
    0
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
