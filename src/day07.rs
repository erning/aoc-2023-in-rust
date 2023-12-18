use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<(&str, i64)> {
    input
        .trim()
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|v| (v.0, v.1.parse().unwrap()))
        .collect()
}

fn bytes2mask(bytes: &[u8]) -> Vec<u8> {
    let mut c = bytes.to_vec();
    c.sort_unstable();
    let mut mask: Vec<u8> = vec![1; c.len()];
    for i in 1..c.len() {
        if c[i] == c[i - 1] {
            mask[i] = mask[i - 1];
        } else {
            mask[i] = mask[i - 1] + 1;
        }
    }
    mask
}

fn mask2score(mask: &[u8]) -> i32 {
    match mask[..] {
        [1, 1, 1, 1, 1] => 6,
        [1, 1, 1, 1, 2] | [1, 2, 2, 2, 2] => 5,
        [1, 1, 1, 2, 2] | [1, 1, 2, 2, 2] => 4,
        [1, 1, 1, 2, 3] | [1, 2, 2, 2, 3] | [1, 2, 3, 3, 3] => 3,
        [1, 1, 2, 2, 3] | [1, 1, 2, 3, 3] | [1, 2, 2, 3, 3] => 2,
        [1, 1, 2, 3, 4]
        | [1, 2, 2, 3, 4]
        | [1, 2, 3, 3, 4]
        | [1, 2, 3, 4, 4] => 1,
        _ => 0,
    }
}

pub fn part_one(input: &str) -> i64 {
    let mut values: Vec<(i32, Vec<u8>, &str, i64)> = parse_input(input)
        .iter()
        .map(|&(hand, bid)| {
            let bytes: Vec<u8> = hand
                .as_bytes()
                .iter()
                .map(|v| match v {
                    b'A' => 14,
                    b'K' => 13,
                    b'Q' => 12,
                    b'J' => 11,
                    b'T' => 10,
                    _ => v - b'0',
                })
                .collect();

            let mask = bytes2mask(&bytes);
            let score = mask2score(&mask);
            (score, bytes, hand, bid)
        })
        .collect();
    values.sort_unstable();

    values
        .iter()
        .enumerate()
        .map(|(i, v)| (i as i64 + 1) * v.3)
        .sum()
}

pub fn part_two(input: &str) -> i64 {
    let mut values: Vec<(i32, Vec<u8>, &str, i64)> = parse_input(input)
        .iter()
        .map(|&(hand, bid)| {
            let bytes: Vec<u8> = hand
                .as_bytes()
                .iter()
                .map(|v| match v {
                    b'A' => 14,
                    b'K' => 13,
                    b'Q' => 12,
                    b'J' => 0,
                    b'T' => 10,
                    _ => v - b'0',
                })
                .collect();

            if !bytes.iter().any(|&b| b == 0) {
                let mask = bytes2mask(&bytes);
                let score = mask2score(&mask);
                (score, bytes, hand, bid)
            } else {
                let score: i32 = bytes
                    .iter()
                    .filter(|&&b| b != 0)
                    .copied()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .map(|v| {
                        let mut c = bytes.clone();
                        c.iter_mut()
                            .filter(|&&mut b| b == 0)
                            .for_each(|b| *b = v);
                        let mask = bytes2mask(&c);
                        mask2score(&mask)
                    })
                    .max()
                    .unwrap_or(6);
                (score, bytes, hand, bid)
            }
        })
        .collect();
    values.sort_unstable();

    values
        .iter()
        .enumerate()
        .map(|(i, v)| (i as i64 + 1) * v.3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(7);
        assert_eq!(part_one(&input), 6440);
        assert_eq!(part_two(&input), 5905);
    }
}
