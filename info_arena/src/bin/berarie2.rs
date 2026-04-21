use std::{collections::VecDeque, io};

use std::io::{BufWriter, Write};

use templates::Scanner;

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner {
        reader: stdin.lock(),
        buffer: Vec::new()
    };
    let n: usize = scanner.next();
    let m: usize = scanner.next();
    let p: usize = scanner.next();

    let mut adj_trans = vec![vec![]; n + 1];

    for _ in 0..m {
        let u: usize = scanner.next();
        let v: usize = scanner.next();
        // inversing for transposed graph
        adj_trans[v].push(u);   
    }

    let mut visited = vec![false; n + 1];
    let mut queue = VecDeque::new();

    // reading beraries and saving in the queue
    for _ in 0..p {
        let berarie: usize = scanner.next();
        if !visited[berarie] {
            visited[berarie] = true;
            queue.push_back(berarie);
        }
    }

    // bfs from every brewery
    while let Some(node) = queue.pop_front() {
        for &neigh in &adj_trans[node] {
            if !visited[neigh] {
                visited[neigh] = true;
                queue.push_back(neigh);
            } 
        }
    }

    // getting result
    let mut result = Vec::new();
    for i in 1..=n {
        if !visited[i] {
            result.push(i);
        }
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    
    writeln!(out, "{}", result.len()).unwrap();
    for node in result {
        writeln!(out, "{}", node).unwrap();
    }
}