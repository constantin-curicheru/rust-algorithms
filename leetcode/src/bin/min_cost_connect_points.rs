struct Solution;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n == 0 {
            return 0;
        }

        let mut total_cost = 0;
        let mut visited = vec![false; n];
        
        // dist[i] stochează distanța minimă de la MST-ul curent la punctul i
        let mut min_dist = vec![i32::MAX; n];
        
        // Pornim arbitrar de la primul punct
        min_dist[0] = 0;

        for _ in 0..n {
            let mut u = usize::MAX;
            let mut current_min = i32::MAX;

            // Găsim nodul nevizitat cu distanța minimă până la MST
            for i in 0..n {
                if !visited[i] && min_dist[i] < current_min {
                    current_min = min_dist[i];
                    u = i;
                }
            }

            // Marcăm nodul ca fiind inclus în MST și adăugăm costul
            visited[u] = true;
            total_cost += current_min;

            // Actualizăm distanțele pentru vecinii nevizitați ai lui u
            for v in 0..n {
                if !visited[v] {
                    // Calculăm distanța Manhattan între punctul u și punctul v
                    let dist = (points[u][0] - points[v][0]).abs() 
                             + (points[u][1] - points[v][1]).abs();
                    
                    if dist < min_dist[v] {
                        min_dist[v] = dist;
                    }
                }
            }
        }

        total_cost
    }
}

fn main() {
    let points = vec![
        vec![0, 0],
        vec![2, 2],
        vec![3, 10],
        vec![5, 2],
        vec![7, 0],
    ];

    let result = Solution::min_cost_connect_points(points);
    println!("Costul minim pentru conectarea tuturor punctelor: {}", result);

}