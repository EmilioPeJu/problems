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

fn main() -> io::Result<()> {
    let mut scan = Scan::new();
    let mut arr = [0usize; 100001];
    let n: usize = scan.next();
    for i in 0..n {
        let x: usize = scan.next();
        arr[x] = i;
    }
    let m: usize = scan.next();
    let mut count1 = 0u64;
    let mut count2 = 0u64;
    for _ in 0..m {
        let x: usize = scan.next();
        count1 += (arr[x] + 1) as u64;
        count2 += (n - arr[x]) as u64;
    }
    println!("{} {}", count1, count2);
    Ok(())
}
