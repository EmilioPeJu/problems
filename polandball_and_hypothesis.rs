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

fn is_prime(val: u32) -> bool {
    for i in 2..=((val as f64).sqrt() as u32) {
        if val % i == 0 {
            return false;
        }
    }
    true
}

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let n: u32 = scan.next();
    for m in 1..=1000 {
        if !is_prime(n * m + 1) {
            println!("{}", m);
            break;
        }
    }
    Ok(())
}
