use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let src = src as usize;
        let dst = dst as usize;

        // Build adjacency list
        let mut adj = vec![vec![]; n];
        for flight in flights {
            adj[flight[0] as usize].push((flight[1] as usize, flight[2]));
        }

        // Track min costs to reach each city
        let mut min_costs = vec![i32::MAX; n];
        min_costs[src] = 0;

        // Queue for BFS
        let mut queue = VecDeque::new();
        queue.push_back((src, 0));

        // We can take up to k stops, which means k + 1 flights.
        let mut stops = 0;
        while stops <= k && !queue.is_empty() {
            let level_size = queue.len();
            
            // Process current "level"
            for _ in 0..level_size {
                let (u, current_cost) = queue.pop_front().unwrap();

                for &(v, price) in &adj[u] {
                    let new_cost = current_cost + price;
                    
                    // Only explore if this path is cheaper 
                    // than anything we've seen at any previous level.
                    if new_cost < min_costs[v] {
                        min_costs[v] = new_cost;
                        queue.push_back((v, new_cost));
                    }
                }
            }
            stops += 1;
        }

        if min_costs[dst] == i32::MAX { -1 } else { min_costs[dst] }
    }
}

fn main() {
    let n = 4;
    let flights = vec![
        vec![0, 1, 100],
        vec![1, 2, 100],
        vec![2, 0, 100],
        vec![1, 3, 600],
        vec![2, 3, 200],
    ];
    let src = 0;
    let dst = 3;
    let k = 1;

    let result = Solution::find_cheapest_price(n, flights, src, dst, k);
    println!("The cheapest price is: {}", result); 
}