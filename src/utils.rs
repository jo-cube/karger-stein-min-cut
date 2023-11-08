use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::data_structures::api::DirectedEdge;
use crate::data_structures::graph::Graph;
use crate::traits::EdgeWeightedGraph;

pub fn read_graph(path: impl AsRef<Path>) -> Graph {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let n = lines
        .by_ref()
        .take(1)
        .map(Result::unwrap)
        .nth(0)
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();

    let parse_directed_edge = |line: String| {
        let mut t = line
            .trim()
            .split_whitespace()
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .take(3);
        let v = t.next().unwrap();
        let w = t.next().unwrap();
        let weight = t.next().unwrap_or(1);
        DirectedEdge::Weighted(v, w, weight)
    };

    let directed_edges = lines.map(Result::unwrap).map(parse_directed_edge);

    Graph::from_directed_edges(n, directed_edges)
}
