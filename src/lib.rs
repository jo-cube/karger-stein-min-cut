mod data_structures;
pub mod karger;
pub mod karger_stein;
mod min_cut_algo;
mod traits;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;

    use crate::data_structures::graph::Graph;
    use crate::karger::KargerAlgo;
    use crate::karger_stein::KargerSteinAlgo;
    use crate::min_cut_algo::MinCutAlgo;
    use crate::utils::read_graph;

    #[test]
    fn karger_sample() {
        let path = "src/files/input_random_40_200.txt";
        let graph = read_graph(path);
        KargerAlgo::new(&graph).approx_execute(true);
    }

    #[test]
    fn karger_stein_sample() {
        let path = "src/files/input_random_40_200.txt";
        let graph = read_graph(path);
        KargerSteinAlgo::<10>::new(&graph).approx_execute(true);
    }

    fn get_all_test_cases() -> Vec<(PathBuf, Graph, usize)> {
        let paths = std::fs::read_dir("src/files")
            .unwrap()
            .map(Result::unwrap)
            .map(|it| it.path())
            .collect::<Box<[PathBuf]>>();

        let (mut inputs, mut outputs): (Vec<_>, Vec<_>) = paths
            .into_iter()
            .partition(|it| it.file_name().unwrap().to_str().unwrap().contains("input"));
        inputs.sort();
        outputs.sort();

        let expected_min_cut = |path| {
            BufReader::new(File::open(path).unwrap())
                .lines()
                .nth(0)
                .unwrap()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        };

        inputs
            .into_iter()
            .zip(outputs.into_iter())
            .map(|(p1, p2)| (p1.clone(), read_graph(p1), expected_min_cut(p2)))
            .collect()
    }

    #[ignore]
    #[test]
    fn karger_test_all() {
        for (file, graph, expected_min_cut) in get_all_test_cases() {
            println!();
            println!(
                "Testing file: {:?} | Expected Min Cut: {}",
                file, expected_min_cut
            );

            let actual_min_cut = KargerAlgo::new(&graph).approx_execute(true);

            assert_eq!(
                actual_min_cut,
                expected_min_cut,
                "Karger algorithm (approx. version) failed for file {:?}. Re-try with stricter error bounds.",
                file.clone()
            );
        }
    }

    #[ignore]
    #[test]
    fn karger_stein_test_all() {
        for (file, graph, expected_min_cut) in get_all_test_cases() {
            println!();
            println!(
                "Testing file: {:?} | Expected Min Cut: {}",
                file, expected_min_cut
            );

            let actual_min_cut = KargerSteinAlgo::<10>::new(&graph).approx_execute(true);

            assert_eq!(
                actual_min_cut,
                expected_min_cut,
                "Karger-Stein algorithm (approx. version) failed for file {:?}. Re-try with stricter error bounds.",
                file.clone()
            );
        }
    }
}
