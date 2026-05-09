use std::io;
use templates::Scanner;

struct Edge {
    u: usize,
    v: usize,
    w: i64,
}

fn dfs(u: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[u] = true;
    for &v in &adj[u] {
        if !visited[v] {
            dfs(v, adj, visited);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut sc = Scanner {
        reader: stdin.lock(),
        buffer: Vec::new(),
    };

    // safe "negative infinity"
    let ninf: i64 = -1e17 as i64;

    let n: usize = sc.next();
    let m: usize = sc.next();

    let mut edges = Vec::with_capacity(m);
    let mut adj = vec![vec![]; n + 1];
    let mut rev_adj = vec![vec![]; n + 1];

    for _ in 0..m {
        let u: usize = sc.next();
        let v: usize = sc.next();
        let w: i64 = sc.next();
        edges.push(Edge { u, v, w });
        adj[u].push(v);
        rev_adj[v].push(u);
    }

    // Reachability Check
    // A positive cycle only matters if it's on a path from 1 to N
    let mut from_1 = vec![false; n + 1];
    dfs(1, &adj, &mut from_1);

    let mut to_n = vec![false; n + 1];
    dfs(n, &rev_adj, &mut to_n);

    // Bellman-Ford (Maximizing path)
    let mut dist = vec![ninf; n + 1];
    dist[1] = 0;

    // Run relaxation N times. Updates on the N-th pass indicate a cycle.
    for i in 1..=n {
        for edge in &edges {
            if dist[edge.u] > ninf {
                if dist[edge.u] + edge.w > dist[edge.v] {
                    dist[edge.v] = dist[edge.u] + edge.w;
                    
                    // If we can still improve on the N-th iteration
                    // AND this node is part of a path from 1 to N
                    if i == n && from_1[edge.v] && to_n[edge.v] {
                        println!("-1");
                        return;
                    }
                }
            }
        }
    }

    println!("{}", dist[n]);
}