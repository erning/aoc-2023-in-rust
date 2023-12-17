fn parse_input(input: &str) -> Vec<Vec<[u32; 3]>> {
    input
        .trim()
        .lines()
        .map(|s| {
            let (_, s) = s.split_once(':').unwrap();
            s.trim()
                .split("; ")
                .map(|s| {
                    let mut cubes = [0, 0, 0];
                    s.split(", ").for_each(|s| {
                        let (a, b) = s.split_once(' ').unwrap();
                        let c = match b {
                            "red" => 0,
                            "green" => 1,
                            "blue" => 2,
                            _ => panic!(),
                        };
                        cubes[c] = a.parse().unwrap()
                    });
                    cubes
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    let configuration = [12, 13, 14];
    let games = parse_input(input);
    games
        .into_iter()
        .enumerate()
        .filter(|(_, game)| {
            let mut possible = true;
            'outer: for cubes in game {
                for c in 0..3 {
                    if cubes[c] > configuration[c] {
                        possible = false;
                        break 'outer;
                    }
                }
            }
            possible
        })
        .map(|(i, _)| i as u32 + 1)
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let games = parse_input(input);
    games
        .into_iter()
        .map(|game| {
            let mut fewest = [0, 0, 0];
            for cubes in game {
                for c in 0..3 {
                    if cubes[c] > fewest[c] {
                        fewest[c] = cubes[c];
                    }
                }
            }
            fewest[0] * fewest[1] * fewest[2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(2);
        assert_eq!(part_one(&input), 8);
        assert_eq!(part_two(&input), 2286);
    }
}
