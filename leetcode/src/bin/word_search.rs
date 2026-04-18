struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        let letters: Vec<char> = word.chars().collect();
        let rows = board.len();
        let cols = board[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if Self::backtrack(&mut board, &letters, r, c, 0) {
                    return true;
                }
            }
        }

        false
    }

    fn backtrack(
        board: &mut Vec<Vec<char>>,
        word: &[char],
        r: usize,
        c: usize,
        index: usize
    ) -> bool {
        if index == word.len() {
            return true;
        }

        if r >= board.len() || c >= board[0].len() || board[r][c] != word[index] {
            return false;
        }

        // marking it as we used the letter
        let temp = board[r][c];
        board[r][c] = '#';

        let found = (r > 0 && Self::backtrack(board, word, r - 1, c, index + 1)) ||
            (r + 1 < board.len() && Self::backtrack(board, word, r + 1, c, index + 1)) ||
            (c > 0 && Self::backtrack(board, word, r, c - 1, index + 1)) ||
            (c + 1 < board[0].len() && Self::backtrack(board, word, r, c + 1, index + 1));

        board[r][c] = temp;
        found
    }
}

fn main() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];

    let word = String::from("ABCCED");
    
    let result = Solution::exist(board, word);
    println!("Word found: {}", result);
}