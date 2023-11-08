use std::time::Instant;

use crate::data_structures::graph::Graph;
use crate::karger::KargerAlgo;
use crate::min_cut_algo::MinCutAlgo;
use crate::traits::{EdgeWeightedGraph, MinCutGraph};

pub struct KargerSteinAlgo<'a, const N: usize = 10usize> {
    graph: &'a Graph,
}

impl<'a, const N: usize> KargerSteinAlgo<'a, N> {
    pub fn new(graph: &'a Graph) -> KargerSteinAlgo<'a, N> {
        assert!(N >= 2);
        Self { graph }
    }
}

impl<'a, const N: usize> MinCutAlgo for KargerSteinAlgo<'a, N> {
    fn graph(&self) -> &Graph {
        self.graph
    }

    fn single_trial_fail_prob(&self) -> f64 {
        let branch_height = 2f64 * (self.graph.num_vertices() as f64).log2().ceil();
        1f64 - 1f64 / (branch_height + 1f64)
    }

    fn iterate(&self) -> usize {
        let n = self.graph.num_vertices();
        if n <= N {
            let karger = KargerAlgo::new(self.graph);
            karger
                .iterate_success_lower_bound(1f64 / (self.graph.num_vertices() as f64).ln(), false)
        } else {
            let t = std::cmp::max(2usize, (n as f64 / 2f64.sqrt()).ceil() as usize);
            let g1 = self.graph.contract(t);
            let g2 = self.graph.contract(t);

            let min_cut1 = KargerSteinAlgo::<N>::new(&g1).iterate();
            let min_cut2 = KargerSteinAlgo::<N>::new(&g2).iterate();
            std::cmp::min(min_cut1, min_cut2)
        }
    }

    fn approx_execute(&self, verbose: bool) -> usize {
        let step = (self.graph.num_vertices() as f64).ln().ceil() as usize;
        let mut num_trials = step;
        let mut min_cut = usize::MAX;

        let mut i = 0;
        let instant = Instant::now();
        while i < num_trials {
            i += 1;
            let new_min_cut = self.iterate();
            if new_min_cut < min_cut {
                min_cut = new_min_cut;
                num_trials = i + step;
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
    use crate::karger_stein::KargerSteinAlgo;
    use crate::min_cut_algo::MinCutAlgo;
    use crate::traits::EdgeWeightedGraph;

    #[test]
    fn karger_stein_sanity_test() {
        use crate::data_structures::api::DirectedEdge;

        let edges = [
            (0, 1, 1),
            (0, 1, 2),
            (0, 2, 1),
            (1, 0, 2),
            (1, 2, 5),
            (1, 2, 3),
            (2, 1, 3),
            (3, 1, 8),
            (3, 2, 5),
        ][..]
            .iter()
            .map(|it| DirectedEdge::from(*it));
        let graph: Graph = Graph::from_directed_edges(4, edges);

        let karger_stein = KargerSteinAlgo::<3>::new(&graph);
        karger_stein.iterate_n(9, false);
        karger_stein.iterate_success_lower_bound(0.9, false);
        karger_stein.execute(false);
    }
}
