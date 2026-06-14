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

    // Regula 1: Distanța de la sursă la ea însăși trebuie să fie 0
    if d[s] != 0 {
        return false;
    }

    // Regula 2: Inegalitatea triunghiului
    for edge in &edges {
        if d[edge.u] + edge.w < d[edge.v] || d[edge.v] + edge.w < d[edge.u] {
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
            println!("DA (Partial)");
        } else {
            println!("NU");
        }
    }
}