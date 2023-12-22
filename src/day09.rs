fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .trim()
        .lines()
        .map(|s| s.split_whitespace().map(|v| v.parse().unwrap()).collect())
        .collect()
}

pub fn part_one(input: &str) -> i64 {
    fn predict(s: &[i64]) -> i64 {
        if s.iter().all(|&x| x == 0) {
            return 0;
        }
        let next: Vec<i64> = s.windows(2).map(|v| v[1] - v[0]).collect();
        s.last().unwrap() + predict(&next)
    }

    let input = parse_input(input);
    input.iter().map(|sequence| predict(sequence)).sum()
}

pub fn part_two(input: &str) -> i64 {
    fn predict(s: &[i64]) -> i64 {
        if s.iter().all(|&x| x == 0) {
            return 0;
        }
        let next: Vec<i64> = s.windows(2).map(|v| v[1] - v[0]).collect();
        s.first().unwrap() - predict(&next)
    }

    let input = parse_input(input);
    input.iter().map(|sequence| predict(sequence)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(9);
        assert_eq!(part_one(&input), 114);
        assert_eq!(part_two(&input), 2);
    }
}
