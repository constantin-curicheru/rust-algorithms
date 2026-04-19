use std::{io, vec};

use templates::Scanner;

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner {
        reader: stdin.lock(),
        buffer: Vec::new(),
    };

    let n: usize = scanner.next();
    let total_states = 1 << n;

    let mut path = Vec::with_capacity(total_states);
    let mut visited = vec![false; total_states];

    path.push(0);
    visited[0] = true;

    if solve(n, total_states, &mut visited, &mut path) {
        for code in path {
            // Formatting to binary string of length n
            println!("{:0>width$b}", code, width = n);
        }
    }
}

fn solve(n: usize, total: usize, visited: &mut Vec<bool>, path: &mut Vec<usize>) -> bool {
    if path.len() == total {
        return true;
    }

    let curr = *path.last().unwrap();

    // flipping each of the n bits
    for i in 0..n {
        let next = curr ^ (1 << i);

        if !visited[next] {
            visited[next] = true;
            path.push(next);

            if solve(n, total, visited, path) {
                return true;
            }

            // backtrack
            path.pop();
            visited[next] = false;
        }
    }

    false
}