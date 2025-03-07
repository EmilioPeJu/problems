use std::collections::VecDeque;
use std::io;

fn can_do(a: u64, b: u64, n: u64, m: u64) -> bool {
    if a + b < n + m {
        return false;
    }
    if b == 0 || m == 0 {
        return true;
    }
    if n == 0 {
        if a > b {
            return b >= m;
        } else {
            return a >= m;
        }
    }
    if a > b {
        let x = m.min(b);
        return can_do(a, b - x, n, m - x);
    } else {
        let x = n.min(b);
        return can_do(a, b - x, n - x, m);
    }
}

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let a: u64 = scan.next();
    let b: u64 = scan.next();
    let n: u64 = scan.next();
    let m: u64 = scan.next();
    println!("{}", if can_do(a, b, n, m) { "Yes" } else { "No" });
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
