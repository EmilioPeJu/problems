use std::collections::VecDeque;
use std::io;

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let n: usize = scan.next();
    let m: usize = scan.next();
    let mut a = [[0u8; 101]; 101];
    let mut b = [[0u8; 101]; 101];
    let mut full_i = vec![true; n];
    let mut full_j = vec![true; m];
    let mut allzeros = true;
    for i in 0..n {
        for j in 0..m {
            let x: u8 = scan.next();
            a[i][j] = x;
            allzeros = allzeros && x == 0;
            full_i[i] = full_i[i] && x == 1;
            full_j[j] = full_j[j] && x == 1;
        }
    }
    let mut result = true;
    for i in 0..n {
        for j in 0..m {
            if a[i][j] == 1 && !full_i[i] && !full_j[j] {
                result = false;
                break;
            }
            if full_i[i] && full_j[j] {
                b[i][j] = 1;
            }
        }
    }
    if !allzeros && result {
        result = full_i.iter().any(|&x| x) && full_j.iter().any(|&x| x);
    }
    println!("{}", if result { "YES" } else { "NO" });
    if result {
        for i in 0..n {
            for j in 0..m {
                print!("{} ", b[i][j]);
            }
            println!();
        }
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
