use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let k: usize = scan.next();
    let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    let b: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    if k == 1 {
        println!("{}", a[0]);
        return Ok(());
    }
    let mut max_b = vec![0usize; b.len()];
    max_b[0] = b[0];
    for i in 1..b.len() {
        max_b[i] = max_b[i - 1].max(b[i]);
    }
    let mut dp = vec![0usize; a.len()];
    dp[0] = a[0];
    for i in 1..k.min(a.len()) {
        dp[i] = dp[i - 1] + a[i];
    }
    for i in 0..k.min(a.len()) {
        let repeats = k - i - 1;
        dp[i] += max_b[i] * repeats;
    }
    let result = &dp[0..k.min(a.len())].iter().max().unwrap();
    println!("{}", result);
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
