use std::{collections::VecDeque, io, vec};

use templates::Scanner;

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner {
        reader: stdin.lock(),
        buffer: Vec::new()
    };

    let n: usize = scanner.next();
    let m: usize = scanner.next();

    let mut dist = vec![vec![-1i32; n + 1]; n + 1];


    // marking trees
    for _ in 0..m {
        let cx: usize = scanner.next();
        let cy: usize = scanner.next();

        if cx <= n && cy <= n {
           dist[cx][cy] = -2;
        }
    }
    let start_x: usize = scanner.next();
    let start_y: usize = scanner.next();
    let end_x: usize = scanner.next();
    let end_y: usize = scanner.next();

    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));
    dist[start_x][start_y] = 1;

    let dx = [1, -1, 0, 0];
    let dy = [0, 0, 1, -1];

    // bfs
    while let Some((x, y)) = queue.pop_front() {
        if x == end_x && y == end_y {
            println!("{}", dist[x][y]);
        }

        for i in 0..4 {
            let nx = x as i32 + dx[i];
            let ny = y as i32 + dy[i];

            if nx > 0 && nx <= n as i32 && ny > 0 && ny <= n as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                // if not visited
                if dist[nx][ny] == -1 {
                    dist[nx][ny] = dist[x][y] + 1;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

}