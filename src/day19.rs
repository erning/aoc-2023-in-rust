use std::collections::HashMap;

type Value = i64;
type Rule<'a> = (&'a str, &'a str, Value, &'a str);
type Workflow<'a> = (&'a str, Vec<Rule<'a>>);
type RatingsPart<'a> = HashMap<&'a str, Value>;

fn parse_workflows(input: &str) -> Vec<Workflow> {
    fn parse_rules(input: &str) -> Vec<Rule> {
        // a<2006:qkq,m>2090:A,rfg
        input
            .trim()
            .split(',')
            .map(|rule| {
                // a<2006:qkq
                let s: Vec<&str> = rule.split(':').collect();
                if s.len() == 1 {
                    ("", "", 0, s[0])
                } else {
                    (
                        &s[0][0..1], // var name
                        &s[0][1..2], // condition '<' or '>'
                        s[0][2..].parse::<Value>().unwrap(),
                        s[1],
                    )
                }
            })
            .collect()
    }

    input
        .trim()
        .lines()
        .map(|line| {
            // px{a<2006:qkq,m>2090:A,rfg}
            let s: Vec<&str> = line.trim().split(&['{', '}'][..]).collect();
            let name = s[0];
            let rules = parse_rules(s[1]);
            (name, rules)
        })
        .collect()
}

fn parse_parts(input: &str) -> Vec<RatingsPart> {
    input
        .trim()
        .lines()
        .map(|line| {
            // {x=787,m=2655,a=1222,s=2876}
            line.trim_matches(&['{', '}'])
                .split(',')
                .map(|s| (&s[0..1], s[2..].parse::<Value>().unwrap()))
                .collect::<Vec<(&str, Value)>>()
                .into_iter()
                .collect()
        })
        .collect()
}

fn parse_input(input: &str) -> (Vec<Workflow>, Vec<RatingsPart>) {
    let s: Vec<&str> = input.trim().split("\n\n").collect();
    (parse_workflows(s[0]), parse_parts(s[1]))
}

fn process(
    ratings: &RatingsPart,
    workflows: &HashMap<&str, Vec<Rule>>,
) -> bool {
    let mut name = "in";
    while let Some(rules) = workflows.get(name) {
        for (var, condition, value, next) in rules {
            if var.is_empty() {
                name = next;
                break;
            }
            let rating = ratings.get(var).unwrap();
            if match *condition {
                ">" => rating > value,
                "<" => rating < value,
                _ => panic!(),
            } {
                name = next;
                break;
            }
        }
        match name {
            "A" => return true,
            "R" => return false,
            _ => continue,
        }
    }
    panic!()
}

pub fn part_one(input: &str) -> Value {
    let (workflows, parts) = parse_input(input);
    let workflows: HashMap<&str, Vec<Rule>> = workflows.into_iter().collect();

    parts
        .iter()
        .filter(|ratings| process(ratings, &workflows))
        .map(|ratings| ratings.values().sum::<Value>())
        .sum()
}

pub fn part_two(input: &str) -> Value {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(19);
        assert_eq!(part_one(&input), 19114);
        assert_eq!(part_two(&input), 167409079868000);
    }
}
