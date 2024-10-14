#[derive(Debug, Clone, Copy, PartialEq)]
enum Oriention {
    X,
    Y,
    Z,
}

type Cube = (i32, i32, i32);

type Vector = (i32, i32, i32);

#[derive(Debug, Clone, Copy)]
struct Brick {
    a: Cube,
    b: Cube,
}

impl Brick {
    fn new(a: Cube, b: Cube) -> Self {
        assert!(
            (a.0 == b.0 && (a.1 == b.1 || a.2 == b.2))
                || (a.1 == b.1 && a.2 == b.2)
        );
        assert!(a.0 <= b.0 && a.1 <= b.1 && a.2 <= b.2);
        Brick { a, b }
    }

    fn oriention(&self) -> Oriention {
        match self.vector() {
            (0, 0, 0) => panic!(),
            (0, 0, _) => Oriention::Z,
            (0, _, 0) => Oriention::Y,
            (_, 0, 0) => Oriention::X,
            _ => panic!(),
        }
    }

    fn vector(&self) -> Vector {
        (
            self.b.0 - self.a.0,
            self.b.1 - self.a.1,
            self.b.2 - self.a.2,
        )
    }

    fn is_encountered(&self, other: &Brick) -> bool {
        false
    }

    fn is_parallel(&self, other: &Brick) -> bool {
        match (self.oriention(), other.oriention()) {
            (Oriention::X, Oriention::X) => {
                self.a.1 != other.a.1 || self.a.2 != other.a.2
            }
            (Oriention::Y, Oriention::Y) => {
                self.a.0 != other.a.0 || self.a.2 != other.a.2
            }
            (Oriention::Z, Oriention::Z) => {
                self.a.0 != other.a.0 || self.a.1 != other.a.1
            }
            _ => false,
        }
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|s| {
            let parts: Vec<Vec<i32>> = s
                .splitn(2, '~')
                .map(|s| {
                    s.splitn(3, ',').map(|v| v.parse().unwrap()).collect()
                })
                .collect();
            Brick::new(
                (parts[0][0], parts[0][1], parts[0][2]),
                (parts[1][0], parts[1][1], parts[1][2]),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    let bricks = parse_input(input);
    println!("{:?}", bricks);
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
        let input = read_example(22);
        assert_eq!(part_one(&input), 5);
        assert_eq!(part_two(&input), 0);
    }
}
