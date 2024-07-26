use std::collections::HashMap;

type Value = i32;
type Condition<'a> = (&'a str, &'a str, Value);
type Rule<'a> = (Option<Condition<'a>>, &'a str);
type Workflows<'a> = HashMap<&'a str, Vec<Rule<'a>>>;
type RatingsPart<'a> = HashMap<&'a str, Value>;

// (x, m, a, s)
type RatingsRange = [(Value, Value); 4];

fn parse_workflows(input: &str) -> Workflows {
    fn parse_rules(input: &str) -> Vec<Rule> {
        // a<2006:qkq,m>2090:A,rfg
        input
            .trim()
            .split(',')
            .map(|rule| {
                // a<2006:qkq
                let s: Vec<&str> = rule.split(':').collect();
                if s.len() == 1 {
                    (None, s[0])
                } else {
                    (
                        Some((
                            &s[0][0..1], // var name
                            &s[0][1..2], // compare '<' or '>'
                            s[0][2..].parse::<Value>().unwrap(),
                        )),
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

fn parse_input(input: &str) -> (Workflows, Vec<RatingsPart>) {
    let s: Vec<&str> = input.trim().split("\n\n").collect();
    (parse_workflows(s[0]), parse_parts(s[1]))
}

fn process(ratings: &RatingsPart, workflows: &Workflows) -> bool {
    let mut name = "in";
    while let Some(rules) = workflows.get(name) {
        for (condition, next) in rules {
            if condition.is_none() {
                name = next;
                break;
            }
            let (var, cmp, value) = condition.unwrap();
            let &rating = ratings.get(var).unwrap();
            if match cmp {
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
    parts
        .iter()
        .filter(|ratings| process(ratings, &workflows))
        .map(|ratings| ratings.values().sum::<Value>())
        .sum()
}

fn dfs(
    workflows: &Workflows,
    name: &str,
    mut ratings: RatingsRange,
    ranges: &mut Vec<RatingsRange>,
) {
    if name == "A" {
        ranges.push(ratings);
        return;
    }
    if name == "R" {
        return;
    }
    let rules = workflows.get(name).unwrap();
    for (condition, next) in rules.iter() {
        let mut next_ratings = ratings;
        if let Some((var, cmp, value)) = condition {
            let i = match *var {
                "x" => 0,
                "m" => 1,
                "a" => 2,
                "s" => 3,
                _ => panic!(),
            };
            match *cmp {
                "<" => {
                    next_ratings[i].1 = next_ratings[i].1.min(*value) - 1;
                    ratings[i].0 = ratings[i].0.max(*value);
                }
                ">" => {
                    next_ratings[i].0 = next_ratings[i].0.max(*value) + 1;
                    ratings[i].1 = ratings[i].1.min(*value);
                }
                _ => panic!(),
            }
        }
        dfs(workflows, next, next_ratings, ranges)
    }
}

pub fn part_two(input: &str) -> usize {
    let (workflows, _) = parse_input(input);
    let mut ranges: Vec<RatingsRange> = vec![];
    dfs(
        &workflows,
        "in",
        [(1, 4000), (1, 4000), (1, 4000), (1, 4000)],
        &mut ranges,
    );
    ranges
        .into_iter()
        .map(|ratings| {
            ratings
                .into_iter()
                .map(|(a, b)| b - a + 1)
                .map(|v| v as usize)
                .product::<usize>()
        })
        .sum()
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
