use crate::data_structures::api::DirectedEdge;
use crate::data_structures::graph_util::Node;

pub(crate) trait EdgeWeightedGraph {
    fn from_directed_edges(n: usize, directed_edges: impl Iterator<Item = DirectedEdge>) -> Self;
    fn num_vertices(&self) -> usize;
    fn num_edges(&self) -> usize;
    fn weight(&self) -> usize;
    fn adjacency_list(&self) -> &[Node];
}

pub(crate) trait MinCutGraph: EdgeWeightedGraph {
    fn contract_full(&self) -> Self;

    fn contract(&self, t: usize) -> Self;
}
