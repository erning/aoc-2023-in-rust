use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

struct Garden {
    start: (i32, i32),
    rocks: HashSet<(i32, i32)>,
    w: i32,
    h: i32,
}

fn parse_input(input: &str) -> Garden {
    let mut start: (i32, i32) = (0, 0);
    let mut rocks: HashSet<(i32, i32)> = HashSet::new();
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    input
        .lines()
        .enumerate()
        .map(|(x, s)| (x as i32, s))
        .for_each(|(x, s)| {
            s.chars().enumerate().map(|(y, c)| (y as i32, c)).for_each(
                |(y, c)| {
                    match c {
                        '#' => {
                            rocks.insert((x, y));
                        }
                        'S' => {
                            start = (x, y);
                        }
                        _ => {}
                    };
                    h = h.max(y)
                },
            );
            w = w.max(x)
        });
    assert!(w == h);
    w += 1;
    h += 1;
    Garden { start, rocks, w, h }
}

fn count_plots(garden: &Garden, steps: usize) -> Vec<(i32, i32)> {
    let mut plots: Vec<(i32, i32)> = Vec::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: VecDeque<(usize, (i32, i32))> = VecDeque::new();
    queue.push_back((steps, garden.start));
    while let Some((step, (x, y))) = queue.pop_front() {
        if !visited.insert((x, y)) {
            continue;
        }
        if step % 2 == 0 {
            plots.push((x, y));
        }
        if step == 0 {
            continue;
        }
        for (x, y) in [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
            let cx = ((x % garden.w) + (garden.w)) % garden.w;
            let cy = ((y % garden.h) + (garden.h)) % garden.h;
            if garden.rocks.contains(&(cx, cy)) {
                continue;
            }
            queue.push_back((step - 1, (x, y)));
        }
    }

    plots
}

fn _part_one(garden: &Garden, steps: usize) -> usize {
    count_plots(garden, steps).len()
}

fn _part_two(garden: &Garden, steps: usize) -> usize {
    let w = garden.w as usize;
    let d = steps / w;
    let m = steps % w;
    let v = 5 + d as i32 % 2;
    let s = steps.min(w * (v as usize - 1) + m);
    let plots = count_plots(garden, s);
    if steps <= s {
        return plots.len();
    }
    println!("==== {} -> s={}, d={}, m={}", steps, s, d, m);

    let mut boxes: HashMap<(i32, i32), usize> = HashMap::new();
    plots
        .iter()
        .map(|(x, y)| {
            (
                if x < &0 {
                    (x + 1) / garden.w - 1
                } else {
                    x / garden.w
                },
                if y < &0 {
                    (y + 1) / garden.h - 1
                } else {
                    y / garden.h
                },
            )
        })
        .for_each(|k| {
            if let Some(count) = boxes.get_mut(&k) {
                *count += 1;
            } else {
                boxes.insert(k, 1);
            }
        });

    for j in -v..=v {
        for i in -v..=v {
            print!("{:5}, ", boxes.get(&(i, j)).unwrap_or(&0));
        }
        println!();
    }

    let c1 = boxes.get(&(0, 0)).unwrap_or(&0);
    let c2 = boxes.get(&(0, 1)).unwrap_or(&0);
    println!("c={}, {}", c1, c2);

    let n1 = boxes.get(&(0, -v)).unwrap_or(&0);
    let n2 = boxes.get(&(0, -v + 1)).unwrap_or(&0);
    let n3 = boxes.get(&(0, -v + 2)).unwrap_or(&0);
    println!("n={}, {}, {}", n1, n2, n3);

    let s1 = boxes.get(&(0, v)).unwrap_or(&0);
    let s2 = boxes.get(&(0, v - 1)).unwrap_or(&0);
    let s3 = boxes.get(&(0, v - 2)).unwrap_or(&0);
    println!("s={}, {}, {}", s1, s2, s3);

    let e1 = boxes.get(&(v, 0)).unwrap_or(&0);
    let e2 = boxes.get(&(v - 1, 0)).unwrap_or(&0);
    let e3 = boxes.get(&(v - 2, 0)).unwrap_or(&0);
    println!("e={}, {}, {}", e1, e2, e3);

    let w1 = boxes.get(&(-v, 0)).unwrap_or(&0);
    let w2 = boxes.get(&(-v + 1, 0)).unwrap_or(&0);
    let w3 = boxes.get(&(-v + 2, 0)).unwrap_or(&0);
    println!("w={}, {}, {}", w1, w2, w3);

    let ne1 = boxes.get(&(1, -v + 1)).unwrap_or(&0);
    let ne2 = boxes.get(&(1, -v + 2)).unwrap_or(&0);
    let ne3 = boxes.get(&(1, -v + 3)).unwrap_or(&0);
    println!("ne={}, {}, {}", ne1, ne2, ne3);

    let se1 = boxes.get(&(1, v - 1)).unwrap_or(&0);
    let se2 = boxes.get(&(1, v - 2)).unwrap_or(&0);
    let se3 = boxes.get(&(1, v - 3)).unwrap_or(&0);
    println!("se={}, {}, {}", se1, se2, se3);

    let nw1 = boxes.get(&(-1, -v + 1)).unwrap_or(&0);
    let nw2 = boxes.get(&(-1, -v + 2)).unwrap_or(&0);
    let nw3 = boxes.get(&(-1, -v + 3)).unwrap_or(&0);
    println!("nw={}, {}, {}", nw1, nw2, nw3);

    let sw1 = boxes.get(&(-1, v - 1)).unwrap_or(&0);
    let sw2 = boxes.get(&(-1, v - 2)).unwrap_or(&0);
    let sw3 = boxes.get(&(-1, v - 3)).unwrap_or(&0);
    println!("sw={}, {}, {}", sw1, sw2, sw3);

    let mut answer = 0;
    answer += n1 + n2 + n3;
    answer += e1 + e2 + e3;
    answer += s1 + s2 + s3;
    answer += w1 + w2 + w3;

    answer += ne1 * d + ne2 * (d - 1) + ne3 * (d - 2);
    answer += se1 * d + se2 * (d - 1) + se3 * (d - 2);
    answer += sw1 * d + sw2 * (d - 1) + sw3 * (d - 2);
    answer += nw1 * d + nw2 * (d - 1) + nw3 * (d - 2);

    answer += {
        let n = (d - 2) / 2;
        // let t = (1 + n) * n / 2 * 8 + 1;
        let t = 4 * n * (n + 1) + 1;
        // println!("{:?}", (n, t));
        t * c1
    };

    answer += {
        let n = (d - 1) / 2;
        // let t = (4 + (2 * n - 1) * 4) * n / 2;
        let t = 4 * n * n;
        // println!("{:?}", (n, t));
        t * c2
    };

    answer
}

pub fn part_one(input: &str) -> usize {
    let garden = parse_input(input);
    _part_one(&garden, 64)
}

pub fn part_two(input: &str) -> usize {
    let garden = parse_input(input);
    _part_two(&garden, 26501365)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(21);
        let garden = parse_input(&input);
        assert_eq!(_part_one(&garden, 6), 16);
        assert_eq!(_part_two(&garden, 6), 16);
        assert_eq!(_part_two(&garden, 10), 50);
        assert_eq!(_part_two(&garden, 50), 1594);
        assert_eq!(_part_two(&garden, 100), 6536);
        assert_eq!(_part_two(&garden, 500), 167004);
        assert_eq!(_part_two(&garden, 1000), 668697);
        assert_eq!(_part_two(&garden, 5000), 16733044);
    }
}
