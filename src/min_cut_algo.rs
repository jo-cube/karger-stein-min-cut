use std::time::{Duration, Instant};

use crate::data_structures::graph::Graph;
use crate::traits::EdgeWeightedGraph;

pub trait MinCutAlgo {
    fn graph(&self) -> &Graph;

    fn single_trial_fail_prob(&self) -> f64;

    fn iterate(&self) -> usize;

    fn approx_execute(&self, verbose: bool) -> usize;

    fn success_lower_bound(&self, n: usize) -> f64 {
        let n_trial_fail_prob = self.single_trial_fail_prob().powi(n as i32);
        1f64 - n_trial_fail_prob
    }

    fn min_num_trials(&self, prob: f64) -> usize {
        let fail_prob = 1f64 - prob;
        (fail_prob.log2() / self.single_trial_fail_prob().log2()).ceil() as usize
    }

    fn execute(&self, verbose: bool) -> usize {
        let expected_lower_bound = 1f64 - 1f64 / self.graph().num_vertices() as f64;
        self.iterate_success_lower_bound(expected_lower_bound, verbose)
    }

    fn iterate_success_lower_bound(&self, prob: f64, verbose: bool) -> usize {
        let n = self.min_num_trials(prob);
        let instant = Instant::now();
        let min_cut = self.iterate_n(n, false);
        let elapsed = instant.elapsed();
        if verbose {
            self.print_stats(n, min_cut, elapsed);
        }
        min_cut
    }

    fn iterate_n(&self, n: usize, verbose: bool) -> usize {
        let mut min_cut = usize::MAX;
        let instant = Instant::now();
        for _ in 0..n {
            min_cut = std::cmp::min(min_cut, self.iterate())
        }
        let elapsed = instant.elapsed();
        if verbose {
            self.print_stats(n, min_cut, elapsed);
        }
        min_cut
    }

    fn print_stats(&self, num_trials: usize, min_cut: usize, duration: Duration) {
        let success_prob = self.success_lower_bound(num_trials) * 100f64;
        println!("Min Cut: {} | |V|: {} | |E|: {} | Number of trials: {} | Probability of success: {:.2}% | Elapsed time: {:?}",
                 min_cut, self.graph().num_vertices(), self.graph().num_edges(), num_trials, success_prob, duration
        );
    }
}
