use std::collections::VecDeque;
use std::io;

fn possible(target: usize, x1: usize, x2: usize) -> usize {
    let minx = x1.min(x2);
    let mut maxx = x1.max(x2);
    let target2 = target + minx;
    maxx -= minx;
    if maxx % 2 == 0 && target2 > 0 {
        return 1;
    } else {
        return 0;
    }
}

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let a: usize = scan.next();
    let b: usize = scan.next();
    let c: usize = scan.next();
    println!(
        "{} {} {}",
        possible(a, b, c),
        possible(b, a, c),
        possible(c, a, b)
    );
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
