use core::panic;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    mtype: ModuleType,
    outputs: Vec<&'a str>,
    inputs: Vec<&'a str>,
}

fn parse_input(input: &str) -> HashMap<&str, Module> {
    let mut modules: HashMap<&str, Module> = input
        .lines()
        .map(|line| {
            if let Some((lhs, rhs)) = line.split_once("->") {
                let outputs: Vec<&str> =
                    rhs.split(',').map(|s| s.trim()).collect();
                let mtype = match &lhs[..1] {
                    "%" => ModuleType::FlipFlop,
                    "&" => ModuleType::Conjunction,
                    _ => ModuleType::Broadcaster,
                };
                let name = match mtype {
                    ModuleType::Broadcaster => lhs.trim(),
                    _ => lhs[1..].trim(),
                };
                Module {
                    name,
                    mtype,
                    outputs,
                    inputs: vec![],
                }
            } else {
                panic!()
            }
        })
        .map(|module| (module.name, module))
        .collect();

    modules
        .values()
        .flat_map(|a| {
            a.outputs
                .iter()
                .filter_map(|&b| modules.get(b))
                .map(|b| (a.name, b.name))
        })
        .collect::<Vec<(&str, &str)>>()
        .iter()
        .for_each(|&(a, b)| {
            if let Some(module) = modules.get_mut(b) {
                module.inputs.push(a)
            }
        });

    modules
}

fn init_flags<'a>(
    modules: &'a HashMap<&str, Module>,
) -> HashMap<&'a str, bool> {
    modules
        .iter()
        .filter(|(_, module)| matches!(module.mtype, ModuleType::FlipFlop))
        .map(|(name, _)| (*name, false))
        .collect()
}

fn init_memories<'a>(
    modules: &'a HashMap<&str, Module>,
) -> HashMap<&'a str, HashMap<&'a str, Pulse>> {
    modules
        .iter()
        .filter(|(_, module)| matches!(module.mtype, ModuleType::Conjunction))
        .map(|(name, module)| {
            (
                *name,
                module
                    .inputs
                    .iter()
                    .map(|input| (*input, Pulse::Low))
                    .collect(),
            )
        })
        .collect()
}

fn passthrough(
    prev: &str,
    pulse: Pulse,
    name: &str,
    modules: &HashMap<&str, Module>,
    flags: &mut HashMap<&str, bool>,
    memories: &mut HashMap<&str, HashMap<&str, Pulse>>,
) -> Option<Pulse> {
    if !modules.contains_key(name) {
        return None;
    }
    let module = modules.get(name).unwrap();
    match module.mtype {
        ModuleType::Broadcaster => Some(pulse),
        ModuleType::FlipFlop => match pulse {
            Pulse::High => None,
            Pulse::Low => {
                let flag = flags.get_mut(name).unwrap();
                let pulse = if *flag { Pulse::Low } else { Pulse::High };
                *flag = !*flag;
                Some(pulse)
            }
        },
        ModuleType::Conjunction => {
            let memory = memories.get_mut(name).unwrap();
            let rememered_pulse = memory.get_mut(prev).unwrap();
            *rememered_pulse = pulse;
            let pulse = if memory.values().all(|v| *v == Pulse::High) {
                Pulse::Low
            } else {
                Pulse::High
            };
            Some(pulse)
        }
    }
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn part_one(input: &str) -> u32 {
    let modules = parse_input(input);
    let mut flags = init_flags(&modules);
    let mut memories = init_memories(&modules);

    let mut lo = 0;
    let mut hi = 0;

    for _ in 0..1000 {
        let mut queue: VecDeque<(&str, Pulse, &str)> = VecDeque::new();
        queue.push_back(("button", Pulse::Low, "broadcaster"));
        while let Some((prev, pulse, name)) = queue.pop_front() {
            match pulse {
                Pulse::Low => lo += 1,
                Pulse::High => hi += 1,
            }
            let pulse = passthrough(
                prev,
                pulse,
                name,
                &modules,
                &mut flags,
                &mut memories,
            );
            if let Some(pulse) = pulse {
                modules.get(name).unwrap().outputs.iter().for_each(
                    |output| {
                        queue.push_back((name, pulse, output));
                    },
                );
            }
        }
    }

    lo * hi
}

pub fn part_two(input: &str) -> usize {
    let modules = parse_input(input);
    let mut flags = init_flags(&modules);
    let mut memories = init_memories(&modules);

    let feed: &str = modules
        .values()
        .filter(|m| m.outputs.contains(&"rx"))
        .map(|m| m.name)
        .next()
        .unwrap();

    let mut feeds: HashMap<&str, usize> = modules
        .values()
        .filter(|m| m.outputs.contains(&feed))
        .map(|m| (m.name, 0))
        .collect();

    let mut pressed = 0;

    while feeds.values().any(|v| *v == 0) {
        pressed += 1;
        let mut queue: VecDeque<(&str, Pulse, &str)> = VecDeque::new();
        queue.push_back(("button", Pulse::Low, "broadcaster"));
        while let Some((prev, pulse, name)) = queue.pop_front() {
            if pulse == Pulse::Low {
                if let Some(cycle) = feeds.get_mut(name) {
                    *cycle = pressed
                }
            }
            let pulse = passthrough(
                prev,
                pulse,
                name,
                &modules,
                &mut flags,
                &mut memories,
            );
            if let Some(pulse) = pulse {
                modules.get(name).unwrap().outputs.iter().for_each(
                    |output| {
                        queue.push_back((name, pulse, output));
                    },
                );
            }
        }
    }
    let cycles: Vec<usize> = feeds.values().copied().collect();
    lcm(&cycles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(20);
        assert_eq!(part_one(&input), 32000000);
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
