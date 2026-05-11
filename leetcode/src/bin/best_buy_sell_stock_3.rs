struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        // Initialize states
        // buy1 and buy2 are initialized to very small values (spent money)
        let mut buy1 = i32::MIN;
        let mut sell1 = 0;
        let mut buy2 = i32::MIN;
        let mut sell2 = 0;

        for price in prices {
            // We use .max() to greedily pick the best financial outcome for each state
            
            // 1st Transaction: Buying
            buy1 = buy1.max(-price);
            
            // 1st Transaction: Selling
            sell1 = sell1.max(buy1 + price);
            
            // 2nd Transaction: Buying (re-investing profit from sell1)
            buy2 = buy2.max(sell1 - price);
            
            // 2nd Transaction: Selling
            sell2 = sell2.max(buy2 + price);
        }

        sell2
    }
}

fn main() {
    let prices = vec![3, 3, 5, 0, 0, 3, 1, 4];
    let result = Solution::max_profit(prices);
    println!("Maximum profit with at most 2 transactions: {}", result);
}