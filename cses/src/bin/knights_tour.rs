use std::io;

use std::io::BufRead;

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

// Knight's 8 possible L-shaped moves
const DX: [i8; 8] = [-2, -2, -1, -1, 1, 1, 2, 2];
const DY: [i8; 8] = [-1, 1, -2, 2, -2, 2, -1, 1];

fn get_degree(board: &[[i32; 8]; 8], x: i8, y: i8) -> i8{
    let mut count = 0;
    for i in  0..8 {
        let nx = x + DX[i];
        let ny = y + DY[i];

        if nx >= 0 && nx < 8 && ny >= 0 && ny < 8 && board[nx as usize][ny as usize] == 0 {
            count += 1;
        }
    }
    count
}

fn solve(board: &mut [[i32; 8]; 8], x: i8, y: i8, move_count: i32) -> bool {
    board[x as usize][y as usize] = move_count;

    // base case we are at last cell
    if move_count == 64 {
        return true;
    }

    // improvement, try moves with lowest onward degree
    let mut moves = Vec::new();
    for i in 0..8 {
        let nx = x + DX[i];
        let ny = y + DY[i];
        if nx >= 0 && nx < 8 && ny >= 0 && ny < 8 && board[nx as usize][ny as usize] == 0 {
            let degree = get_degree(board, nx, ny);
            moves.push((degree, nx, ny));
        }
    }

    // Sort moves by degree ascending
    moves.sort_by_key(|m| m.0);

    for (_, nx, ny) in moves {
        if solve(board, nx, ny, move_count + 1) {
            return true;
        }
    }
    
    // backtrack
    board[x as usize][y as usize] = 0;
    false
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner {
        reader: stdin.lock(),
        buffer: Vec::new()
    };

    let y: i8 = scanner.next();
    let x: i8 = scanner.next();

    let mut board = [[0; 8]; 8];

    if solve(&mut board, x - 1, y - 1, 1) {
        for row in board.iter() {
            for (i, val) in row.iter().enumerate() {
                print!("{}{}", val, if i == 7 { "" } else { " " });
            }
            println!();
        }
    }
}