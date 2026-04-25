use std::{collections::VecDeque, io::{self, BufWriter, Write}, vec};

use templates::Scanner;

const DR: [i32; 4] = [-1, 1, 0, 0];
const DC: [i32; 4] = [0, 0, -1, 1];

#[inline]
fn is_valid(r: i32, c: i32, n: usize, m: usize) -> bool {
    r >= 0 && r < n as i32 && c >= 0 && c < m as i32
}

fn count_islands(n: usize, m: usize, map: &Vec<Vec<u32>>) -> [u32;4] {
    let mut visited = vec![vec![false; m]; n];
    let mut nr_islands = [0; 4];
    
    for i in 0..n {
        for j in 0..m {      
            let land_type = map[i][j] as usize;
            if land_type > 0 && !visited[i][j] {
                nr_islands[land_type] += 1;
                flood_fill(i, j, n, m, map, &mut visited);
            }
        }
    }

    nr_islands
}

fn flood_fill(i: usize, j: usize, n: usize, m: usize, map: &Vec<Vec<u32>>, visited: &mut Vec<Vec<bool>>) {
    let land_type = map[i][j] as usize;
    let mut queue = VecDeque::new();

    queue.push_back((i, j));
    visited[i][j] = true;

    while let Some((r, c)) = queue.pop_front() {
        for dir in 0..4 {
            let nr = r as i32 + DR[dir];
            let nc = c as i32 + DC[dir];

            if is_valid(nr, nc, n, m) {
                let (nr, nc) = (nr as usize, nc as usize);
                if !visited[nr][nc] && map[nr][nc] as usize == land_type {
                    visited[nr][nc] = true;
                    queue.push_back((nr, nc));
                }
            }
        }
    }
}

fn min_bridge_len(n: usize, m: usize, map: &Vec<Vec<u32>>) -> i32 {
    let mut queue = VecDeque::new();
    let mut dist = vec![vec![-1i32; m]; n];

    // starting from all the ones
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == 1 {
                queue.push_back((i, j));
                dist[i][j] = 0;
            }
        }
    }

    while let Some((r, c)) = queue.pop_front() {
        for i in 0..4 {
            let nr = r as i32 + DR[i];
            let nc = c as i32 + DC[i];

            if is_valid(nr, nc, n, m) {
                let (nr, nc) = (nr as usize, nc as usize);
                
                if map[nr][nc] == 2 && map[r][c] == 0 {
                    // if we found island 2
                    return dist[r][c];
                }

                if map[nr][nc] == 0 && dist[nr][nc] == -1 {
                    dist[nr][nc] = dist[r][c] + 1;
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    -1
}

fn main() {
    let stdin = io::stdin();

    let mut scanner = Scanner {
        reader: stdin.lock(),
        buffer: Vec::new()
    };

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let n: usize = scanner.next();
    let m: usize = scanner.next();

    let mut map = vec![vec![0; m]; n];

    for i in 0..n {
        let row: String = scanner.next();

        let chars = row.chars().enumerate();
        for (j, c) in chars {
            map[i][j] = c.to_digit(10).unwrap();
        }
    }

    // multiple bfs
    let nr_islands = count_islands(n, m, &map);
    let min_len = min_bridge_len(n, m, &map);

    writeln!(out, "{} {} {} {}", nr_islands[1], nr_islands[2], nr_islands[3], min_len).unwrap();

    out.flush().unwrap();
}