use std::collections::HashMap;
use std::collections::HashSet;

type Pos = (i16, i16);
type Tile = u8;
type Trails = HashMap<Pos, Tile>;

fn parse_input(input: &str) -> (Trails, i16, i16) {
    let trails: Trails = input
        .lines()
        .enumerate()
        .flat_map(|(y, s)| {
            s.bytes()
                .enumerate()
                .filter(|(_, tile)| tile != &b'#')
                .map(|(x, tile)| ((x as i16, y as i16), tile))
                .collect::<Vec<(Pos, Tile)>>()
        })
        .collect();
    let height = input.lines().count() as i16;
    let start = input.find('.').unwrap() as i16;

    (trails, height, start)
}

fn dfs(
    trails: &Trails,
    visited: &mut HashSet<Pos>,
    max_steps: &mut u16,
    height: i16,
    x: i16,
    y: i16,
    steps: u16,
    is_part_two: bool,
) {
    if y == height - 1 {
        *max_steps = steps.max(*max_steps);
        // println!("{:?}", (steps, max_steps));
        return;
    }
    const DIRS: [Pos; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let dirs = if is_part_two {
        &DIRS
    } else {
        match trails.get(&(x, y)) {
            Some(b'.') => &DIRS,
            Some(b'>') => &DIRS[0..1],
            Some(b'v') => &DIRS[1..2],
            Some(b'<') => &DIRS[2..3],
            Some(b'^') => &DIRS[3..4],
            _ => {
                panic!();
            }
        }
    };
    for (dx, dy) in dirs {
        let (x, y) = (x + dx, y + dy);
        if trails.contains_key(&(x, y)) && visited.insert((x, y)) {
            dfs(
                trails,
                visited,
                max_steps,
                height,
                x,
                y,
                steps + 1,
                is_part_two,
            );
            visited.remove(&(x, y));
        }
    }
}

pub fn part_one(input: &str) -> u16 {
    let (trails, height, start) = parse_input(input);

    let mut visited: HashSet<Pos> = HashSet::new();
    let mut max_steps = 0;
    dfs(
        &trails,
        &mut visited,
        &mut max_steps,
        height,
        start,
        0,
        0,
        false,
    );
    max_steps
}

pub fn part_two(input: &str) -> u16 {
    let (trails, height, start) = parse_input(input);

    let mut visited: HashSet<Pos> = HashSet::new();
    let mut max_steps = 0;
    dfs(
        &trails,
        &mut visited,
        &mut max_steps,
        height,
        start,
        0,
        0,
        true,
    );
    max_steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(23);
        assert_eq!(part_one(&input), 94);
        assert_eq!(part_two(&input), 154);
    }
}
