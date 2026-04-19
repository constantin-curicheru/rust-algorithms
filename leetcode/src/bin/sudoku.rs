struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::backtrack(board);
    }
    fn backtrack(board: &mut Vec<Vec<char>>) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] == '.' {
                    // trying every number
                    for num in '1'..='9' {
                        if Self::is_valid(board, row, col, num) {
                            board[row][col] = num;

                            // continue on this path
                            if Self::backtrack(board) {
                                return true;
                            }

                            // else try another
                            board[row][col] = '.';

                        }
                    }
                    // if none of the numbers work this isnt good path
                    return false;
                }
            }
        }

        // no cells left
        true
    }

    fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, num: char) -> bool {
        for i in 0..9 {
            // col
            if board[i][col] == num {
                return false;
            }
            // row
            if board[row][i] == num {
                return false;
            }
            // check small square
            let row_box = 3 * (row / 3) + i / 3;
            let col_box = 3 * (col / 3) + i % 3;
            if board[row_box][col_box] == num {
                return false;
            }
        }

        true
    }
}

fn main() {
    let mut board = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9'],
    ];
    
    Solution::solve_sudoku(&mut board);
    println!("Solution: {:?}", board);
}