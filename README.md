# Karger-Stein Min-Cut Algorithm

Implementations of the Karger's Min-Cut and the Karger-Stein algorithms for weighted directed graphs.
___

### Problem Statement

Given an undirected graph G = (V, E), where V is the set of vertices and E is the set of edges, the minimum cut problem is to find a partition of V into two nonempty subsets, V1 and V2, such that the number of edges crossing the cut, E(V1, V2), is minimized.

### Implementation

The implementation is done in Rust (v1.72.1) and has an asymptotic time-complexity of O(mα(n)) for a single trial, where m is min(|V|^2, |E|) and α(n) is the [inverse Ackermann function](https://en.wikipedia.org/wiki/Inverse_Ackermann_function).
To achieve a lower bound of 1/|V| on the error probability, this amounts to O(mα(n) x |V|^2 x log(|V|)) for Karger's algorithm and to O(mα(n) x log^3(|V|)) for Karger-Stein.

### Usage

The input to the application should follow the following format:

    n
    
    v1 w1 e2
    
    v2 w2 e3
    
    ...
    
    vm wm sm

where 

    n is the number of vertices of the graph,
    m is the number of edges of the graph,
    (vi, wi, ei) is a directed edge from vertex vi to wi with a weight of ei (ei is optional and will defaul to 1).

### Test Cases:

The test cases included in the repository were obtained from [stanford-algs](https://github.com/beaunus/stanford-algs) and [KargerSteinAlgorithm](https://github.com/ArthurRouquan/KargerSteinAlgorithm).

### Disclaimer:

This repository and its contents are intended solely for educational purposes.