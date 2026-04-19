struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        // sorting for ease of solving
        candidates.sort();

        let mut results = Vec::new();
        let mut curr = Vec::new();

        Self::backtrack(&candidates, target, 0, &mut curr, &mut results);

        results
    }

    fn backtrack(
        candidates: &[i32],
        target: i32,
        start: usize,
        path: &mut Vec<i32>,
        results: &mut Vec<Vec<i32>>
    ) {
        // base case
        if target == 0 {
            results.push(path.clone());
        }

        for i in start..candidates.len() {
            // beacuse itrs sorted if curr is bigger we cant get an answer
            if candidates[i] > target {
                break;
            }
            
            // if this is not the first time picking a number for 
            // this specific position in the combination and it's the 
            // same as the previous one, skip it.
            if i > start && candidates[i] == candidates[i - 1] {
                continue;
            }

            path.push(candidates[i]);

            Self::backtrack(candidates, target - candidates[i], i + 1, path, results);
            path.pop();
        }
        
    }
}

fn main() {
    let candidates = vec![1, 1, 4, 2];
    let target = 2;
    
    let results = Solution::combination_sum2(candidates, target);
    println!("Possible sums: {:?}", results);
}