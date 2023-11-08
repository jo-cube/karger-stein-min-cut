pub enum DirectedEdge {
    Unweighted(usize, usize),
    Weighted(usize, usize, usize),
}

impl From<(usize, usize)> for DirectedEdge {
    fn from(value: (usize, usize)) -> Self {
        DirectedEdge::Unweighted(value.0, value.1)
    }
}

impl From<(usize, usize, usize)> for DirectedEdge {
    fn from(value: (usize, usize, usize)) -> Self {
        DirectedEdge::Weighted(value.0, value.1, value.2)
    }
}
