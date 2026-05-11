use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, PartialEq)]
struct State {
    prob: f64,
    u: usize,
}

// Manually implement Ord for f64 to work with BinaryHeap
impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Use partial_cmp to handle floats; we assume no NaN in this problem
        self.prob.partial_cmp(&other.prob).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solution;

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let n = n as usize;
        let start = start as usize;
        let end = end as usize;

        // Build adjacency list for an undirected graph
        let mut adj = vec![vec![]; n];
        for (i, edge) in edges.iter().enumerate() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let p = succ_prob[i];
            adj[u].push((v, p));
            adj[v].push((u, p));
        }

        // Max probabilities array
        let mut max_probs = vec![0.0; n];
        max_probs[start] = 1.0;

        // Max-Heap for Dijkstra
        let mut pq = BinaryHeap::new();
        pq.push(State { prob: 1.0, u: start });

        while let Some(State { prob, u }) = pq.pop() {
            // If we reached the end, this is the maximum probability
            if u == end {
                return prob;
            }

            // If we found a better probability already, skip this stale entry
            if prob < max_probs[u] {
                continue;
            }

            for &(v, p) in &adj[u] {
                let new_prob = prob * p;
                if new_prob > max_probs[v] {
                    max_probs[v] = new_prob;
                    pq.push(State { prob: new_prob, u: v });
                }
            }
        }

        0.0
    }
}

fn main() {
    let n = 3;
    let edges = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
    let succ_prob = vec![0.5, 0.5, 0.2];
    let start = 0;
    let end = 2;

    let result = Solution::max_probability(n, edges, succ_prob, start, end);
    println!("Maximum probability: {:.5}", result); 
}