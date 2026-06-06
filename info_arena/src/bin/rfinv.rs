use std::io::{self, BufRead};

use templates::Scanner;

fn solve_test_case<R: BufRead>(sc: &mut Scanner<R>) -> bool {
    let n_token = sc.reader.read_line(&mut String::new());
    if n_token.is_err() { return false; }
    
    let n: usize = sc.next();
    let m: usize = sc.next();

    // Track if a direct undirected edge exists between two nodes
    let mut has_edge = vec![vec![false; n + 1]; n + 1];
    for _ in 0..m {
        let u: usize = sc.next();
        let v: usize = sc.next();
        has_edge[u][v] = true;
        has_edge[v][u] = true;
    }

    // Parse the target Roy-Floyd matrix
    let mut dist = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            dist[i][j] = sc.next::<i32>();
        }
    }

    // Check Triangle Inequality condition:
    for i in 1..=n {
        for j in 1..=n {
            for k in 1..=n {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    return false; 
                }
            }
        }
    }

    true
}

fn main() {
    let stdin = io::stdin();
    let mut sc = Scanner {
        reader: stdin.lock(),
        buffer: Vec::new(),
    };

    let t: usize = sc.next();
    for _ in 0..t {
        if solve_test_case(&mut sc) {
            println!("DA");
        } else {
            println!("NU");
        }
    }
}