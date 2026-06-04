use std::cmp::min;

struct Solution;

impl Solution {
    pub fn number_of_sets(n: i32, max_distance: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut ans = 0;
        let max_masks = 1 << n;

        // Iterate through all 2^n possible subsets of active branches
        for mask in 0..max_masks {
            // Initialize distance matrix with a large value (infinity simulation)
            let mut dist = vec![vec![1_000_000_000; n]; n];
            for i in 0..n {
                dist[i][i] = 0;
            }

            // Build the adjacency matrix for the current active subset (mask)
            for road in &roads {
                let u = road[0] as usize;
                let v = road[1] as usize;
                let w = road[2];

                // Only consider the road if both endpoints are active in the current mask
                if (mask & (1 << u)) != 0 && (mask & (1 << v)) != 0 {
                    dist[u][v] = min(dist[u][v], w);
                    dist[v][u] = min(dist[v][u], w);
                }
            }

            // Run Floyd-Warshall using only active branches
            for k in 0..n {
                if (mask & (1 << k)) == 0 { continue; }
                for i in 0..n {
                    if (mask & (1 << i)) == 0 { continue; }
                    for j in 0..n {
                        if (mask & (1 << j)) == 0 { continue; }
                        if dist[i][k] + dist[k][j] < dist[i][j] {
                            dist[i][j] = dist[i][k] + dist[k][j];
                        }
                    }
                }
            }

            // Validate if all remaining active pairs are within max_distance
            let mut valid = true;
            'check: for i in 0..n {
                if (mask & (1 << i)) == 0 { continue; }
                for j in 0..n {
                    if (mask & (1 << j)) == 0 { continue; }
                    if dist[i][j] > max_distance {
                        valid = false;
                        break 'check; // Break early if validation fails
                    }
                }
            }

            if valid {
                ans += 1;
            }
        }

        ans
    }
}

fn main() {
    let n1 = 3;
    let max_distance1 = 5;
    let roads1 = vec![vec![0, 1, 2], vec![1, 2, 10], vec![0, 2, 10]];
    println!(
        "Example 1 Output: {} (Expected: 5)", 
        Solution::number_of_sets(n1, max_distance1, roads1)
    );
}