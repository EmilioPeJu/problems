use std::collections::{HashSet, VecDeque};
use std::io;

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let k: usize = scan.next();
    let mut set: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    for _ in 0..n {
        let x: usize = scan.next();
        if !set.contains(&x) {
            set.insert(x);
            queue.push_front(x);
            if queue.len() > k {
                let y = queue.pop_back().unwrap();
                set.remove(&y);
            }
        }
    }

    println!("{}", queue.len());
    for i in queue {
        print!("{} ", i);
    }

    println!();
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
