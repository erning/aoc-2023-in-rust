pub fn part_one(input: &str) -> usize {
    let numbers: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|s| {
            s.split_whitespace()
                .skip(1)
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect();
    let n = numbers[0].len();
    (0..n)
        .map(|i| (numbers[0][i], numbers[1][i]))
        .map(|(time, distance)| {
            (1..time).filter(|&t| (time - t) * t > distance).count()
        })
        .product()
}

pub fn part_two(input: &str) -> usize {
    let numbers: Vec<u64> = input
        .trim()
        .lines()
        .map(|line| line.split_once(':').unwrap().1.replace(' ', ""))
        .map(|v| v.parse().unwrap())
        .collect();

    let time = numbers[0];
    let distance = numbers[1];

    let mut delta = time;
    let mut t = time / 2;
    while delta > 0 {
        if (time - t) * t > distance {
            delta /= 2;
            t -= delta;
        } else {
            t += delta;
        }
    }
    (time + 1 - t - t) as usize

    /*
    let mut a = 0;
    loop {
        let b = distance / (time - a);
        if a >= b {
            break;
        }
        a = b
    }
    for t in a..time {
        if (time - t) * t > distance {
            return (time + 1 - t - t) as usize;
        }
    }
    0
    */
    // (a..time-a).filter(|&t| (time - t) * t > distance).count()
    // (1..time).filter(|&t| (time - t) * t > distance).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(6);
        assert_eq!(part_one(&input), 288);
        assert_eq!(part_two(&input), 71503);
    }
}
