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
    let mut vecs: Vec<(isize, isize)> = vec![];
    for _ in 0..n {
        let p = (scan.next::<isize>(), scan.next::<isize>());
        vecs.push(p);
    }
    let mut res = 0usize;
    for (x, y) in &vecs {
        let mut conditions = [false, false, false, false];
        for (xi, yi) in &vecs {
            if xi == x {
                if yi < y {
                    conditions[0] = true;
                } else if yi > y {
                    conditions[1] = true;
                }
            } else if yi == y {
                if xi < x {
                    conditions[2] = true;
                } else if xi > x {
                    conditions[3] = true;
                }
            }
        }
        if conditions.iter().all(|x| *x) {
            res += 1;
        }
    }
    println!("{}", res);
    Ok(())
}
