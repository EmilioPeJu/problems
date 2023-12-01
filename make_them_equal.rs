use std::collections::VecDeque;
use std::io;

fn solve(scan: &mut Scan) -> io::Result<()> {
    let n: usize = scan.next();
    let c: char = scan.next();
    let a: Vec<char> = scan.next::<String>().chars().collect();
    if a.iter().all(|&x| x == c) {
        println!("0");
        return Ok(());
    }

    for i in (1..=n).rev() {
        let mut good = true;
        for j in (i..=n).step_by(i) {
            if a[j - 1] != c {
                good = false;
                break;
            }
        }
        if good {
            println!("1\n{}", i);
            return Ok(());
        }
    }

    println!("2\n{} {}", n - 1, n);
    Ok(())
}

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let ts: usize = scan.next();
    for _ in 0..ts {
        solve(&mut scan)?;
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
