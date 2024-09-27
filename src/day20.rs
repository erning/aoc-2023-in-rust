use std::collections::{HashMap, VecDeque};
use std::convert::From;

type Pulse = bool;
const PULSE_LOW: Pulse = false;
const PULSE_HIGH: Pulse = true;

#[derive(Debug)]
enum Module<'a> {
    Broadcaster {
        name: &'a str,
        outputs: Vec<&'a str>,
    },
    FlipFlop {
        name: &'a str,
        outputs: Vec<&'a str>,
        is_on: bool,
    },
    Conjunction {
        name: &'a str,
        outputs: Vec<&'a str>,
        memory: HashMap<&'a str, Pulse>,
    },
}

impl<'a> Module<'a> {
    fn name(&self) -> &'a str {
        match self {
            Module::Broadcaster { name, outputs: _ } => name,
            Module::FlipFlop {
                name,
                outputs: _,
                is_on: _,
            } => name,
            Module::Conjunction {
                name,
                outputs: _,
                memory: _,
            } => name,
        }
    }
    fn outputs(&self) -> &[&'a str] {
        match self {
            Module::Broadcaster { name: _, outputs } => outputs,
            Module::FlipFlop {
                name: _,
                outputs,
                is_on: _,
            } => outputs,
            Module::Conjunction {
                name: _,
                outputs,
                memory: _,
            } => outputs,
        }
    }
}

impl<'a> From<&'a str> for Module<'a> {
    fn from(value: &'a str) -> Module<'a> {
        if let Some((a, b)) = value.split_once("->") {
            let outputs: Vec<&'a str> =
                b.split(',').map(|s| s.trim()).collect();
            match &a[0..1] {
                "%" => Module::FlipFlop {
                    name: a[1..].trim(),
                    outputs,
                    is_on: false,
                },
                "&" => Module::Conjunction {
                    name: a[1..].trim(),
                    outputs,
                    memory: HashMap::new(),
                },
                _ => Module::Broadcaster {
                    name: "broadcaster",
                    outputs,
                },
            }
        } else {
            panic!("unsupported input")
        }
    }
}

fn parse_input(input: &str) -> HashMap<&str, Module> {
    let mut modules: HashMap<&str, Module> = input
        .lines()
        .map(Module::from)
        .map(|m| (m.name(), m))
        .collect();
    modules
        .values()
        .flat_map(|a| {
            a.outputs()
                .iter()
                .filter_map(|&b| modules.get(b))
                .map(|b| (a.name(), b.name()))
        })
        .collect::<Vec<(&str, &str)>>()
        .iter()
        .for_each(|&(a, b)| {
            if let Some(Module::Conjunction {
                name: _,
                outputs: _,
                memory,
            }) = modules.get_mut(b)
            {
                memory.insert(a, PULSE_LOW);
            }
        });
    modules
}

pub fn part_one(input: &str) -> i32 {
    let mut modules = parse_input(input);

    let times = 1000;
    let mut low = 0;
    let mut high = 0;

    for _ in 0..times {
        let mut queue: VecDeque<(&str, Pulse, &str)> = VecDeque::new();
        queue.push_back(("button", PULSE_LOW, "broadcaster"));
        while let Some((from, pulse, name)) = queue.pop_front() {
            match pulse {
                PULSE_HIGH => high += 1,
                PULSE_LOW => low += 1,
            }
            match modules.get_mut(name) {
                Some(Module::Broadcaster { name, outputs }) => {
                    outputs.iter().for_each(|&output| {
                        queue.push_back((name, pulse, output));
                    })
                }
                Some(Module::FlipFlop {
                    name,
                    outputs,
                    is_on,
                }) => {
                    match pulse {
                        PULSE_HIGH => {}
                        PULSE_LOW => {
                            *is_on = !*is_on;
                            let p =
                                if *is_on { PULSE_HIGH } else { PULSE_LOW };
                            outputs.iter().for_each(|&output| {
                                queue.push_back((name, p, output));
                            });
                        }
                    };
                }
                Some(Module::Conjunction {
                    name,
                    outputs,
                    memory,
                }) => {
                    match memory.get_mut(from) {
                        None => {
                            memory.insert(from, pulse);
                        }
                        Some(v) => *v = pulse,
                    }
                    let p = if memory.values().all(|&v| v == PULSE_HIGH) {
                        PULSE_LOW
                    } else {
                        PULSE_HIGH
                    };
                    outputs.iter().for_each(|&output| {
                        queue.push_back((name, p, output));
                    })
                }
                _ => {}
            }
        }
    }
    low * high
}

pub fn part_two(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn modules_from_str() {
        let cases = [
            "broadcaster -> a",
            "%a -> inv, con",
            "&inv -> b",
            "%b -> con",
            "&con -> output",
        ];
        cases.iter().for_each(|&s| {
            let m = Module::from(s);
            println!("{:?}", m);
        });
    }

    #[test]
    fn test_parse_input() {
        let input = "broadcaster -> a, b, c\n\
            %a -> b\n\
            %b -> c\n\
            %c -> inv\n\
            &inv -> a";
        let modules = parse_input(input);
        println!("{:?}", modules);
    }

    #[test]
    fn example() {
        let input = read_example(20);
        assert_eq!(part_one(&input), 32000000);
        assert_eq!(part_two(&input), 0);
    }

    #[test]
    fn example2() {
        let input = "broadcaster -> a\n\
            %a -> inv, con\n\
            &inv -> b\n\
            %b -> con\n\
            &con -> output";
        assert_eq!(part_one(input), 11687500);
    }
}
