use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Point3D { x, y, z }
    }
}

#[derive(Debug, Clone)]
struct Brick {
    i: usize,
    a: Point3D,
    b: Point3D,
}

impl Brick {
    fn new(i: usize, a: Point3D, b: Point3D) -> Self {
        // assert!(x1 <= x2 && y1 <= y2 && z1 <= z2);
        // assert!(
        //     (x1 == x2 && (y1 == y2 || z1 == z2)) || (y1 == y2 && z1 == z2)
        // );
        Brick { i, a, b }
    }

    fn is_encountered(&self, other: &Self) -> bool {
        (self.b.x >= other.a.x && self.a.x <= other.b.x)
            && (self.b.y >= other.a.y && self.a.y <= other.b.y)
            && (self.b.z >= other.a.z && self.a.z <= other.b.z)
    }

    fn set_z(&mut self, z: i32) {
        let d = self.b.z - self.a.z;
        self.a.z = z;
        self.b.z = z + d;
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
            let a = Point3D::new(parts[0][0], parts[0][1], parts[0][2]);
            let b = Point3D::new(parts[1][0], parts[1][1], parts[1][2]);

            Brick::new(i, a, b)
        })
        .collect()
}

fn fall(bricks: &mut Vec<Brick>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let n = bricks.len();
    // let mut bricks = bricks.clone();
    bricks.sort_unstable_by_key(|k| -k.a.z);
    let mut supported: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut supporting: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut settled: Vec<Brick> = Vec::new();
    while let Some(mut brick) = bricks.pop() {
        let mut z = 0;
        for other in settled.iter() {
            if z == 0 {
                brick.set_z(other.b.z);
            } else if z != other.b.z {
                break;
            }
            if brick.is_encountered(other) {
                z = brick.a.z;
                supported[brick.i].push(other.i);
                supporting[other.i].push(brick.i);
            }
        }
        brick.set_z(z + 1);
        settled.push(brick);
        settled.sort_unstable_by_key(|k| -k.b.z);
    }
    bricks.append(&mut settled);
    (supported, supporting)
}

pub fn part_one(input: &str) -> usize {
    let mut bricks = parse_input(input);
    let (supported, _) = fall(&mut bricks);
    let set: HashSet<usize> = supported
        .iter()
        .filter(|v| v.len() == 1)
        .map(|v| v[0])
        .collect();
    bricks.len() - set.len()
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
        let b1 = Brick::new(1, Point3D::new(1, 0, 0), Point3D::new(1, 2, 0));
        let b2 = Brick::new(2, Point3D::new(0, 0, 2), Point3D::new(2, 0, 2));
        let b3 = Brick::new(3, Point3D::new(0, 0, 0), Point3D::new(2, 0, 0));
        assert!(!b1.is_encountered(&b2));
        assert!(b1.is_encountered(&b3));
    }

    #[test]
    fn example() {
        let input = read_example(22);
        assert_eq!(part_one(&input), 5);
        assert_eq!(part_two(&input), 0);
    }
}
