use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    let mut valids = vec![0usize; n];
    let mut sum = vec![0usize; n];
    for i in 0..n {
        valids[i] = if a[i] < i + 1 { 1 } else { 0 };
    }

    sum[0] = valids[0];
    for i in 1..n {
        sum[i] += sum[i - 1] + valids[i];
    }

    let mut res = 0u64;
    for i in 0..n {
        if valids[i] > 0 {
            if a[i] > 1 {
                res += sum[a[i] - 2] as u64;
            }
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
