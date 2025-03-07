use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let mut range: (usize, usize) = (1, 1_000_000_000);
    let mut points: Vec<usize> = vec![];
    for _ in 0..n {
        let a: usize = scan.next();
        let x: usize = scan.next();
        if a == 1 {
            range.0 = range.0.max(x);
        } else if a == 2 {
            range.1 = range.1.min(x);
        } else {
            points.push(x);
        }
    }
    if range.0 > range.1 {
        println!("0");
        return Ok(());
    }
    let mut res = range.1 + 1 - range.0;
    for x in points {
        if x <= range.1 && x >= range.0 {
            res -= 1;
        }
    }
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
