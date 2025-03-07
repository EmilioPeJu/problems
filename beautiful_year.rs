use std::collections::VecDeque;
use std::io;

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

fn valid(x: usize) -> bool {
    let a = x % 10;
    let b = x / 10 % 10;
    let c = x / 100 % 10;
    let d = x / 1000 % 10;
    a != b && a != c && a != d && b != c && b != d && c != d
}

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let mut x: usize = scan.next();
    x += 1;
    while !valid(x) {
        x += 1;
    }
    println!("{}", x);
    Ok(())
}
