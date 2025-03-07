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

fn count_ways(x: u32, y: u32, n: u32, m: u32) -> u32 {
    let mut res = 4;
    if x == 1 || x == n {
        res -= 1;
    }
    if y == 1 || y == m {
        res -= 1;
    }
    res
}

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: u32 = scan.next();
    let m: u32 = scan.next();
    let x1: u32 = scan.next();
    let y1: u32 = scan.next();
    let x2: u32 = scan.next();
    let y2: u32 = scan.next();
    let res = count_ways(x1, y1, n, m).min(count_ways(x2, y2, n, m));
    println!("{}", res);
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
