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

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }
}

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let ts: usize = scan.next();
    for _ in 0..ts {
        let n: usize = scan.next();
        let d: usize = scan.next();
        let h: usize = scan.next();
        let yi: Vec<usize> = scan.next_n(n);
        let mut res = ((d * h) as f64) / 2.0;
        for i in 0..(n - 1) {
            if yi[i + 1] >= yi[i] + h {
                res += ((d * h) as f64) / 2.0;
            } else {
                let hp = yi[i + 1] - yi[i];
                res += (2 * d * h * hp - d * hp * hp) as f64 / (2.0 * h as f64);
            }
        }
        println!("{}", res);
    }
    Ok(())
}
