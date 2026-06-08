use std::io;

use templates::Scanner;

struct Edge {
    u: usize,
    v: usize,
    w: i32,
}

fn main() {
    let stdin = io::stdin();
    let mut sc = Scanner {
        reader: stdin.lock(),
        buffer: Vec::new(),
    };

    // Citire N, M și T (ore)
    let n: usize = sc.next();
    let m: usize = sc.next();
    let t_hours: i32 = sc.next();
    let target_time = t_hours * 60; // Transformăm orele în minute

    // Citire calorii pentru fiecare intersecție (1-indexed)
    let mut calories = vec![0; n + 1];
    let mut unique_calories = Vec::new();
    for i in 1..=n {
        calories[i] = sc.next::<i32>();
        unique_calories.push(calories[i]);
    }

    // Sortăm și eliminăm duplicatele pentru a reduce numărul de intervale verificate
    unique_calories.sort_unstable();
    unique_calories.dedup();

    // Citire drumuri
    let mut edges = Vec::with_capacity(m);
    for _ in 0..m {
        let u: usize = sc.next();
        let v: usize = sc.next();
        let w: i32 = sc.next();
        edges.push(Edge { u, v, w });
    }

    let inf = 1_000_000_000;

    // Iterăm prin toate perechile posibile de limite de calorii [min_c, max_c]
    for i in 0..unique_calories.len() {
        for j in i..unique_calories.len() {
            let min_c = unique_calories[i];
            let max_c = unique_calories[j];

            // Inițializăm matricea de distanțe pentru Floyd-Warshall
            let mut dist = vec![vec![inf; n + 1]; n + 1];
            for k in 1..=n {
                dist[k][k] = 0;
            }

            // Construim graful doar cu muchiile ce conectează noduri valide din interval
            for edge in &edges {
                let u = edge.u;
                let v = edge.v;
                
                if calories[u] >= min_c && calories[u] <= max_c &&
                   calories[v] >= min_c && calories[v] <= max_c {
                    if edge.w < dist[u][v] {
                        dist[u][v] = edge.w;
                        dist[v][u] = edge.w;
                    }
                }
            }

            // Rulăm Floyd-Warshall pe nodurile permise
            for k in 1..=n {
                if calories[k] < min_c || calories[k] > max_c { continue; }
                for u in 1..=n {
                    if calories[u] < min_c || calories[u] > max_c { continue; }
                    for v in 1..=n {
                        if calories[v] < min_c || calories[v] > max_c { continue; }
                        
                        if dist[u][k] + dist[k][v] < dist[u][v] {
                            dist[u][v] = dist[u][k] + dist[k][v];
                        }
                    }
                }
            }

            // Căutăm o pereche de noduri (u, v) care are distanța exact egală cu target_time
            for u in 1..=n {
                if calories[u] < min_c || calories[u] > max_c { continue; }
                for v in 1..=n {
                    if calories[v] < min_c || calories[v] > max_c { continue; }
                    
                    if dist[u][v] == target_time {
                        println!("{} {} {} {}", u, v, min_c, max_c);
                        return; // Am găsit o soluție validă, încheiem programul
                    }
                }
            }
        }
    }
}