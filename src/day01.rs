pub fn part_one(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|s| {
            let a = s
                .chars()
                .find(|ch| ch.is_numeric())
                .unwrap()
                .to_digit(10)
                .unwrap();
            let b = s
                .chars()
                .rfind(|ch| ch.is_numeric())
                .unwrap()
                .to_digit(10)
                .unwrap();
            a * 10 + b
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    fn numeric(s: &str, i: usize) -> Option<u32> {
        let words = [
            "one", "two", "three", "four", "five", "six", "seven", "eight",
            "nine",
        ];
        let ch = s.chars().nth(i).unwrap();
        return if ch.is_numeric() {
            ch.to_digit(10)
        } else {
            words
                .into_iter()
                .enumerate()
                .find(|(_, word)| s[i..].starts_with(word))
                .map(|(j, _)| j as u32 + 1)
        };
    }

    input
        .trim()
        .lines()
        .map(|s| {
            let n = s.len();
            let a = (0..n).find_map(|i| numeric(s, i)).unwrap();
            let b = (0..n).rev().find_map(|i| numeric(s, i)).unwrap();
            a * 10 + b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(1);
        assert_eq!(part_one(&input), 142);
        assert_eq!(part_two(&input), 142);
    }
}
