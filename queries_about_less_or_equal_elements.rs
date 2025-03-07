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

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let m: usize = scan.next();
    let mut a: Vec<isize> = (0..n).map(|_| scan.next()).collect();
    a.sort();
    for _ in 0..m {
        let x: isize = scan.next();
        let mut l = 0;
        let mut r = a.len() - 1;
        while l <= r {
            let m = (l + r) / 2;
            if a[m] <= x {
                l = m + 1;
            } else {
                if m == 0 {
                    break;
                } else {
                    r = m - 1;
                }
            }
        }
        print!("{} ", l);
    }
    println!();
    Ok(())
}
