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

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let mut v1: Vec<bool> = scan
        .next::<String>()
        .chars()
        .take(n)
        .map(|x| if x == '1' { true } else { false })
        .collect();
    let v2: Vec<bool> = scan
        .next::<String>()
        .chars()
        .take(n)
        .map(|x| if x == '1' { true } else { false })
        .collect();
    let mut res = 0usize;
    for i in 0..n {
        if !v2[i] {
            continue;
        }
        if i > 0 && v1[i - 1] {
            res += 1;
            v1[i - 1] = false;
        } else if !v1[i] {
            res += 1;
        } else if i < (n - 1) && v1[i + 1] {
            res += 1;
            v1[i + 1] = false;
        }
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
