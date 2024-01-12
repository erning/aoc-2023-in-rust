use std::collections::HashMap;
use std::collections::HashSet;

type Pos = (i32, i32);
type Pipes = HashMap<Pos, [Pos; 2]>;

// returns (piles as hashmap, starting position)
fn parse_input(input: &str) -> (Pipes, Pos) {
    let mut start = (0, 0);
    let mut pipes = Pipes::new();
    input.trim().lines().enumerate().for_each(|(y, s)| {
        let y = y as i32;
        s.chars().enumerate().for_each(|(x, c)| {
            let x = x as i32;
            match c {
                '.' => {}
                'S' => start = (x, y),
                _ => {
                    pipes.insert(
                        (x, y),
                        match c {
                            '|' => [(x, y - 1), (x, y + 1)],
                            '-' => [(x - 1, y), (x + 1, y)],
                            'L' => [(x, y - 1), (x + 1, y)],
                            'J' => [(x, y - 1), (x - 1, y)],
                            '7' => [(x - 1, y), (x, y + 1)],
                            'F' => [(x + 1, y), (x, y + 1)],
                            _ => panic!(),
                        },
                    );
                }
            }
        })
    });

    pipes.insert(
        start,
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .iter()
            .map(|(dx, dy)| (start.0 + dx, start.1 + dy))
            .filter(|pos| match pipes.get(pos) {
                // Some(nexts) => nexts.iter().any(|next| next == &start),
                Some(nexts) => nexts.contains(&start),
                None => false,
            })
            .collect::<Vec<Pos>>()
            .try_into()
            .unwrap(),
    );

    (pipes, start)
}

pub fn part_one(input: &str) -> i64 {
    let (pipes, start) = parse_input(input);

    let mut step = 0;
    let mut prev = (-1, -1);
    let mut curr = start;

    while let Some(nexts) = pipes.get(&curr) {
        step += 1;
        let next = nexts.iter().find(|&pos| pos != &prev).unwrap().to_owned();
        if next == start {
            break;
        }
        prev = curr;
        curr = next;
    }

    step / 2
}

pub fn part_two(input: &str) -> usize {
    let (pipes, start) = parse_input(input);

    let mut main_loop: HashSet<Pos> = HashSet::new();
    let mut prev = (-1, -1);
    let mut curr = start;

    while let Some(nexts) = pipes.get(&curr) {
        main_loop.insert(curr);
        let next = nexts.iter().find(|&pos| pos != &prev).unwrap().to_owned();
        if next == start {
            break;
        }
        prev = curr;
        curr = next;
    }

    let h = input.trim().lines().count() as i32;
    let w = input.trim().lines().next().unwrap().len() as i32;

    // Ray casting algorithm
    let mut tiles = 0;
    for y in 0..h {
        let mut i = 0; // intersection
        let mut j = 0; // south and/or north
        for x in 0..w {
            if main_loop.contains(&(x, y)) {
                let nexts = pipes.get(&(x, y)).unwrap();
                let pipe: u8 = [(1, 0), (0, 1), (-1, 0), (0, -1)]
                    .iter()
                    .enumerate()
                    .map(|(i, (dx, dy))| (i, (x + dx, y + dy)))
                    .filter(|(_, pos)| nexts.contains(pos))
                    .map(|(i, _)| 1 << i)
                    .reduce(|a, i| a | i)
                    .unwrap();
                if pipe & 0b0100 == 0 {
                    i += 1;
                    j = pipe & 0b1010;
                } else if pipe & j > 0 {
                    i += 1;
                }
            } else if i % 2 > 0 {
                tiles += 1;
            }
        }
    }
    tiles
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(10);
        assert_eq!(part_one(&input), 8);
        assert_eq!(part_two(&input), 1);
    }

    #[test]
    fn example2() {
        let input = "
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
";
        assert_eq!(part_two(&input), 4);
    }

    #[test]
    fn example3() {
        let input = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
        assert_eq!(part_two(&input), 8);
    }

    #[test]
    fn example4() {
        let input = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
        assert_eq!(part_two(&input), 10);
    }
}
