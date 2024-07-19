fn parse_input(input: &str) -> Vec<&str> {
    input.trim().split(',').collect()
}

fn hash(s: &str) -> usize {
    let mut value: usize = 0;
    for ascii in s.bytes() {
        value += ascii as usize;
        value *= 17;
        value %= 256;
    }
    value
}

pub fn part_one(input: &str) -> usize {
    let sequence = parse_input(input);
    sequence.iter().map(|v| hash(v)).sum()
}

pub fn part_two(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    let sequence = parse_input(input);
    sequence.iter().for_each(|code| {
        let lens: Vec<&str> = code.split(['=', '-']).collect();
        let label = lens[0];
        let slots = &mut boxes[hash(label)];
        if let Ok(focal_length) = lens[1].parse::<usize>() {
            if let Some(lens) = slots.iter_mut().find(|v| v.0 == label) {
                lens.1 = focal_length;
            } else {
                slots.push((label, focal_length))
            }
        } else {
            slots.retain(|v| v.0 != label);
        }
    });
    boxes
        .iter()
        .enumerate()
        .filter(|(_, v)| !v.is_empty())
        .map(|(box_number, lens)| {
            lens.iter()
                .map(|v| v.1)
                .enumerate()
                .map(|(slot_number, focal_length)| {
                    (box_number + 1) * (slot_number + 1) * focal_length
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn hash_value() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn example() {
        let input = read_example(15);
        assert_eq!(part_one(&input), 1320);
        assert_eq!(part_two(&input), 145);
    }
}
