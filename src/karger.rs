use std::time::Instant;

use crate::data_structures::graph::Graph;
use crate::min_cut_algo::MinCutAlgo;
use crate::traits::{EdgeWeightedGraph, MinCutGraph};

pub struct KargerAlgo<'a> {
    graph: &'a Graph,
}

impl<'a> KargerAlgo<'a> {
    pub fn new(graph: &'a Graph) -> KargerAlgo<'a> {
        Self { graph }
    }
}

impl<'a> MinCutAlgo for KargerAlgo<'a> {
    fn graph(&self) -> &Graph {
        self.graph
    }

    fn single_trial_fail_prob(&self) -> f64 {
        1f64 - 2f64 / (self.graph.num_vertices().pow(2) as f64)
    }

    fn iterate(&self) -> usize {
        let contracted_graph = self.graph.contract_full();
        contracted_graph.weight()
    }

    fn approx_execute(&self, verbose: bool) -> usize {
        let step = self.graph.num_vertices();
        let mut num_trials = step * step;
        let mut min_cut = usize::MAX;
        let mut i = 0;
        let instant = Instant::now();
        while i < num_trials {
            i += 1;
            let new_min_cut = self.iterate();
            if new_min_cut < min_cut {
                min_cut = new_min_cut;
                num_trials = 2 * i + step;
            }
        }
        let elapsed = instant.elapsed();
        if verbose {
            self.print_stats(num_trials, min_cut, elapsed);
        }
        min_cut
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::graph::Graph;
    use crate::karger::KargerAlgo;
    use crate::min_cut_algo::MinCutAlgo;
    use crate::traits::EdgeWeightedGraph;

    #[test]
    fn karger_sanity_test() {
        use crate::data_structures::api::DirectedEdge;

        let edges = [
            (0, 1, 1),
            (0, 1, 2),
            (0, 2, 1),
            (1, 0, 2),
            (1, 2, 1),
            (1, 2, 3),
            (2, 1, 3),
        ][..]
            .iter()
            .map(|it| DirectedEdge::from(*it));
        let graph: Graph = Graph::from_directed_edges(3, edges);

        let karger = KargerAlgo::new(&graph);
        karger.iterate_n(9, false);
        karger.iterate_success_lower_bound(0.9, false);
        karger.execute(false);
    }
}
