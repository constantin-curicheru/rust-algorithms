use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Scanner {
            reader,
            buffer: Vec::new(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut line = String::new();
            self.reader.read_line(&mut line).expect("Failed read");
            self.buffer = line.split_whitespace().rev().map(String::from).collect();
        }
    }
}

// Structură pentru muchiile grafului
struct Edge {
    to: usize,
    weight: i32,
}

// Structură pentru starea din coada de priorități
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    dist: i32,
    node: usize,
}

fn main() -> io::Result<()> {
    // Deschidem fișierele de intrare și ieșire
    let fin = File::open("catun.in")?;
    let mut fout = File::create("catun.out")?;
    let mut sc = Scanner::new(BufReader::new(fin));

    let n: usize = sc.next(); // Număr de așezări
    let m: usize = sc.next(); // Număr de drumuri
    let k: usize = sc.next(); // Număr de fortărețe

    let mut dist = vec![i32::MAX; n + 1];
    let mut fortress_owner = vec![0; n + 1];
    let mut is_fortress = vec![false; n + 1];
    
    // Min-Heap pentru Dijkstra (folosind Reverse pentru a extrage distanța minimă)
    let mut pq = BinaryHeap::new();

    // Citim fortărețele și le adăugăm ca puncte de pornire
    for _ in 0..k {
        let f: usize = sc.next();
        dist[f] = 0;
        fortress_owner[f] = f;
        is_fortress[f] = true;
        pq.push(Reverse(State { dist: 0, node: f }));
    }

    // Construim graful ca listă de adiacență
    let mut graph: Vec<Vec<Edge>> = Vec::with_capacity(n + 1);
    for _ in 0..=n {
        graph.push(Vec::new());
    }

    for _ in 0..m {
        let u: usize = sc.next();
        let v: usize = sc.next();
        let w: i32 = sc.next();
        
        graph[u].push(Edge { to: v, weight: w });
        graph[v].push(Edge { to: u, weight: w });
    }

    // Algoritmul Dijkstra Multi-Source
    while let Some(Reverse(State { dist: d, node: u })) = pq.pop() {
        // Dacă am găsit deja o distanță mai mică pentru acest nod, sărim peste starea veche
        if d > dist[u] {
            continue;
        }

        let current_owner = fortress_owner[u];

        for edge in &graph[u] {
            let v = edge.to;
            let next_dist = d + edge.weight;

            // Cazul 1: Am găsit un drum strict mai scurt către nodul `v`
            if next_dist < dist[v] {
                dist[v] = next_dist;
                fortress_owner[v] = current_owner;
                pq.push(Reverse(State { dist: next_dist, node: v }));
            }
            // Cazul 2: Distanța este egală, decidem după ID-ul minim al fortăreței
            else if next_dist == dist[v] {
                if current_owner < fortress_owner[v] {
                    fortress_owner[v] = current_owner;
                    pq.push(Reverse(State { dist: next_dist, node: v }));
                }
            }
        }
    }

    // Pregătim vectorul de rezultate pentru afișare
    let mut ans = Vec::with_capacity(n);
    for i in 1..=n {
        // Fortărețele sau cătunele izolate (la care nu se ajunge) primesc valoarea 0
        if is_fortress[i] || fortress_owner[i] == 0 {
            ans.push("0".to_string());
        } else {
            ans.push(fortress_owner[i].to_string());
        }
    }

    // Scrim rezultatul final în fișier separat prin spații
    writeln!(fout, "{}", ans.join(" "))?;

    Ok(())
}