use std::io;
use templates::Scanner;

struct Project {
    start: i32,
    end: i32,
    profit: i64,
}

fn main() {
    let stdin = io::stdin();
    // intialising scanner
    let mut scan = Scanner {
        reader: stdin.lock(),
        buffer: Vec::new()
    };

    let n: usize = scan.next();
    let mut projects: Vec<Project> = Vec::with_capacity(n);
    // reading input
    for _ in 0..n {
        let start: i32 = scan.next();
        let end: i32 = scan.next();
        let profit: i64 = scan.next();
        
        projects.push(Project { start, end, profit });
    }

    // sorting projects by end date
    projects.sort_unstable_by_key(|p| p.end);

    let mut dp = vec![0i64; n + 1];

    // creating dp vector
    for i in 1..=n {
        // getting best with project i considered
        let project = &projects[i-1];

        // the best profit so far
        let option_skip = dp[i-1];

        // calculating idx of last project that doesnt collide with current
        let j = projects[..i - 1].partition_point(|x| x.end < project.start);

        // calculating profit if we choose to take thisproject
        let option_take = project.profit + dp[j];

        dp[i] = option_skip.max(option_take);
    }

    // printing best profit
    println!("{}", dp[n]);
}
