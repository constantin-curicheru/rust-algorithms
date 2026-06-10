use std::cmp::max;
use std::io;

use templates::Scanner;

fn main() {
    let stdin = io::stdin();
    let mut sc = Scanner {
        reader: stdin.lock(),
        buffer: Vec::new(),
    };

    let n: usize = sc.next();
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
                max_edges[i][j] = 1;
            }
        }
    }

    // Algoritmul Roy-Floyd extins pentru distanță minimă și străzi maxime
    for k in 1..=n {
        for i in 1..=n {
            for j in 1..=n {
                if dist[i][k] < inf && dist[k][j] < inf {
                    let path_through_k = dist[i][k] + dist[k][j];
                    let edges_through_k = max_edges[i][k] + max_edges[k][j];

                    if path_through_k < dist[i][j] {
                        // Caz 1: Am găsit un drum strict mai scurt
                        dist[i][j] = path_through_k;
                        max_edges[i][j] = edges_through_k;
                    } else if path_through_k == dist[i][j] {
                        // Caz 2: Distanță egală, dar maximizăm numărul de străzi
                        max_edges[i][j] = max(max_edges[i][j], edges_through_k);
                    }
                }
            }
        }
    }

    // Afișarea primei matrice (Lungimile minime)
    for i in 1..=n {
        for j in 1..=n {
            print!("{}", dist[i][j]);
            if j < n { print!(" "); }
        }
        println!();
    }

    // Afișarea celei de-a doua matrice (Numărul maxim de străzi)
    for i in 1..=n {
        for j in 1..=n {
            print!("{}", max_edges[i][j]);
            if j < n { print!(" "); }
        }
        println!();
    }
}