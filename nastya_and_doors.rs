use std::collections::VecDeque;
use std::io;

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let k: usize = scan.next();
    let a: Vec<u64> = (0..n).map(|_| scan.next()).collect();
    let mut acc = vec![0u64; n];
    for i in 1..(n - 1) {
        acc[i] = acc[i - 1]
            + (if a[i] > a[i - 1] && a[i] > a[i + 1] {
                1
            } else {
                0
            });
    }
    acc[n - 1] = acc[n - 2];
    let mut l = 0usize;
    let mut max_npeaks = 0u64;
    for i in 0..=(n - k) {
        let npeaks = acc[i + k - 2] - acc[i];
        if npeaks > max_npeaks {
            max_npeaks = npeaks;
            l = i;
        }
    }
    println!("{} {}", max_npeaks + 1, l + 1);
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
