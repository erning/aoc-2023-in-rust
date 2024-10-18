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

pub fn part_one(input: &str) -> u16 {
    let (trails, height, start) = parse_input(input);
    println!("{:?}", (height, start));

    fn dfs(
        trails: &Trails,
        visited: &mut HashSet<Pos>,
        height: i16,
        x: i16,
        y: i16,
        steps: u16,
    ) -> u16 {
        if y == height - 1 {
            println!("steps={}", steps);
            return steps;
        }
        let mut max_steps = 0;
        let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let range = match trails.get(&(x, y)) {
            Some(b'.') => 0..4,
            Some(b'>') => 0..1,
            Some(b'v') => 1..2,
            Some(b'<') => 2..3,
            Some(b'^') => 3..4,
            _ => {
                panic!();
            }
        };
        for (dx, dy) in &dirs[range] {
            let (x, y) = (x + dx, y + dy);
            if visited.contains(&(x, y)) {
                continue;
            }
            if trails.get(&(x, y)).is_some() {
                visited.insert((x, y));
                max_steps = dfs(trails, visited, height, x, y, steps + 1)
                    .max(max_steps);
                visited.remove(&(x, y));
            }
        }
        max_steps
    }
    let mut visited: HashSet<Pos> = HashSet::new();
    dfs(&trails, &mut visited, height, start, 0, 0)
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
        let input = read_example(23);
        assert_eq!(part_one(&input), 94);
        assert_eq!(part_one(&input), 154);
    }
}
