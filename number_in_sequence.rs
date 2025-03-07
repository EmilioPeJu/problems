use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let mut n: u64 = scan.next();
    let mut res: Vec<u64> = vec![];
    let mut candidate = 2u64;
    let mut last_candidate = 1u64;
    let limit: u64 = (n as f64).sqrt() as u64;
    while n > 1 && candidate <= limit {
        let mut got_candidate = false;
        while candidate % last_candidate == 0
            && n % candidate == 0
            && (n / candidate) % candidate == 0
        {
            res.push(candidate);
            n /= candidate;
            got_candidate = true;
            last_candidate = candidate;
        }

        if got_candidate {
            candidate *= 2;
        } else {
            candidate += 1;
        }
    }

    if n > 1 {
        res.push(n);
    }
    println!("{}", res.len());
    for i in res {
        print!("{} ", i);
    }
    println!();
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
