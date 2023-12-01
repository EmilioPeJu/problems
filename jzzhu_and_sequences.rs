use std::collections::VecDeque;
use std::io;

static BIG: i64 = 1000000007;

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let x: i64 = scan.next();
    let y: i64 = scan.next();
    let mut n = scan.next::<usize>() - 1;
    n = n % 6;
    let possible: [i64; 6] = [x, y, y - x, -x, -y, x - y];
    let result = ((possible[n] % BIG) + BIG) % BIG;
    println!("{}", result);
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
