use std::char;

struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut results = Vec::new();
        let mut curr = Vec::new();
        let chars: Vec<char> = s.chars().collect();

        Self::backtrack(&chars, 0, &mut curr, &mut results);

        results
    }

    fn backtrack(
        chars: &[char],
        start: usize,
        path: &mut Vec<String>,
        results: &mut Vec<Vec<String>>
    ) {
        // base case: partitioned the string
        if start == chars.len() {
            results.push(path.clone());
            return;
        }

        for end in start..chars.len() {
            if Self::is_palindrome(chars, start, end) {
                // extracting substring and adding it
                let substring: String = chars[start..=end].iter().collect();
                path.push(substring);

                // continue from next char
                Self::backtrack(chars, end + 1, path, results);

                // remove last cut to try a longer one
                path.pop();
            }
        }
    }

    fn is_palindrome(chars: &[char], mut left: usize, mut right: usize) -> bool {
        while left < right {
            if chars[left] != chars[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

fn main() {
    let s = "abc".to_string();
    
    let result = Solution::partition(s);
    println!("Palindrome partition: {:?}", result);
}