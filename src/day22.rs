#[derive(Debug, Clone)]
struct Brick {
    i: usize,
    x1: i32,
    y1: i32,
    z1: i32,
    x2: i32,
    y2: i32,
    z2: i32,
    supporting: Vec<usize>,
    supported: Vec<usize>,
}

impl Brick {
    fn new(
        i: usize,
        x1: i32,
        y1: i32,
        z1: i32,
        x2: i32,
        y2: i32,
        z2: i32,
    ) -> Self {
        assert!(x1 <= x2 && y1 <= y2 && z1 <= z2);
        assert!(
            (x1 == x2 && (y1 == y2 || z1 == z2)) || (y1 == y2 && z1 == z2)
        );
        Brick {
            i,
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
            supporting: Vec::new(),
            supported: Vec::new(),
        }
    }

    fn is_encountered(&self, other: &Self) -> bool {
        fn is_encountered_1d(a1: i32, a2: i32, b1: i32, b2: i32) -> bool {
            assert!(a1 <= a2 && b1 <= b2);
            if a2 < b1 {
                // a1-----a2.....b1-----b2
                false
            } else if b2 < a1 {
                // b1-----b2.....a1-----a2
                false
            } else {
                true
            }
        }

        is_encountered_1d(self.x1, self.x2, other.x1, other.x2)
            && is_encountered_1d(self.y1, self.y2, other.y1, other.y2)
            && is_encountered_1d(self.z1, self.z2, other.z1, other.z2)
    }

    fn fall_to(&mut self, z: i32) {
        let d = self.z2 - self.z1;
        self.z1 = z;
        self.z2 = z + d;
    }

    fn up(&mut self) {
        self.z1 += 1;
        self.z2 += 1;
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .enumerate()
        .map(|(i, s)| {
            let parts: Vec<Vec<i32>> = s
                .splitn(2, '~')
                .map(|s| {
                    s.splitn(3, ',').map(|v| v.parse().unwrap()).collect()
                })
                .collect();
            Brick::new(
                i + 1,
                parts[0][0],
                parts[0][1],
                parts[0][2],
                parts[1][0],
                parts[1][1],
                parts[1][2],
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> usize {
    let mut bricks = parse_input(input);
    bricks.sort_unstable_by(|a, b| b.z1.cmp(&a.z1));
    let mut settled: Vec<Brick> = Vec::new();
    while let Some(mut brick) = bricks.pop() {
        let mut z: Option<i32> = None;
        for other in settled.iter_mut() {
            match z {
                Some(z) => {
                    if z != other.z2 {
                        break;
                    }
                }
                None => {
                    brick.fall_to(other.z2);
                }
            }
            if brick.is_encountered(other) {
                z = Some(brick.z1);
                brick.supported.push(other.i);
                other.supporting.push(brick.i);
            }
        }
        if z.is_none() {
            brick.fall_to(1);
        } else {
            brick.up();
        }
        settled.push(brick);
        settled.sort_unstable_by(|a, b| b.z2.cmp(&a.z2));
    }
    for b in settled.iter() {
        println!("{:?}", b);
    }
    let mut x = settled.len();
    for b in settled.iter() {
        let c = settled
            .iter()
            .filter(|v| v.supported.len() == 1 && v.supported.contains(&b.i))
            .count();
        if c > 0 {
            x -= 1;
        }
        println!("{:?}", (c, b.i));
    }

    x
}

pub fn part_two(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn test_encountered() {
        let b1 = Brick::new(1, 1, 0, 0, 1, 2, 0);
        let b2 = Brick::new(2, 0, 0, 2, 2, 0, 2);
        let b3 = Brick::new(3, 0, 0, 0, 2, 0, 0);
        assert_eq!(false, b1.is_encountered(&b2));
        assert_eq!(true, b1.is_encountered(&b3));
    }

    #[test]
    fn example() {
        let input = read_example(22);
        assert_eq!(part_one(&input), 5);
        assert_eq!(part_two(&input), 0);
    }
}
