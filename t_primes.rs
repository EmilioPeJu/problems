use std::collections::VecDeque;
use std::io;

fn is_prime(i: u64) -> bool {
    for j in 2..=((i as f64).sqrt() as u64) {
        if i % j == 0 {
            return false;
        }
    }
    true
}

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let x: u64 = scan.next();
    let xs = (x as f64).sqrt() as u64;
    let res = x != 1 && x == xs * xs && is_prime(xs);
    println!("{}", if res { "YES" } else { "NO" });
    Ok(())
}

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let ts: usize = scan.next();
    for _ in 0..ts {
        solve1(&mut scan)?;
    }
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
