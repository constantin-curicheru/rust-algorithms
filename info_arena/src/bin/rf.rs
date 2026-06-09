use std::io;

use templates::Scanner;

fn main() {
    let stdin = io::stdin();
    let mut sc = Scanner {
        reader: stdin.lock(),
        buffer: Vec::new(),
    };

    let n: usize = sc.next();

    // inf reprezintă absența unui drum direct
    let inf = 1_000_000_000;
    let mut dist = vec![vec![inf; n + 1]; n + 1];
    let mut max_edges = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=n {
            let weight: i32 = sc.next();
            if i == j {
                dist[i][j] = 0;
                max_edges[i][j] = 0;
            } else if weight > 0 {
                dist[i][j] = weight;
                max_edges[i][j] = 1; // Un drum direct înseamnă exact 1 stradă
            }
        }
    }

}