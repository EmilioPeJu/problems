use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: i64 = scan.next();
    let mut segments: Vec<(i64, i64)> = vec![];
    for _ in 0..n {
        let l: i64 = scan.next();
        let r: i64 = scan.next();
        segments.push((l, r));
    }
    let mut minx = segments[0].0;
    let mut maxx = segments[0].0;
    let mut h = segments[0].0;
    for i in 1..segments.len() {
        if segments[i].0 >= segments[i - 1].0 {
            if maxx + h < segments[i].0 {
                let offset = segments[i].0 - (maxx + h);
                h += (offset + 1) / 2;
            }
            minx = segments[i].0;
            maxx = (maxx + h).min(segments[i].1);
        } else {
            if minx - h > segments[i].1 {
                let offset = (minx - h) - segments[i].1;
                h += (offset + 1) / 2;
            }
            minx = (minx - h).max(segments[i].0);
            maxx = segments[i].1;
        }
    }
    println!("{}", h);
    Ok(())
}

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let ts: i64 = scan.next();
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
