use std::io::BufRead;

pub struct Scanner<R> {
    pub reader: R,
    pub buffer: Vec<String>,
}

impl<R: BufRead> Scanner<R> {
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(word) = self.buffer.pop() {
                return word.parse().ok().unwrap();
            }
            let mut line = String::new();
            self.reader.read_line(&mut line).unwrap();
            self.buffer = line.split_whitespace().map(String::from).collect();
            self.buffer.reverse();
        }
    }
}