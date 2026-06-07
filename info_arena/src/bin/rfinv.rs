use std::io::{self, BufRead};

pub struct Scanner<R> {
    pub reader: R,
    pub buffer: Vec<String>,
}

impl<R: BufRead> Scanner<R> {
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(word) = self.buffer.pop() {
                return word.parse().ok().unwrap();
            }
            let mut line = String::new();
            self.reader.read_line(&mut line).unwrap();
            self.buffer = line.split_whitespace().map(String::from).collect();
            self.buffer.reverse();
        }
    }
}

fn solve_test_case<R: BufRead>(sc: &mut Scanner<R>) -> bool {
    let n: usize = sc.next();
    let m: usize = sc.next();

    let mut has_edge = vec![vec![false; n + 1]; n + 1];
    for _ in 0..m {
        let u: usize = sc.next();
        let v: usize = sc.next();
        has_edge[u][v] = true;
        has_edge[v][u] = true;
    }

    let mut dist = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            dist[i][j] = sc.next::<i32>();
        }
    }

    // 1. Triangle Inequality Check
    for i in 1..=n {
        for j in 1..=n {
            for k in 1..=n {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    return false;
                }
            }
        }
    }

    // 2. Edge Necessity Check
    for i in 1..=n {
        for j in 1..=n {
            if i == j { continue; }

            let mut can_bypass = false;
            for k in 1..=n {
                if k != i && k != j {
                    if dist[i][j] == dist[i][k] + dist[k][j] {
                        can_bypass = true;
                        break;
                    }
                }
            }

            // If the route cannot be broken down into multi-hop paths,
            // the graph must contain a direct edge between node i and node j.
            if !can_bypass && !has_edge[i][j] {
                return false;
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