use std::{collections::VecDeque, io};

use templates::Scanner;

fn main() {
    let stdin = io::stdin();

    let mut scanner = Scanner{
        reader: stdin.lock(),
        buffer: Vec::new()
    };

    let n: usize = scanner.next();
    let m: usize = scanner.next();
    
    let mut adj = vec![vec![]; n + 1];
    let mut in_degree = vec![0; n + 1];

    for _ in 0..m {
        let u: usize = scanner.next();
        let v: usize = scanner.next();
        adj[u].push(v);
        in_degree[v] += 1;
    }

    // intialising queue with 0 degree
    let mut queue = VecDeque::new();
    for i in 1..=n {
        if in_degree[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut result = Vec::with_capacity(n);
    while let Some(u) = queue.pop_front() {
        result.push(u);
        for &v in &adj[u] {
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                queue.push_back(v);
            }
        }
    }

    if result.len() == n {
        for (i, course) in result.iter().enumerate() {
            print!("{}{}", course, if i == n - 1 { "" } else { " " });
        }
    } else {
        println!("IMPOSSIBLE");
    }
}