use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    let v: Vec<(&str, Vec<&str>)> = input
        .lines()
        .map(|line| {
            let v: Vec<&str> =
                line.splitn(2, ':').map(|s| s.trim()).collect();
            (v[0], v[1].split_whitespace().collect())
        })
        .collect();
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut add_item = |a, b| {
        if let Some(item) = map.get_mut(a) {
            item.insert(b);
        } else {
            let item: HashSet<&str> = HashSet::new();
            map.insert(a, item);
        }
    };
    for (k, v) in v {
        for b in v {
            add_item(k, b);
            add_item(b, k);
        }
    }
    map
}

pub fn part_one(input: &str) -> u32 {
    let x = parse_input(input);
    for (k, v) in x.iter() {
        println!("{:?}", (k, v));
    }
    0
}

pub fn part_two(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(25);
        assert_eq!(part_one(&input), 0);
        assert_eq!(part_two(&input), 0);
    }
}
