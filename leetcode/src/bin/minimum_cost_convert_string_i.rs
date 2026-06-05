struct Solution;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let inf = 1e15 as i64;
        let mut dist = vec![vec![inf; 26]; 26];

        // A character can transform into itself for 0 cost
        for i in 0..26 {
            dist[i][i] = 0;
        }

        // Build the initial graph
        for i in 0..original.len() {
            let u = (original[i] as u8 - b'a') as usize;
            let v = (changed[i] as u8 - b'a') as usize;
            let w = cost[i] as i64;
            // There can be multiple edges between the same two nodes; keep the minimum cost
            dist[u][v] = dist[u][v].min(w);
        }

        // Floyd-Warshall Algorithm to find all-pairs shortest paths
        for k in 0..26 {
            for i in 0..26 {
                for j in 0..26 {
                    if dist[i][k] < inf && dist[k][j] < inf {
                        dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                    }
                }
            }
        }

        // Calculate total cost for conversion
        let mut total_cost: i64 = 0;
        let source_bytes = source.as_bytes();
        let target_bytes = target.as_bytes();

        for i in 0..source_bytes.len() {
            let u = (source_bytes[i] - b'a') as usize;
            let v = (target_bytes[i] - b'a') as usize;

            if dist[u][v] >= inf {
                return -1; // Conversion is impossible
            }
            total_cost += dist[u][v];
        }

        total_cost
    }
}

fn main() {
    let source = String::from("abcd");
    let target = String::from("acbe");
    let original = vec!['a', 'b', 'c', 'c', 'e', 'd'];
    let changed = vec!['b', 'c', 'b', 'e', 'b', 'e'];
    let cost = vec![2, 5, 5, 1, 2, 20];

    let result = Solution::minimum_cost(source, target, original, changed, cost);
    println!("Minimum cost to convert: {}", result);
}