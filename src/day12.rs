use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<(&str, Vec<usize>)> {
    input
        .trim()
        .lines()
        .map(|row| match row.split_once(' ') {
            Some((a, b)) => {
                (a, b.split(',').map(|v| v.parse().unwrap()).collect())
            }
            _ => panic!(),
        })
        .collect()
}

/*
  ? # # # ? ? ? ? ? ? ? ?
3
2
1
*/
fn count<'a>(
    mask: &'a [u8],
    nums: &'a [usize],
    cache: &mut HashMap<(&'a [u8], &'a [usize]), usize>,
) -> usize {
    if mask.is_empty() {
        return if nums.is_empty() { 1 } else { 0 };
    }

    if nums.is_empty() {
        return if mask.contains(&b'#') { 0 } else { 1 };
    }

    let key = (mask, nums);
    if let Some(value) = cache.get(&key) {
        return *value;
    }

    let mut result = 0;

    let c = mask[0]; // char
    let s = nums[0]; // size

    // '.' or '?'
    if c != b'#' {
        result += count(&mask[1..], nums, cache);
    }
    // '#' or '?'
    if c != b'.' {
        if s <= mask.len() && !mask[..s].contains(&b'.') {
            if s < mask.len() {
                if mask[s] != b'#' {
                    result += count(&mask[s + 1..], &nums[1..], cache)
                }
            } else {
                result += count(&mask[s..], &nums[1..], cache)
            }
        }
    }

    cache.insert(key, result);
    result
}

pub fn part_one(input: &str) -> usize {
    let mut cache: HashMap<(&[u8], &[usize]), usize> = HashMap::new();
    parse_input(input)
        .iter()
        .map(|(mask, nums)| (mask.as_bytes(), &nums[..]))
        .map(|(mask, nums)| count(mask, nums, &mut cache))
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let mut cache: HashMap<(&[u8], &[usize]), usize> = HashMap::new();
    parse_input(input)
        .iter()
        .map(|(mask, nums)| (mask.as_bytes(), &nums[..]))
        .map(|(mask, nums)| {
            (
                [mask, mask, mask, mask, mask].join(&[b'?'][..]),
                [nums, nums, nums, nums, nums].join(&[][..]),
            )
        })
        .collect::<Vec<_>>()
        .iter()
        // .inspect(|(mask, nums)| {
        //     println!(
        //         "{:?} {:?}",
        //         String::from_utf8(mask.to_vec()).unwrap(),
        //         nums
        //     )
        // })
        .map(|(mask, nums)| count(mask, nums, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(12);
        assert_eq!(part_one(&input), 21);
        assert_eq!(part_two(&input), 525152);
    }
}
