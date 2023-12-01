use std::collections::VecDeque;
use std::io;

fn is_ordered(a: &Vec<usize>) -> bool {
    for i in 0..a.len() - 1 {
        if a[i] > a[i + 1] {
            return false;
        }
    }
    true
}

fn solve1(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let mut a: Vec<usize> = (0..n).map(|_| scan.next()).collect();
    let com = a.iter().min().unwrap();
    let b: Vec<(usize, usize)> = a
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| if x % com == 0 { Some((i, x)) } else { None })
        .collect();
    let inds = b.iter().map(|p| p.0).collect::<Vec<usize>>();
    let mut vals = b.iter().map(|p| p.1).collect::<Vec<usize>>();
    vals.sort();
    for (i, val) in inds.into_iter().zip(vals.into_iter()) {
        a[i] = val;
    }
    println!("{}", if is_ordered(&a) { "YES" } else { "NO" });
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
