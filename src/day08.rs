use std::collections::HashMap;
use std::collections::HashSet;

fn parse_input(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    let network: HashMap<&str, (&str, &str)> = lines
        .skip(1)
        .map(|s| (&s[0..3], (&s[7..10], &s[12..15])))
        .collect();
    (instructions, network)
}

pub fn part_one(input: &str) -> usize {
    let (instructions, network) = parse_input(input);
    let instructions = instructions.as_bytes();
    let n = instructions.len();

    let mut step = 0;

    let mut node = "AAA";
    while node != "ZZZ" {
        let next = network.get(node).unwrap();
        node = match instructions[step % n] {
            b'L' => next.0,
            b'R' => next.1,
            _ => panic!(),
        };
        step += 1;
    }

    step
}

pub fn part_two(input: &str) -> usize {
    let (instructions, network) = parse_input(input);
    let instructions = instructions.as_bytes();
    let n = instructions.len();

    let mut nodes: Vec<&str> = network
        .keys()
        .filter(|x| x.ends_with('A'))
        .copied()
        .collect();

    0

    // let mut step = 0;
    // let mut done = 0;
    // while done < nodes.len() {
    //     let instruction = instructions[step % n];
    //     done = 0;
    //     for node in nodes.iter_mut() {
    //         let next = network.get(node).unwrap();
    //         *node = match instruction {
    //             b'L' => next.0,
    //             b'R' => next.1,
    //             _ => panic!(),
    //         };
    //         if node.ends_with('Z') {
    //             done += 1;
    //             println!("{:?} - {:?} {:?}", step, node, done);
    //         }
    //     }
    //     step += 1;
    // }
    // step
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(8);
        assert_eq!(part_one(&input), 6);

        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(part_two(&input), 6);
    }
}
