use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;
use templates::Scanner;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i64,
    u: usize,
    used: usize, // 0 for Layer 0, 1 for Layer 1
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Min-heap behavior
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let stdin = io::stdin();
    let mut sc = Scanner {
        reader: stdin.lock(),
        buffer: Vec::new(),
    };

    let n: usize = sc.next();
    let m: usize = sc.next();

    let mut adj = vec![vec![]; n + 1];
    for _ in 0..m {
        let u: usize = sc.next();
        let v: usize = sc.next();
        let c: i64 = sc.next();
        adj[u].push((v, c));
    }

    // dist[node][0] -> min cost to reach node without using coupon
    // dist[node][1] -> min cost to reach node having used the coupon
    let mut dist = vec![[i64::MAX; 2]; n + 1];
    dist[1][0] = 0;

    let mut pq = BinaryHeap::new();
    pq.push(State { cost: 0, u: 1, used: 0 });

    while let Some(State { cost, u, used }) = pq.pop() {
        if cost > dist[u][used] {
            continue;
        }

        for &(v, c) in &adj[u] {
            // Move within the same layer (Full price)
            if dist[u][used] + c < dist[v][used] {
                dist[v][used] = dist[u][used] + c;
                pq.push(State { cost: dist[v][used], u: v, used });
            }

            // Use coupon to jump from Layer 0 to Layer 1
            if used == 0 {
                let discounted = dist[u][0] + (c / 2);
                if discounted < dist[v][1] {
                    dist[v][1] = discounted;
                    pq.push(State { cost: discounted, u: v, used: 1 });
                }
            }
        }
    }

    println!("{}", dist[n][1]);
}