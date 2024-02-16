use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let s1: Vec<char> = scan.next::<String>().chars().collect();
    let s2: Vec<char> = scan.next::<String>().chars().collect();
    let mut onezero = 0usize;
    let mut zeroone = 0usize;
    for i in 0..n {
        if s1[i] == '1' && s2[i] == '0' {
            onezero += 1;
        } else if s1[i] == '0' && s2[i] == '1' {
            zeroone += 1;
        }
    }
    // res = swaps + insertions/deletions
    //     = smallest + (greatest - smallest) = greatest
    println!("{}", onezero.max(zeroone));
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
