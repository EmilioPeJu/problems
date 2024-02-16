use std::collections::VecDeque;
use std::io;

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let mut a: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    a.sort();
    let min = a[0];
    let max = a[n - 1];
    let n_min = a.iter().filter(|x| **x == min).count();
    let n_max = a.iter().filter(|x| **x == max).count();
    if min == max {
        println!("{} {}", 0, n * (n - 1) / 2);
    } else {
        println!("{} {}", max - min, n_min * n_max);
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
