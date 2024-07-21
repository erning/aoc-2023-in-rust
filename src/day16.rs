use std::collections::HashSet;

// 0:E 1:S 2:W 3:N
// const DIRECTION: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

type Beam = (i32, i32, char);

struct Contraption {
    grid: Vec<Vec<char>>,
    beams: Vec<Beam>,
    energized: HashSet<(i32, i32)>,
    track: HashSet<Beam>,
}

impl Contraption {
    fn width(&self) -> i32 {
        self.grid[0].len() as i32
    }

    fn height(&self) -> i32 {
        self.grid.len() as i32
    }

    fn tile(&self, x: i32, y: i32) -> char {
        self.grid[y as usize][x as usize]
    }

    fn move_beam(&self, beam: &Beam) -> Vec<Beam> {
        let (x, y) = match *beam {
            (x, y, 'E') => (x + 1, y),
            (x, y, 'S') => (x, y + 1),
            (x, y, 'W') => (x - 1, y),
            (x, y, 'N') => (x, y - 1),
            _ => panic!(),
        };
        if x < 0 || x >= self.width() || y < 0 || y >= self.height() {
            return vec![];
        }
        let tile = self.tile(x, y);
        let mut beams: Vec<Beam> = match (tile, beam.2) {
            ('|', 'E' | 'W') => {
                vec!['S', 'N']
            }
            ('-', 'S' | 'N') => {
                vec!['E', 'W']
            }
            ('/', 'E') => {
                vec!['N']
            }
            ('/', 'S') => {
                vec!['W']
            }
            ('/', 'W') => {
                vec!['S']
            }
            ('/', 'N') => {
                vec!['E']
            }
            ('\\', 'E') => {
                vec!['S']
            }
            ('\\', 'S') => {
                vec!['E']
            }
            ('\\', 'W') => {
                vec!['N']
            }
            ('\\', 'N') => {
                vec!['W']
            }
            (_, d) => vec![d],
        }
        .into_iter()
        .map(|d| (x, y, d))
        .collect();

        beams.retain(|v| !self.track.contains(v));
        beams
    }

    fn tick(&mut self) -> usize {
        let mut new_beams: Vec<Beam> = vec![];
        for beam in self.beams.iter() {
            let mut beams = self.move_beam(beam);
            new_beams.append(&mut beams);
        }
        for beam in &new_beams {
            self.track.insert(*beam);
            self.energized.insert((beam.0, beam.1));
        }
        self.beams = new_beams;
        self.beams.len()
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.trim().lines().map(|s| s.chars().collect()).collect()
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let start = (-1, 0, 'E');
    let mut contraption = Contraption {
        grid,
        beams: vec![start],
        energized: HashSet::new(),
        track: HashSet::new(),
    };
    while contraption.tick() > 0 {}
    contraption.energized.len()
}
pub fn part_two(input: &str) -> usize {
    let grid = parse_input(input);
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;
    let mut starts: Vec<(i32, i32, char)> = vec![];
    for i in 0..w {
        starts.push((i, -1, 'S'));
        starts.push((i, h, 'N'));
    }
    for i in 0..h {
        starts.push((-1, i, 'E'));
        starts.push((w, i, 'W'));
    }
    starts
        .into_iter()
        .map(|start| {
            let mut contraption = Contraption {
                grid: grid.clone(),
                beams: vec![start],
                energized: HashSet::new(),
                track: HashSet::new(),
            };
            while contraption.tick() > 0 {}
            contraption.energized.len()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(16);
        assert_eq!(part_one(&input), 46);
        assert_eq!(part_two(&input), 51);
    }
}
