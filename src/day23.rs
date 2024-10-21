use std::collections::HashMap;
use std::collections::HashSet;

type Pos = (i16, i16);
type Trails = HashMap<Pos, u8>;
type Map = HashMap<Pos, Vec<(Pos, u16)>>;

fn parse_input(input: &str) -> (Trails, i16, i16) {
    let trails: Trails = input
        .lines()
        .enumerate()
        .flat_map(|(y, s)| {
            s.bytes()
                .enumerate()
                .filter(|(_, tile)| tile != &b'#')
                .map(|(x, tile)| ((x as i16, y as i16), tile))
                .collect::<Vec<(Pos, u8)>>()
        })
        .collect();
    let height = input.lines().count() as i16;
    let start = input.find('.').unwrap() as i16;

    (trails, height, start)
}

pub fn part_one(input: &str) -> u16 {
    fn dfs(
        trails: &Trails,
        visited: &mut HashSet<Pos>,
        max_steps: &mut u16,
        height: i16,
        x: i16,
        y: i16,
        steps: u16,
    ) {
        if y == height - 1 {
            *max_steps = steps.max(*max_steps);
            return;
        }
        const DIRS: [Pos; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let dirs = match trails.get(&(x, y)) {
            Some(b'.') => &DIRS,
            Some(b'>') => &DIRS[0..1],
            Some(b'v') => &DIRS[1..2],
            Some(b'<') => &DIRS[2..3],
            Some(b'^') => &DIRS[3..4],
            _ => {
                panic!();
            }
        };
        for (dx, dy) in dirs {
            let (x, y) = (x + dx, y + dy);
            if trails.contains_key(&(x, y)) && visited.insert((x, y)) {
                dfs(trails, visited, max_steps, height, x, y, steps + 1);
                visited.remove(&(x, y));
            }
        }
    }

    let (trails, height, start) = parse_input(input);
    let mut visited: HashSet<Pos> = HashSet::from([(start, 0)]);
    let mut max_steps = 0;
    dfs(&trails, &mut visited, &mut max_steps, height, start, 0, 0);
    max_steps
}

pub fn part_two(input: &str) -> u16 {
    fn compress_map(trails: &Trails) -> Map {
        let mut map: Map = HashMap::new();
        const DIRS: [Pos; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        let nodes: HashMap<Pos, Vec<(i16, i16)>> = trails
            .keys()
            .map(|(x, y)| {
                let nexts: Vec<(i16, i16)> = DIRS
                    .iter()
                    .map(|(dx, dy)| (x + dx, y + dy))
                    .filter(|k| trails.contains_key(k))
                    .collect();
                ((*x, *y), nexts)
            })
            .filter(|(_, dirs)| dirs.len() != 2)
            .collect();

        for (pos, nexts) in nodes.iter() {
            for next in nexts {
                let (mut x0, mut y0) = pos;
                let (mut x1, mut y1) = next;
                let mut steps = 1;
                loop {
                    steps += 1;
                    let (x, y) = DIRS
                        .iter()
                        .map(|(dx, dy)| (dx + x1, dy + y1))
                        .filter(|k| k != &(x0, y0))
                        .find(|k| trails.contains_key(k))
                        .unwrap();
                    if nodes.contains_key(&(x, y)) {
                        if let Some(v) = map.get_mut(pos) {
                            v.push(((x, y), steps));
                        } else {
                            map.insert(*pos, vec![((x, y), steps)]);
                        }
                        break;
                    }
                    (x0, y0) = (x1, y1);
                    (x1, y1) = (x, y);
                }
            }
        }
        map
    }

    fn dfs(
        map: &Map,
        visited: &mut HashSet<(i16, i16)>,
        max_steps: &mut u16,
        height: i16,
        x: i16,
        y: i16,
        steps: u16,
    ) {
        if y == height - 1 {
            *max_steps = steps.max(*max_steps);
            // println!("{:?}", (steps, max_steps));
            return;
        }
        for (p, s) in map.get(&(x, y)).unwrap() {
            if visited.insert(*p) {
                dfs(map, visited, max_steps, height, p.0, p.1, steps + s);
                visited.remove(p);
            }
        }
    }

    let (trails, height, start) = parse_input(input);
    let map = compress_map(&trails);
    let mut visited: HashSet<Pos> = HashSet::from([(start, 0)]);
    let mut max_steps = 0;
    dfs(&map, &mut visited, &mut max_steps, height, start, 0, 0);
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
