use std::collections::{HashMap, VecDeque};
use std::io;

struct Scan {
    buffer: std::collections::VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: VecDeque::new(),
        }
    }

    fn next_line(&self) -> io::Result<String> {
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line)? {
            0 => Err(io::Error::new(io::ErrorKind::Other, "EOF")),
            _ => Ok(line),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> io::Result<T> {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                break match token.parse::<T>() {
                    Ok(x) => Ok(x),
                    _ => Err(io::Error::new(io::ErrorKind::Other, "parsing is fucked")),
                };
            }
            let line = self.next_line()?;
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }

    fn next_n<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>().unwrap()).collect()
    }
}

fn _main() -> io::Result<()> {
    let mut scan = Scan::new();
    let ts: u64 = scan.next().unwrap();
    for _ in 0..ts {
        let n: u64 = scan.next().unwrap();
        let k: u64 = scan.next().unwrap();
        let mut rem: HashMap<u64, u64> = HashMap::new();
        for _ in 0..n {
            let current: u64 = scan.next().unwrap();
            if current % k != 0 {
                *rem.entry(k - current % k).or_insert(0) += 1;
            }
        }
        let mut result = 0u64;
        for (key, val) in rem.iter() {
            result = result.max((key + 1) + k * (val - 1));
        }
        println!("{}", result);
    }
    Ok(())
}

fn main() -> io::Result<()> {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap()?;
    Ok(())
}
