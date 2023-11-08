use std::borrow::Borrow;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge {
    pub other: usize,
    pub weight: usize,
}

impl From<&(usize, usize)> for Edge {
    fn from(edge: &(usize, usize)) -> Self {
        Edge {
            other: edge.0,
            weight: edge.1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub vertex: usize,
    pub weight: usize,
    pub edges: Rc<[Edge]>,
}

impl Node {
    pub fn from(vertex: usize, edges: &impl Borrow<[(usize, usize)]>) -> Self {
        Self {
            vertex,
            weight: edges.borrow().iter().map(|edge| edge.1).sum(),
            edges: edges.borrow().iter().map(|edge| Edge::from(edge)).collect(),
        }
    }
}

#[derive(Debug)]
pub struct MergeUtil {
    pub stack: Box<[usize]>,
    pub merge_proxy: Box<[usize]>,
}

impl MergeUtil {
    pub fn new(n: usize) -> Self {
        MergeUtil {
            stack: vec![0; n].into_boxed_slice(),
            merge_proxy: vec![0; n].into_boxed_slice(),
        }
    }
}
