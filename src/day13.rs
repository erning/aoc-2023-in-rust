use std::convert::From;

#[derive(Debug)]
struct Pattern(Vec<Vec<char>>);

impl Pattern {
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn at(&self, x: usize, y: usize) -> char {
        self.0[y][x]
    }

    fn row_diff(&self, y1: usize, y2: usize) -> usize {
        (0..self.width())
            .map(|x| self.at(x, y1) == self.at(x, y2))
            .filter(|&v| !v)
            .count()
    }

    fn column_diff(&self, x1: usize, x2: usize) -> usize {
        (0..self.height())
            .map(|y| self.at(x1, y) == self.at(x2, y))
            .filter(|&v| !v)
            .count()
    }

    fn is_row_reflact_at(&self, y: usize, d: usize) -> bool {
        let mut diff = 0;
        let mut a = y;
        let mut b = y + 1;
        loop {
            diff += self.row_diff(a, b);
            if diff > d {
                return false;
            }
            if a == 0 || b >= self.height() - 1 {
                return diff == d;
            }
            a -= 1;
            b += 1;
        }
    }

    fn is_column_reflact_at(&self, x: usize, d: usize) -> bool {
        let mut diff = 0;
        let mut a = x;
        let mut b = x + 1;
        loop {
            diff += self.column_diff(a, b);
            if diff > d {
                return false;
            }
            if a == 0 || b >= self.width() - 1 {
                return diff == d;
            }
            a -= 1;
            b += 1;
        }
    }

    fn row_reflact_line(&self, d: usize) -> usize {
        for y in 0..&self.height() - 1 {
            if self.is_row_reflact_at(y, d) {
                return y + 1;
            }
        }
        0
    }

    fn column_reflact_line(&self, d: usize) -> usize {
        for x in 0..&self.width() - 1 {
            if self.is_column_reflact_at(x, d) {
                return x + 1;
            }
        }
        0
    }
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        Self(value.lines().map(|row| row.chars().collect()).collect())
    }
}

fn parse_input(input: &str) -> Vec<Pattern> {
    input.split("\n\n").map(Pattern::from).collect()
}

pub fn part_one(input: &str) -> usize {
    let patterns = parse_input(input);
    patterns
        .iter()
        .map(|pattern| {
            pattern.row_reflact_line(0) * 100 + pattern.column_reflact_line(0)
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let patterns = parse_input(input);
    patterns
        .iter()
        .map(|pattern| {
            pattern.row_reflact_line(1) * 100 + pattern.column_reflact_line(1)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(13);
        assert_eq!(part_one(&input), 405);
        assert_eq!(part_two(&input), 400);
    }
}
