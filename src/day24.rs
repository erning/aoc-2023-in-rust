#[derive(Debug, Clone, Copy)]
struct Line {
    px: i32,
    py: i32,
    pz: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}

impl Line {}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|s| {
            s.split([',', '@'])
                .map(|s| s.trim())
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
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

pub fn part_one(input: &str) -> u32 {
    let x = parse_input(input);
    println!("{:?}", x);
    0
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
        let input = read_example(24);
        assert_eq!(part_one(&input), 0);
        assert_eq!(part_two(&input), 0);
    }
}
