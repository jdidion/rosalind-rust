//! Breadth-First Search
//! url: http://rosalind.info/problems/bfs
//! Given: A simple directed graph with n≤103 vertices in the edge list format.
//! Return: An array D[1..n] where D[i] is the length of a shortest path from the vertex 1 to the
//! vertex i (D[1]=0). If i is not reachable from 1 set D[i] to −1.
use crate::{Answer, Input};
use std::collections::{HashMap, HashSet, VecDeque};

fn parse_line(s: &str) -> (usize, usize) {
    let (a, b) = s.split_once(" ").unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn breadth_first_search(
    graph: &HashMap<usize, Vec<usize>>,
    num_nodes: usize,
    start: usize,
) -> Vec<i64> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut distance: HashMap<usize, i64> = HashMap::with_capacity(graph.len());

    queue.push_back(start);
    visited.insert(start);
    distance.insert(start, 0);

    while let Some(node) = queue.pop_front() {
        let d = distance[&node];
        if let Some(neighbors) = graph.get(&node) {
            for &n in neighbors {
                if visited.insert(n) {
                    distance.insert(n, d + 1);
                    queue.push_back(n);
                }
            }
        }
    }

    (1..=num_nodes)
        .map(|i| distance.get(&i).copied().unwrap_or(-1))
        .collect()
}

pub fn run(input: Input) -> Answer {
    let mut lines = input.lines().into_iter();
    let (num_nodes, _num_edges) = parse_line(lines.next().unwrap());
    // map of vertex index to neighbors pairs
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::with_capacity(num_nodes);
    lines.map(parse_line).for_each(|(a, b)| {
        graph.entry(a).and_modify(|v| v.push(b)).or_insert(vec![b]);
    });

    let distance = breadth_first_search(&graph, num_nodes, 1);
    Answer::IntVec(distance)
}
