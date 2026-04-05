use std::{cmp::min, io};
fn main() {
    let mut dp: Vec<Vec<usize>>;
 
    let mut first_str = String::new();
    let mut second_str = String::new();
 
    io::stdin()
        .read_line(&mut first_str)
        .expect("Failed to read");
 
    io::stdin()
        .read_line(&mut second_str)
        .expect("Failed to read");
 
    let first_str_trim: Vec<char> = first_str.trim().chars().collect();
    let second_str_trim: Vec<char> = second_str.trim().chars().collect();
 
    let rows = first_str_trim.len() + 1;
    let cols = second_str_trim.len() + 1;
 
    dp = vec![vec![0; cols]; rows];
 
    for i in 0..cols {
        dp[0][i] = i;
    }
 
    for i in 0.. rows {
        dp[i][0] = i;
    }
 
    for i in 1..rows {
        for j in 1..cols {
            if first_str_trim[i - 1] == second_str_trim[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else { 
                dp[i][j] = 1 + min(
                    dp[i - 1][j],
                    min(dp[i][j - 1],
                        dp[i - 1][j - 1]
                    )
                );
            }
        }
    }
 
    println!("{}", dp[rows - 1][cols - 1]); 
    
}