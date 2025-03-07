use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let a: Vec<u64> = (0..n).map(|_| scan.next()).collect();
    let mut ord: Vec<usize> = (0..n).collect();
    ord.sort_by_key(|&x| a[x]);
    let mut acc = vec![0u64; n];
    acc[0] = a[ord[0]];
    for i in 1..n {
        acc[i] = acc[i - 1] + a[ord[i]];
    }
    let mut res = vec![0u64; n];
    res[n - 1] = (n - 1) as u64;
    for i in (0..n - 1).rev() {
        if acc[i] >= a[ord[i + 1]] {
            res[i] = res[i + 1];
        } else {
            res[i] = i as u64;
        }
    }
    let mut res2 = vec![0u64; n];
    for i in 0..n {
        res2[ord[i]] = res[i];
    }
    println!(
        "{}",
        res2.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
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
