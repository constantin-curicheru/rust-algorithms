struct Solution;

impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        if envelopes.is_empty() { return 0; }

        // sort
        envelopes.sort_unstable_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1]) // Height descending
            } else {
                a[0].cmp(&b[0]) // Width ascending
            }
        });

        // finding the longest incr subseq
        let mut tails = Vec::new();
        for env in envelopes {
            let h = env[1];
            
            // search for first element >= h
            match tails.binary_search(&h) {
                Ok(_) => {
                    // h already exists in tails
                },
                Err(i) => {
                    if i == tails.len() {
                        tails.push(h); // h is larger than any element
                    } else {
                        tails[i] = h; // Replace the first element >= h
                    }
                }
            }
        }

        tails.len() as i32
    }
}

fn main() {
    let envelopes = vec![
        vec![5, 4],
        vec![6, 4],
        vec![6, 7],
        vec![2, 3]
    ];
    
    let result = Solution::max_envelopes(envelopes);
    println!("Max envelopes: {}", result);
}