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
    #[cfg(debug_assertions)]
    println!("==== {} -> steps={}, div={}, mod={}", steps, s, d, m);

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

    #[cfg(debug_assertions)]
    for j in -v..=v {
        for i in -v..=v {
            print!("{:5}, ", boxes.get(&(i, j)).unwrap_or(&0));
        }
        println!();
    }

    let count = |i, j| boxes.get(&(i, j)).unwrap_or(&0);
    let mut answer = 0;
    {
        let c1 = count(0, 0);
        let n = (d - 2) / 2;
        let t = 4 * n * (n + 1) + 1;
        answer += t * c1;
    }
    {
        let c2 = count(0, 1);
        let n = (d - 1) / 2;
        let t = 4 * n * n;
        answer += t * c2;
    }

    for i in 0..3 {
        let n = count(0, -v + i);
        let s = count(0, v - i);
        let e = count(v - i, 0);
        let w = count(-v + i, 0);

        let ne = count(1, -v + 1 + i);
        let se = count(1, v - 1 - i);
        let nw = count(-1, -v + 1 + i);
        let sw = count(-1, v - 1 - i);
        #[cfg(debug_assertions)]
        println!(
            "{}: N={}, S={}, E={}, W={}, NE={}, SE={}, NW={}, SW={}",
            i, n, s, e, w, ne, se, nw, sw
        );

        answer += n + s + e + w;
        let c = d - i as usize;
        answer += ne * c;
        answer += se * c;
        answer += nw * c;
        answer += sw * c;
    }

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
