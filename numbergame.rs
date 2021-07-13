use std::collections::VecDeque;
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

fn get_n_divisors(n: u64) -> (u64, u64) {
    let mut nodd = 0u64;
    let mut neven = 0u64;
    for i in 2..=((n as f64).sqrt() as u64) {
        for j in &[i, n / i] {
            if n % j == 0 {
                if j % 2 == 0 {
                    neven += 1;
                } else {
                    nodd += 1;
                }
            }
        }
    }
    (nodd, neven)
}

fn _main() -> io::Result<()> {
    let mut scan = Scan::new();
    let ts: usize = scan.next().unwrap();
    for _ in 0..ts {
        let n: u64 = scan.next().unwrap();
        let mut winner = 0usize;
        if n == 1 {
            winner = 1;
        } else if n % 2 == 1 || n == 2 {
            winner = 0;
        } else {
            let (nodd, neven) = get_n_divisors(n);
            winner = if nodd > 0 { 0 } else { 1 };
            if neven == 1 {
                winner = 1 - winner;
            }
        }
        // failling for case 18
        println!("{}", ["Ashishgup", "FastestFinger"][winner]);
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
