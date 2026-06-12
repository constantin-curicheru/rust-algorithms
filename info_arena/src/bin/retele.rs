use std::io;

use templates::Scanner;

// DFS pe graful normal pentru a stabili ordinea de finalizare a nodurilor
fn dfs1(u: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
    visited[u] = true;
    for &v in &adj[u] {
        if !visited[v] {
            dfs1(v, adj, visited, order);
        }
    }
    order.push(u);
}

// DFS pe graful transpus (inversat) pentru a colecta componentele
fn dfs2(u: usize, rev_adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>, component: &mut Vec<usize>) {
    visited[u] = true;
    component.push(u);
    for &v in &rev_adj[u] {
        if !visited[v] {
            dfs2(v, rev_adj, visited, component);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut sc = Scanner {
        reader: stdin.lock(),
        buffer: Vec::new(),
    };

    let n: usize = sc.next();
    let m: usize = sc.next();

    let mut adj = vec![vec![]; n + 1];
    let mut rev_adj = vec![vec![]; n + 1];

    for _ in 0..m {
        let u: usize = sc.next();
        let v: usize = sc.next();
        adj[u].push(v);
        rev_adj[v].push(u);
    }

    // Determinăm ordinea nodurilor folosind primul DFS
    let mut visited = vec![false; n + 1];
    let mut order = Vec::with_capacity(n);
    for i in 1..=n {
        if !visited[i] {
            dfs1(i, &adj, &mut visited, &mut order);
        }
    }

    // Identificăm rețelele folosind DFS pe graful inversat
    let mut rev_visited = vec![false; n + 1];
    let mut networks = Vec::new();

    // Parcurgem ordinea inversată (de la sfârșit la început ca o stivă)
    for &u in order.iter().rev() {
        if !rev_visited[u] {
            let mut current_network = Vec::new();
            dfs2(u, &rev_adj, &mut rev_visited, &mut current_network);
            
            // Sortăm abonații din interiorul aceleiași rețele crescător
            current_network.sort_unstable();
            networks.push(current_network);
        }
    }

    // Sortăm rețelele între ele după ID-ul minim al primului abonat
    networks.sort_by_key(|net| net[0]);

    // Afișăm numărul total de rețele gasite
    println!("{}", networks.len());
    
    // Afișăm fiecare rețea conform formatului cerut
    for net in networks {
        print!("{}", net.len());
        for subscriber in net {
            print!(" {}", subscriber);
        }
        println!();
    }
}