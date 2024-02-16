use std::collections::{HashSet, VecDeque};
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let arr: Vec<i64> = (0..n).map(|_| scan.next()).collect();
    let mut set: HashSet<i64> = HashSet::new();
    set.insert(0);
    let mut acc = 0i64;
    let mut sol = false;
    for i in (0..n).step_by(2) {
        acc += arr[i];
        if set.contains(&acc) {
            sol = true;
        }
        set.insert(acc);
        if i + 1 < n {
            acc -= arr[i + 1];
            if set.contains(&acc) {
                sol = true;
            }
            set.insert(acc);
        }
    }
    println!("{}", if sol { "YES" } else { "NO" });
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
