use std::io;

use templates::Scanner;

const MOD: i32 = 666013;

// Funcție ajutătoare pentru ridicarea la putere la modulo rapid
fn fast_pow(mut base: i32, mut exp: i32) -> i32 {
    let mut res = 1;
    base %= MOD;
    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % MOD;
        }
        base = (base * base) % MOD;
        exp /= 2;
    }
    res
}

fn dfs(
    u: usize,
    p: usize,
    timer: &mut i32,
    adj: &Vec<Vec<usize>>,
    idx: &mut Vec<i32>,
    low: &mut Vec<i32>,
    components_count: &mut Vec<i32>,
) {
    *timer += 1;
    idx[u] = *timer;
    low[u] = *timer;
    
    let mut children = 0;

    for &v in &adj[u] {
        if v == p {
            continue;
        }
        
        if idx[v] != 0 {
            // Muchie de întoarcere
            low[u] = low[u].min(idx[v]);
        } else {
            // Muchie de avansare în arborele DFS
            children += 1;
            dfs(v, u, timer, adj, idx, low, components_count);
            
            low[u] = low[u].min(low[v]);
            
            // Dacă u nu este rădăcină și v nu se poate întoarce deasupra lui u
            if p != 0 && low[v] >= idx[u] {
                components_count[u] += 1;
            }
        }
    }

    if p == 0 {
        // Cazul special pentru rădăcina arborelui DFS
        if children > 1 {
            components_count[u] = children - 1;
        } else {
            components_count[u] = 0; // Nu rupe graful dacă are doar un copil
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
    for _ in 0..m {
        let u: usize = sc.next();
        let v: usize = sc.next();
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut idx = vec![0; n + 1];
    let mut low = vec![0; n + 1];
    
    // components_count[u] reține numărul de componente EXTRA formate după eliminarea lui u.
    let mut components_count = vec![0; n + 1];
    let mut timer = 0;

    // Rulăm DFS din primul nod (graful este garantat conex)
    dfs(1, 0, &mut timer, &adj, &mut idx, &mut low, &mut components_count);

    // Calculăm rezultatul final pentru fiecare nod
    for i in 1..=n {
        if components_count[i] == 0 {
            // Dacă eliminarea nodului nu separă graful în mai multe bucăți, nu poate fi clepsidră
            println!("0");
        } else {
            let total_components = components_count[i] + 1;
            // Formula: (2^K - 2) % MOD
            let mut ans = fast_pow(2, total_components) - 2;
            if ans < 0 {
                ans += MOD;
            }
            println!("{}", ans);
        }
    }
}