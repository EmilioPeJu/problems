use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let mut counts = [0usize; 10];
    for _ in 0..n {
        let x: usize = scan.next();
        counts[x % 10] += 1;
    }
    let mut unrolled: Vec<usize> = vec![];
    for i in 0..10 {
        for _ in 0..3.min(counts[i]) {
            unrolled.push(i)
        }
    }
    let mut res = false;
    'for1: for i in 0..unrolled.len() {
        for j in (i + 1)..unrolled.len() {
            for k in (j + 1)..unrolled.len() {
                if (unrolled[i] + unrolled[j] + unrolled[k]) % 10 == 3 {
                    res = true;
                    break 'for1;
                }
            }
        }
    }
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
