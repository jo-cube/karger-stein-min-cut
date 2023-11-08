use std::borrow::Borrow;
use std::collections::HashMap;
use std::slice::{Iter, IterMut};

use rand::rngs::ThreadRng;
use rand::Rng;

use crate::data_structures::api::DirectedEdge;
use crate::traits::{EdgeWeightedGraph, MinCutGraph};

use super::fenwick_tree::FenwickTree;
use super::graph_util::*;
use super::union_find::UnionFind;

#[derive(Debug)]
pub struct Graph {
    n: usize,
    weight: usize,
    adj: Box<[Node]>,
}

impl Graph {
    fn from<T, U>(adj: &T) -> Self
    where
        U: Borrow<[(usize, usize)]>,
        T: Borrow<[U]>,
    {
        let adj: Box<[Node]> = adj
            .borrow()
            .iter()
            .enumerate()
            .map(|(i, edges)| Node::from(i, edges))
            .collect();

        Graph {
            n: adj.len(),
            weight: adj.iter().map(|node| node.weight).sum(),
            adj,
        }
    }

    fn pick_random_edge<'a>(
        rng: &mut ThreadRng,
        weight: usize,
        adj: &'a Box<[Option<Node>]>,
        vertex_map: &mut UnionFind,
        vertex_weights_map: &FenwickTree,
    ) -> (usize, &'a Node, usize, &'a Node) {
        let r = rng.gen_range(0..weight) + 1;

        let (v, mut rw) = vertex_weights_map.lower_entry(r);
        let n1 = adj[v].as_ref().unwrap();

        let random_edge = n1
            .edges
            .iter()
            .skip_while(|edge| {
                rw += edge.weight;
                rw < r
            })
            .nth(0)
            .unwrap();

        let w = vertex_map.root(random_edge.other);
        let n2 = adj[w].as_ref().unwrap();

        (v, n1, w, n2)
    }

    fn vertex_condenser<T: FnMut(usize) -> usize>(
        v: usize,
        edges: IterMut<Iter<Edge>>,
        merge_util: &mut MergeUtil,
        mut vertex_mapper: T,
    ) -> Node {
        let mut node_weight = 0;
        let mut stack_size = 0;

        let MergeUtil { stack, merge_proxy } = merge_util;

        for e in edges.flatten() {
            let root = vertex_mapper(e.other);
            if root == v {
                continue;
            }
            if merge_proxy[root] == 0 {
                stack[stack_size] = root;
                stack_size += 1;
            }
            merge_proxy[root] += e.weight;
        }

        let edges = (0..stack_size)
            .map(|i| {
                let vertex = stack[i];
                let weight = merge_proxy[vertex];
                node_weight += weight;
                merge_proxy[vertex] = 0;
                Edge::from(&(vertex, weight))
            })
            .collect();

        Node {
            vertex: v,
            weight: node_weight,
            edges,
        }
    }
}

impl EdgeWeightedGraph for Graph {
    fn from_directed_edges(n: usize, directed_edges: impl Iterator<Item = DirectedEdge>) -> Self {
        let mut adj = vec![HashMap::<usize, usize>::new(); n];
        for edge in directed_edges {
            match edge {
                DirectedEdge::Unweighted(v, w) => {
                    assert!(v < n && w < n);
                    if v == w {
                        continue;
                    }
                    let edge_weight = *adj[v].get(&w).unwrap_or(&0);
                    adj[v].insert(w, edge_weight + 1);
                }
                DirectedEdge::Weighted(v, w, weight) => {
                    assert!(v < n && w < n);
                    if v == w {
                        continue;
                    }
                    let edge_weight = *adj[v].get(&w).unwrap_or(&0);
                    adj[v].insert(w, edge_weight + weight);
                }
            }
        }
        let adj = adj
            .into_iter()
            .map(|it| {
                it.iter()
                    .map(|(w, weight)| (*w, *weight))
                    .collect::<Box<[(usize, usize)]>>()
            })
            .collect::<Box<[Box<[(usize, usize)]>]>>();
        Graph::from(&adj)
    }

    fn num_vertices(&self) -> usize {
        self.n
    }

    fn num_edges(&self) -> usize {
        self.adj.iter().map(|node| node.edges.len()).sum()
    }

    fn weight(&self) -> usize {
        self.adj.iter().map(|node| node.weight).sum()
    }

    fn adjacency_list(&self) -> &[Node] {
        self.adj.borrow()
    }
}

impl MinCutGraph for Graph {
    fn contract_full(&self) -> Self {
        self.contract(2)
    }

