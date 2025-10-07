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

fn breadth_first_search(graph: HashMap<usize, Vec<usize>>, start: usize) -> Vec<usize> {
    let queue = VecDeque::new();
    let order = HashSet::new();
    let distance = HashMap::with_capacity(graph.len());

    queue.push_back(start);
    order.insert(start);

    
}

pub fn run(input: Input) -> Answer {
    let mut lines = input.lines().into_iter();
    let (num_nodes, _num_edges) = parse_line(lines.next().unwrap());
    // map of vertex index to neighbors pairs
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::with_capacity(num_nodes);
    lines
        .into_iter()
        .map(|line| parse_line(line))
        .for_each(|(a, b)| {
            graph.entry(a).and_modify(|v| v.push(b)).or_insert(vec![b]);
        });

    let distance = breadth_first_search(graph, 1);
    return Answer::IntVec(distance);
}
