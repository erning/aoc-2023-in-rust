fn parse_input(input: &str) -> Vec<Vec<Vec<u32>>> {
    input
        .trim()
        .lines()
        .map(|s| {
            s.split_once(':')
                .unwrap()
                .1
                .trim()
                .split('|')
                .map(|s| {
                    s.split_whitespace().map(|s| s.parse().unwrap()).collect()
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    let cards = parse_input(input);
    cards
        .iter()
        .map(|card| {
            card[1]
                .iter()
                .filter_map(|a| card[0].iter().find(|&b| a == b))
                .fold(1, |a, _| a << 1)
                >> 1
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let cards = parse_input(input);
    let points = cards
        .iter()
        .map(|card| {
            card[1]
                .iter()
                .filter_map(|a| card[0].iter().find(|&b| a == b))
                .count()
        })
        .collect::<Vec<usize>>();

    let mut numbers: Vec<usize> = vec![1; cards.len()];
    let n = numbers.len();
    for (i, p) in points.iter().enumerate() {
        let t = numbers[i];
        let a = (i + 1).min(n);
        let b = (a + p).min(n);
        for v in numbers.iter_mut().take(b).skip(a) {
            *v += t;
        }
        // for j in a..b {
        //     numbers[j] += t;
        // }
    }
    numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(4);
        assert_eq!(part_one(&input), 13);
        assert_eq!(part_two(&input), 30);
    }
}