    fn contract(&self, t: usize) -> Self {
        let mut rng = rand::thread_rng();

        let mut weight = self.weight;
        let mut adj = self
            .adj
            .iter()
            .map(|node| Some(node.clone()))
            .collect::<Box<[Option<Node>]>>();

        let mut merge_util = MergeUtil::new(self.n);

        let mut vertex_map = UnionFind::new(self.n);
        let mut vertex_weights_map = FenwickTree::from(
            self.adj
                .iter()
                .map(|node| node.weight)
                .collect::<Box<[usize]>>()
                .as_ref(),
        );

        for _ in t..self.n {
            let (v, n1, w, n2) = Self::pick_random_edge(
                &mut rng,
                weight,
                &adj,
                &mut vertex_map,
                &vertex_weights_map,
            );

            let x = vertex_map.union(v, w);

            let node = Self::vertex_condenser(
                x,
                [n1.edges.iter(), n2.edges.iter()].iter_mut(),
                &mut merge_util,
                |v| vertex_map.root(v),
            );
            weight -= n1.weight + n2.weight - node.weight;

            vertex_weights_map.update(v, n1.weight, true);
            vertex_weights_map.update(w, n2.weight, true);
            vertex_weights_map.update(x, node.weight, false);

            adj[v] = None;
            adj[w] = None;
            adj[x] = Some(node);
        }

        let vertex_map = vertex_map.condense(&mut merge_util);

        let adj = adj
            .iter()
            .filter_map(|node| node.as_ref())
            .map(|node| {
                let x = vertex_map[node.vertex];
                let vertex_mapper = |v| vertex_map[v];
                Self::vertex_condenser(
                    x,
                    [node.edges.iter()].iter_mut(),
                    &mut merge_util,
                    vertex_mapper,
                )
            })
            .collect();

        Graph { n: t, weight, adj }
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::api::DirectedEdge;
    use crate::data_structures::graph::Graph;
    use crate::data_structures::graph_util::{Edge, MergeUtil, Node};
    use crate::data_structures::union_find::UnionFind;
    use crate::traits::{EdgeWeightedGraph, MinCutGraph};

    #[test]
    fn create_graph() {
        let expected_fmt =
            "Graph { \
            n: 3, weight: 14, adj: [\
            Node { vertex: 0, weight: 4, edges: [Edge { other: 1, weight: 3 }, Edge { other: 2, weight: 1 }] }, \
            Node { vertex: 1, weight: 7, edges: [Edge { other: 0, weight: 2 }, Edge { other: 2, weight: 5 }] }, \
            Node { vertex: 2, weight: 3, edges: [Edge { other: 1, weight: 3 }] }] \
        }";

        let adj = vec![vec![(1, 3), (2, 1)], vec![(0, 2), (2, 5)], vec![(1, 3)]];
        let actual: Graph = Graph::from(&adj);

        assert_eq!(format!("{actual:?}"), expected_fmt);
        assert_eq!(actual.num_vertices(), 3);
        assert_eq!(actual.num_edges(), 5);
        assert_eq!(actual.weight(), 14);

        let edges = [
            (0, 1, 1),
            (0, 1, 2),
            (0, 2, 1),
            (1, 0, 2),
            (1, 2, 2),
            (1, 2, 3),
            (2, 1, 3),
        ][..]
            .iter()
            .map(|it| DirectedEdge::from(*it));
        let actual: Graph = Graph::from_directed_edges(3, edges);

        assert_eq!(actual.num_vertices(), 3);
        assert_eq!(actual.num_edges(), 5);
        assert_eq!(actual.weight(), 14);
    }

    #[test]
    fn test_vertex_condenser() {
        let mut vertex_map = UnionFind::new(6);
        vertex_map.union(3, 4);
        vertex_map.union(0, 3);
        vertex_map.union(1, 2);
        assert_eq!(vertex_map.ids[..], [3, 1, 1, 3, 3, 5]);

        let mut merge_util = MergeUtil::new(6);

        let converter = |edges: Vec<(usize, usize)>| {
            edges
                .iter()
                .map(|edge| Edge::from(edge))
                .collect::<Vec<Edge>>()
        };

        let edges1 = converter(vec![(3, 4), (5, 1), (0, 4), (0, 5), (1, 2)]);
        let edges2 = converter(vec![(4, 2), (2, 6), (0, 3), (2, 4), (5, 3)]);

        let actual = Graph::vertex_condenser(
            3,
            [edges1.iter(), edges2.iter()][..].iter_mut(),
            &mut merge_util,
            |v| vertex_map.root(v),
        );

        let expected = Node::from(3, &vec![(5, 4), (1, 12)]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_contract() {
        let adj = vec![
            vec![(1, 1), (2, 1), (3, 1)],
            vec![(0, 1), (4, 1), (5, 1), (3, 1)],
            vec![(0, 1), (4, 1)],
            vec![(1, 1), (0, 1), (4, 1)],
            vec![(1, 1), (2, 1), (3, 1), (5, 1)],
            vec![(1, 1), (4, 1)],
        ];

        let graph: Graph = Graph::from(&adj);

        let contracted = graph.contract_full();
        assert_eq!(contracted.n, 2);

        let contracted = graph.contract(4);
        assert_eq!(contracted.n, 4);
    }
}
