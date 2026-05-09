use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    t: i32,
    x: usize,
    y: usize,
}

// Min-heap behavior: prioritize the smallest time 't'
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.t.cmp(&self.t)
            .then_with(|| self.x.cmp(&self.x))
            .then_with(|| self.y.cmp(&self.y))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut visited = vec![vec![false; n]; n];
        let mut pq = BinaryHeap::new();

        // Start at (0,0)
        pq.push(State {
            t: grid[0][0],
            x: 0,
            y: 0,
        });
        visited[0][0] = true;

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some(State { t, x, y }) = pq.pop() {
            // If we reached the bottom-right corner, we found our minimum time
            if x == n - 1 && y == n - 1 {
                return t;
            }

            for (dx, dy) in directions.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if !visited[nx][ny] {
                        visited[nx][ny] = true;
                        // The time to reach the neighbor is the max of current time 
                        // and the neighbor's altitude
                        pq.push(State {
                            t: t.max(grid[nx][ny]),
                            x: nx,
                            y: ny,
                        });
                    }
                }
            }
        }

        0 // Should not reach here given problem constraints
    }
}

fn main() {
    let grid = vec![
        vec![0, 1, 2, 3, 4],
        vec![24, 23, 22, 21, 5],
        vec![12, 13, 14, 15, 16],
        vec![11, 17, 18, 19, 20],
        vec![10, 9, 8, 7, 6],
    ];

    let result = Solution::swim_in_water(grid);
    println!("Minimum time required: {}", result);
}