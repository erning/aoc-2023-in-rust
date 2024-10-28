use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

// Define type aliases for readability
type Graph<'a> = HashMap<&'a str, HashMap<&'a str, i32>>;

fn bfs<'a>(
    residual: &Graph<'a>,
    source: &'a str,
    sink: &'a str,
    parent: &mut HashMap<&'a str, &'a str>,
) -> bool {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(source);
    visited.insert(source);

    while let Some(u) = queue.pop_front() {
        if let Some(neighbors) = residual.get(u) {
            for (&v, &capacity) in neighbors {
                if !visited.contains(v) && capacity > 0 {
                    queue.push_back(v);
                    visited.insert(v);
                    parent.insert(v, u);

                    if v == sink {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn edmonds_karp<'a>(
    graph: &mut Graph<'a>,
    source: &'a str,
    sink: &'a str,
) -> i32 {
    let mut max_flow = 0;
    let mut parent = HashMap::new();

    while bfs(graph, source, sink, &mut parent) {
        // Find the bottleneck capacity in the path from source to sink
        let mut path_flow = i32::MAX;
        let mut v = sink;

        while v != source {
            let u = parent[v];
            let capacity = *graph.get(u).unwrap().get(v).unwrap();
            path_flow = path_flow.min(capacity);
            v = u;
        }

        // Update the capacities in the residual graph along the path
        v = sink;
        while v != source {
            let u = parent[v];
            *graph.get_mut(u).unwrap().get_mut(v).unwrap() -= path_flow;
            *graph.entry(v).or_default().entry(u).or_insert(0) += path_flow;
            v = u;
        }

        max_flow += path_flow;
    }

    max_flow
}

fn parse_input(input: &str) -> Graph {
    let mut map = Graph::new();
    let mut add_item = |a, b| {
        if let Some(item) = map.get_mut(a) {
            item.insert(b, 1);
        } else {
            let mut item: HashMap<&str, i32> = HashMap::new();
            item.insert(b, 1);
            map.insert(a, item);
        }
    };
    input.lines().for_each(|line| {
        let mut iter = line.splitn(2, ':');
        let u = iter.next().unwrap().trim();
        iter.next().unwrap().split_whitespace().for_each(|v| {
            add_item(u, v);
            add_item(v, u)
        });
    });
    map
}

fn find_source_part<'a>(
    graph: &Graph<'a>,
    source: &'a str,
) -> HashSet<&'a str> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(source);
    while let Some(u) = queue.pop_front() {
        if !visited.insert(u) {
            continue;
        }
        graph[u].iter().filter(|(_, c)| *c > &0).for_each(|(v, _)| {
            queue.push_back(v);
        });
    }
    visited
}

pub fn part_one(input: &str) -> usize {
    let graph = parse_input(input);
    let source = graph.keys().next().unwrap();
    for sink in graph.keys().skip(1) {
        let mut graph = graph.clone();
        let max_flow = edmonds_karp(&mut graph, source, sink);
        if max_flow == 3 {
            let source_part = find_source_part(&graph, source);
            let a = source_part.len();
            let b = graph.len() - a;
            return a * b;
        }
    }
    0
}

pub fn part_two<'a>(_: &str) -> &'a str {
    "fifty stars"
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(25);
        assert_eq!(part_one(&input), 54);
    }
}
