struct Solution;

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        let nums_len = nums.len();
        // edge case
        if nums_len == 0 {
            return vec![];
        }

        // sorting the values so we dont double check division both ways
        nums.sort_unstable();

        // to avoid checking vec again we will hold best idx and len
        let mut best_idx = 0;
        let mut max_len = 1;

        let mut dp = vec![1; nums_len];
        let mut parent = vec![None; nums_len];

        for i in 0..nums_len {
            // check previous numbers
            let curr_num = nums[i];

            for j in 0..i {
                // checking previous numbers for division
                if curr_num % nums[j] == 0 {
                    // if found compare with best length so far
                    if dp[i] < dp[j] + 1 {
                        dp[i] = dp[j] + 1;
                        parent[i] = Some(j);
                    }
                }
            }

            // saving if its the best answer so far
            if dp[i] > max_len {
                best_idx = i;
                max_len = dp[i];
            }
        };
        
        // backtracking with parent to recreate subset
        let mut rsp_subset = Vec::with_capacity(max_len);
        let mut curr = Some(best_idx);
        
        while let Some(idx) = curr {
            rsp_subset.push(nums[idx]);
            curr = parent[idx];
        }

        rsp_subset.reverse();
        rsp_subset
    }
}

fn main() {
    let envelopes = vec![1, 5, 4, 6];
    
    let result = Solution::largest_divisible_subset(envelopes);
    println!("Largest Divisible Subset: {:?}", result);
}