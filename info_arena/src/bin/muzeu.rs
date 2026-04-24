use std::{collections::VecDeque, io};

use templates::Scanner;

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner {
        reader: stdin.lock(),
        buffer: Vec::new()
    };

    let n: usize = scanner.next();
    let mut dist = vec![vec![-1i32; n]; n];
    let mut queue = VecDeque::new();

    for i in 0..n {
        let row: String = scanner.next();

        for (j, c) in row.chars().enumerate() {
            match c {
                'P' => {
                    dist[i][j] = 0;
                    queue.push_back((i, j));
                }
                '#' => {
                    dist[i][j] = -2;
                }
                _ => {}
            }
        }
    }

    let dr = [-1, 1, 0, 0];
    let dc = [0, 0, -1, 1];

    while let Some((r, c)) = queue.pop_front() {
        for i in 0..4 {
            let nr = r as i32 + dr[i];
            let nc = c as i32 + dc[i];

            if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                
                if dist[nr][nc] == -1 {
                    dist[nr][nc] = dist[r][c] + 1;
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    // printing
    for i in 0..n {
        for j in 0..n {
            print!("{} ", dist[i][j]);
        }
        println!("");
    }
}