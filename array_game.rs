use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let k: usize = scan.next();
    let a: Vec<u64> = (0..n).map(|_| scan.next()).collect();
    if k > 2 {
        println!("0");
    } else if k == 1 || k == 2 {
        let mut b = a.clone();
        b.sort();
        for i in (1..n).rev() {
            b[i] = b[i] - b[i - 1];
        }
        if k == 1 {
            println!("{}", b.iter().min().unwrap());
        } else {
            b.extend(a.iter().skip(1));
            b.sort();
            for i in (1..n).rev() {
                b[i] = b[i] - b[i - 1];
            }
            println!("{}", b.iter().min().unwrap());
        }
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
