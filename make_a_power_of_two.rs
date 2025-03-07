use std::collections::VecDeque;
use std::io;

fn min_ops(s1: &Vec<char>, s2: &Vec<char>) -> usize {
    let mut res = 0usize;
    let mut i = 0usize;
    let mut j = 0usize;
    while i < s1.len() && j < s2.len() {
        if s1[i] != s2[j] {
            res += 1;
            j += 1;
        } else {
            i += 1;
            j += 1;
        }
    }
    if i < s1.len() {
        res += s1.len() - i;
    } else if j < s2.len() {
        res += s2.len() - j;
    }
    res
}

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let ns: Vec<char> = n.to_string().chars().collect();
    let mut power = 1u64;
    let mut res = std::usize::MAX;
    for _ in 0..=63 {
        let powers: Vec<char> = power.to_string().chars().collect();
        let ops = min_ops(&powers, &ns);
        res = res.min(ops);
        power = power << 1;
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
