use std::collections::VecDeque;
use std::io;
fn is_sorted(a: &Vec<usize>) -> bool {
    for i in 0..(a.len() - 1) {
        if a[i] > a[i + 1] {
            return false;
        }
    }
    true
}

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let k: usize = scan.next();
    let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    if k > 1 {
        // we can always sort apply a bubble like swaps
        println!("YES");
    } else {
        println!("{}", if is_sorted(&a) { "YES" } else { "NO" });
    }
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
