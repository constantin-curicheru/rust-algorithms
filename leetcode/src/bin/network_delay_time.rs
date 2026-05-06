use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: usize,
}

// The priority queue needs to sort by cost (ascending/min-priority)
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        
        // Build Adjacency List
        let mut adj = vec![vec![]; n + 1];
        for edge in times {
            adj[edge[0] as usize].push((edge[1] as usize, edge[2]));
        }

        // Distance array initialized to infinity
        let mut dists = vec![i32::MAX; n + 1];
        dists[k] = 0;

        // Priority Queue (Min-Heap)
        let mut pq = BinaryHeap::new();
        pq.push(State { cost: 0, position: k });

        while let Some(State { cost, position }) = pq.pop() {
            // If we already found a better way to this node, skip
            if cost > dists[position] {
                continue;
            }

            // Relax neighbors
            for &(next_node, weight) in &adj[position] {
                let next_cost = cost + weight;
                if next_cost < dists[next_node] {
                    dists[next_node] = next_cost;
                    pq.push(State { cost: next_cost, position: next_node });
                }
            }
        }

        // Extract results
        let mut max_time = 0;
        for i in 1..=n {
            if dists[i] == i32::MAX {
                return -1; // Node unreachable
            }
            max_time = max_time.max(dists[i]);
        }

        max_time
    }
}

fn main() {
    let times = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
    let n = 4;
    let k = 2;
    println!("Min time: {}", Solution::network_delay_time(times, n, k));
}