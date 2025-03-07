use std::collections::VecDeque;
use std::io;

fn range_overlaps(l1: usize, r1: usize, l2: usize, r2: usize) -> bool {
    return l1 <= r2 && r1 >= l2;
}

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let x: usize = scan.next();
    let m: usize = scan.next();
    let mut gl = x;
    let mut gr = x;
    for _ in 0..m {
        let l: usize = scan.next();
        let r: usize = scan.next();
        if range_overlaps(l, r, gl, gr) {
            gl = l.min(gl);
            gr = r.max(gr);
        }
    }
    println!("{}", (gr - gl + 1));
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
