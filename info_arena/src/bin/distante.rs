use std::io::{self, BufRead};

use templates::Scanner;

struct Edge {
    u: usize,
    v: usize,
    w: i32,
}

fn solve_graph<R: BufRead>(sc: &mut Scanner<R>) -> bool {
    let n: usize = sc.next();
    let m: usize = sc.next();
    let s: usize = sc.next();

    let mut d = vec![0; n + 1];
    for i in 1..=n {
        d[i] = sc.next::<i32>();
    }

    let mut edges = Vec::with_capacity(m);
    for _ in 0..m {
        let u: usize = sc.next();
        let v: usize = sc.next();
        let w: i32 = sc.next();
        edges.push(Edge { u, v, w });
    }

    // 1. Validare Sursă
    if d[s] != 0 {
        return false;
    }

    // 2. Validare inegalitate triunghi
    for edge in &edges {
        if d[edge.u] + edge.w < d[edge.v] || d[edge.v] + edge.w < d[edge.u] {
            return false;
        }
    }

    // 3. Validare Justificare Drumuri
    let mut justified = vec![false; n + 1];
    justified[s] = true; // Sursa este justificată implicit de costul 0

    for edge in &edges {
        if d[edge.u] + edge.w == d[edge.v] {
            justified[edge.v] = true;
        }
        if d[edge.v] + edge.w == d[edge.u] {
            justified[edge.u] = true;
        }
    }

    // Dacă există cel puțin un nod care nu a putut fi justificat, configurarea e greșită
    for i in 1..=n {
        if !justified[i] {
            return false;
        }
    }

    true
}

fn main() {
    let stdin = io::stdin();
    let mut sc = Scanner {
        reader: stdin.lock(),
        buffer: Vec::new(),
    };

    let t: usize = sc.next();
    for _ in 0..t {
        if solve_graph(&mut sc) {
            println!("DA");
        } else {
            println!("NU");
        }
    }
}