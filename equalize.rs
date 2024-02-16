use std::collections::VecDeque;
use std::io;

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let s1: Vec<char> = scan.next::<String>().chars().collect();
    let s2: Vec<char> = scan.next::<String>().chars().collect();
    let mut acc = 0usize;
    let mut i = 0usize;
    while i < n {
        if s1[i] != s2[i] {
            acc += 1;
            if i + 1 < n && s1[i] != s1[i + 1] && s1[i + 1] != s2[i + 1] {
                i += 1;
            }
        }
        i += 1;
    }
    println!("{}", acc);
    Ok(())
}

struct Scan {
    buffer: VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: VecDeque::new(),
        }
    }

    fn next_line(&self) -> io::Result<String> {
        let mut line = String::new();
        match io::stdin().read_line(&mut line)? {
            0 => Err(io::Error::new(io::ErrorKind::Other, "EOF")),
            _ => Ok(line),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                match token.parse() {
                    Ok(x) => {
                        return x;
                    }
                    _ => {
                        panic!("parse");
                    }
                }
            }
            let line = self.next_line().unwrap();
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }
}
